use clap::{Parser, Subcommand};
use anyhow::Result;
use ogl_core::{ConfigManager, GameState};
use ogl_core::engine_manager::{EngineManager, EnginePlatform};
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
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let cfg_manager = ConfigManager::new()?;
    let engine_manager = EngineManager::new()?;

    match &cli.command {
        Commands::Detect { scan_disk } => {
            info!("Running detection for all Gothic variants...");
            let games = vec![
                ogl_core::GothicGame::Gothic1,
                ogl_core::GothicGame::Gothic2,
                ogl_core::GothicGame::Gothic2NotR,
                ogl_core::GothicGame::ChroniclesOfMyrtana,
                ogl_core::GothicGame::Gothic3,
            ];

            let mut found_any = false;
            let mut config = cfg_manager.load()?;

            for game in games.iter() {
                match ogl_core::detect(*game, |path| {
                    tracing::debug!("Scanning: {}", path.display());
                }) {
                    Ok(install) => {
                        info!("FOUND [{}]: {}", game.display_name(), install.root_path.display());
                        found_any = true;
                        
                        // Save per-game state
                        let key = format!("{:?}", game);
                        config.games.insert(key, GameState {
                            install_path: Some(install.root_path.clone()),
                            detected: true,
                        });
                    }
                    Err(_) => {
                        info!("Not found: {}", game.display_name());
                    }
                }
            }

            if !found_any {
                if *scan_disk {
                    info!("No installations found via fast scan. Starting full disk brute-force scan...");
                    for game in games.iter() {
                        let game_name = game.display_name();
                        info!("Brute-forcing {}...", game_name);
                        match ogl_core::detect_brute_force(*game, |_path| {}) {
                            Ok(install) => {
                                info!("FOUND [{}]: {}", game_name, install.root_path.display());
                                found_any = true;
                                let key = format!("{:?}", game);
                                config.games.insert(key, GameState {
                                    install_path: Some(install.root_path.clone()),
                                    detected: true,
                                });
                                info!("Note: Stopping brute-force scan early since an installation was found.");
                                break;
                            }
                            Err(_) => {
                                info!("Brute-force failed to find {}.", game_name);
                            }
                        }
                    }
                }
                
                if !found_any {
                    error!("No Gothic installations found. Try running with `--scan-disk` or configure manually.");
                }
            }

            // Save all detected results
            cfg_manager.save(&config)?;
        },
        Commands::Engines { action } => {
            match action {
                EngineCommands::List => {
                    let installed = engine_manager.list_installed()?;
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
                    let platform = EnginePlatform::current()
                        .ok_or_else(|| anyhow::anyhow!("Unsupported platform"))?;
                    info!("Installing latest OpenGothic engine...");
                    let install = engine_manager.install_latest(platform, Some(Box::new(|current, total| {
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
                    engine_manager.set_active_engine(version)?;
                    info!("Active engine set to {}", version);
                }
                EngineCommands::Active => {
                    let cfg = cfg_manager.load()?;
                    match cfg.active_engine {
                        Some(v) => info!("Active engine: {}", v),
                        None => info!("Active engine: (none)"),
                    }
                }
                EngineCommands::Dir => {
                    info!("Engines dir: {}", engine_manager.engines_dir().display());
                }
            }
        }
        Commands::Mods => {
            let config = cfg_manager.load()?;
            // Find any detected game path to scan mods from
            let gothic_path = config.games.values()
                .find(|g| g.detected)
                .and_then(|g| g.install_path.as_ref());

            if let Some(path) = gothic_path {
                let mod_manager = ogl_mods::ModManager::new(path);
                match mod_manager.scan_mods() {
                    Ok(mods) => {
                        info!("Detected mods:");
                        for m in mods {
                            info!("  - {} (VDF: {})", m.name, m.is_vdf);
                        }
                    }
                    Err(e) => error!("Failed to scan mods: {}", e),
                }
            } else {
                error!("No Gothic path configured. Run `detect` first.");
            }
        }
    }

    Ok(())
}

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
