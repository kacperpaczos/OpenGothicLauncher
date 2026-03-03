use clap::{Parser, Subcommand};
use anyhow::Result;
use ogl_core::{config_manager::ConfigManager, engine_manager::EngineManager};
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
            for game in games.iter() {
                match ogl_core::detect(*game) {
                    Ok(install) => {
                        info!("FOUND [{}]: {}", game.display_name(), install.root_path.display());
                        found_any = true;
                        
                        // For MVP, if we haven't saved a path yet, save the first one we find
                        let mut config = cfg_manager.load()?;
                        if config.gothic_path.is_none() {
                            config.gothic_path = Some(install.root_path.clone());
                            cfg_manager.save(&config)?;
                            info!("Automatically configured default path to: {}", install.root_path.display());
                        }
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
                        match ogl_core::detect_brute_force(*game, |_path| {
                            // Printing every path is too noisy, but we can print some progress if we want.
                            // We will just do nothing to keep output clean, maybe print once every 1000 items.
                            // In GUI we will have a real progress bar.
                        }) {
                            Ok(install) => {
                                info!("FOUND [{}]: {}", game_name, install.root_path.display());
                                found_any = true;
                                let mut config = cfg_manager.load()?;
                                if config.gothic_path.is_none() {
                                    config.gothic_path = Some(install.root_path.clone());
                                    cfg_manager.save(&config)?;
                                }
                                info!("Note: Stopping brute-force scan early to save time since an installation was found.");
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
            if let Some(path) = config.gothic_path {
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
