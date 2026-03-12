use clap::{Parser, Subcommand};
use anyhow::Result;
use ogl_core::{ConfigManager, GameState};
use ogl_core::engine_manager::EngineManager;
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
    /// List installed OpenGothic engines
    Engines,
    /// List detected mods
    Mods,
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
        Commands::Engines => {
            let installed = engine_manager.list_installed()?;
            if installed.is_empty() {
                info!("No OpenGothic engines installed.");
            } else {
                info!("Installed OpenGothic engines:");
                for e in installed {
                    info!("  - Version: {}", e.version);
                }
            }
        },
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
}
