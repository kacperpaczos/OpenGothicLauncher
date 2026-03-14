use clap::{Parser, Subcommand};
use anyhow::Result;
use ogl_core::{GameState, LauncherService, GothicGame};
use ogl_infra::{
    TomlConfigStore, StdAppPaths, StdFileSystem, StdInstallDetector, StdModFilesProvider,
    StdPlatformProvider, ZipArchiveExtractor,
};
use ogl_network::{ReqwestDownloader, ReqwestReleaseProvider};
use ogl_executor::TokioGameRunner;
use std::sync::Arc;
use std::str::FromStr;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "ogl-cli")]
#[command(about = "OpenGothicLauncher CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect existing Gothic installations
    Detect {
        /// Optional: run a full disk brute-force scan if fast detection fails
        #[arg(long)]
        scan_disk: bool,
    },
    /// Manage OpenGothic engines
    Engines {
        #[command(subcommand)]
        action: EngineCommands,
    },
    /// List detected mods
    Mods,
}

#[derive(Subcommand)]
enum EngineCommands {
    /// List installed engines
    List,
    /// Install the latest engine for the current platform
    InstallLatest,
    /// Set active engine version
    SetActive {
        /// Version tag (e.g. opengothic-v1.0.3549)
        version: String,
    },
    /// Show active engine
    Active,
    /// Show engines directory
    Dir,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let service = build_service()?;

    match &cli.command {
        Commands::Detect { scan_disk } => {
            info!("Running detection for all Gothic variants...");

            let mut found_any = false;
            let mut config = service.load_config().await?;

            let installs = service.scan_for_installations(Arc::new(|path| {
                tracing::debug!("Scanning: {}", path.display());
            })).await?;

            for install in installs {
                info!("FOUND [{}]: {}", install.game.display_name(), install.root_path.display());
                found_any = true;
                let key = install.game.profile_id();
                config.games.insert(key, GameState {
                    install_path: Some(install.root_path.clone()),
                    detected: true,
                });
            }

            if !found_any {
                if *scan_disk {
                    info!("No installations found via fast scan. Starting full disk brute-force scan...");
                    let installs = service.scan_for_installations_brute_force(Arc::new(_path_noop)).await?;
                    for install in installs {
                        info!("FOUND [{}]: {}", install.game.display_name(), install.root_path.display());
                        found_any = true;
                        let key = install.game.profile_id();
                        config.games.insert(key, GameState {
                            install_path: Some(install.root_path.clone()),
                            detected: true,
                        });
                    }
                }
                
                if !found_any {
                    error!("No Gothic installations found. Try running with `--scan-disk` or configure manually.");
                }
            }

            service.save_config(&config).await?;
        },
        Commands::Engines { action } => {
            match action {
                EngineCommands::List => {
                    let installed = service.list_installed_engines().await?;
                    if installed.is_empty() {
                        info!("No OpenGothic engines installed.");
                    } else {
                        info!("Installed OpenGothic engines:");
                        for e in installed {
                            info!("  - Version: {} ({})", e.version, e.executable_path.display());
                        }
                    }
                }
                EngineCommands::InstallLatest => {
                    info!("Installing latest OpenGothic engine...");
                    let install = service.install_open_gothic("latest", Some(Box::new(|current, total| {
                        if total > 0 {
                            let pct = (current as f64 / total as f64) * 100.0;
                            info!("Download progress: {:.0}%", pct);
                        }
                    }))).await?;
                    info!("Installed: {}", install.version);
                    info!("Install dir: {}", install.install_dir.display());
                    info!("Executable: {}", install.executable_path.display());
                }
                EngineCommands::SetActive { version } => {
                    service.set_active_engine(version).await?;
                    info!("Active engine set to {}", version);
                }
                EngineCommands::Active => {
                    let cfg = service.load_config().await?;
                    match cfg.active_engine {
                        Some(v) => info!("Active engine: {}", v),
                        None => info!("Active engine: (none)"),
                    }
                }
                EngineCommands::Dir => {
                    info!("Engines dir: {}", service.engines_dir()?.display());
                }
            }
        }
        Commands::Mods => {
            let config = service.load_config().await?;
            // Find any detected game path to scan mods from
            let maybe_game_path = config.games.iter()
                .find(|(_, g)| g.detected)
                .and_then(|(k, g)| GothicGame::from_str(k).ok().map(|game| (game, g.install_path.as_ref())));

            if let Some((game, Some(path))) = maybe_game_path {
                let mods = service.scan_mods(game, path).await?;
                info!("Detected mods:");
                for m in mods {
                    info!("  - {} (VDF: {})", m.name, m.is_vdf);
                }
            } else {
                error!("No Gothic path configured. Run `detect` first.");
            }
        }
    }

    Ok(())
}

fn build_service() -> Result<LauncherService> {
    let paths = Arc::new(StdAppPaths::new()?);
    let fs = Arc::new(StdFileSystem::new());
    let release_provider = Arc::new(ReqwestReleaseProvider::new());
    let downloader = Arc::new(ReqwestDownloader::new());
    let extractor = Arc::new(ZipArchiveExtractor::new());
    let config_store = Arc::new(TomlConfigStore::new(paths.clone(), fs.clone()));
    let install_detector = Arc::new(StdInstallDetector::new());
    let mod_files = Arc::new(StdModFilesProvider::new());
    let platform = Arc::new(StdPlatformProvider::new());
    let runner = Arc::new(TokioGameRunner::new());

    Ok(LauncherService::new(
        paths,
        fs,
        release_provider,
        downloader,
        extractor,
        config_store,
        install_detector,
        mod_files,
        platform,
        runner,
    ))
}

fn _path_noop(_path: &std::path::Path) {}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_engines_subcommands() {
        let _ = Cli::parse_from(["ogl-cli", "engines", "list"]);
        let _ = Cli::parse_from(["ogl-cli", "engines", "install-latest"]);
        let _ = Cli::parse_from(["ogl-cli", "engines", "set-active", "opengothic-v1.0.1"]);
        let _ = Cli::parse_from(["ogl-cli", "engines", "active"]);
        let _ = Cli::parse_from(["ogl-cli", "engines", "dir"]);
    }
}
