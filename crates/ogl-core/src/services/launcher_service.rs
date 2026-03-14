use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::domain::config::{LauncherConfig, GameState};
use crate::domain::engine::{EngineAsset, EngineInstall, EnginePlatform, EngineRelease, EngineVersion};
use crate::domain::install::{GothicGame, GothicInstall};
use crate::domain::launch::GameLaunch;
use crate::domain::mods::ModManager;
use crate::errors::{AppError, CoreError};
use crate::ports::{
    AppPaths, ArchiveExtractor, ConfigStore, DetectProgress, DownloadProgress, EngineDownloader, FileSystem,
    GameProcessRunner, InstallDetector, ModFilesProvider, PlatformProvider, ReleaseProvider,
};
use tracing::{debug, info};

#[derive(Clone)]
pub struct LauncherService {
    paths: Arc<dyn AppPaths>,
    fs: Arc<dyn FileSystem>,
    release_provider: Arc<dyn ReleaseProvider>,
    downloader: Arc<dyn EngineDownloader>,
    extractor: Arc<dyn ArchiveExtractor>,
    config_store: Arc<dyn ConfigStore>,
    install_detector: Arc<dyn InstallDetector>,
    mod_files: Arc<dyn ModFilesProvider>,
    platform: Arc<dyn PlatformProvider>,
    runner: Arc<dyn GameProcessRunner>,
}

impl LauncherService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        paths: Arc<dyn AppPaths>,
        fs: Arc<dyn FileSystem>,
        release_provider: Arc<dyn ReleaseProvider>,
        downloader: Arc<dyn EngineDownloader>,
        extractor: Arc<dyn ArchiveExtractor>,
        config_store: Arc<dyn ConfigStore>,
        install_detector: Arc<dyn InstallDetector>,
        mod_files: Arc<dyn ModFilesProvider>,
        platform: Arc<dyn PlatformProvider>,
        runner: Arc<dyn GameProcessRunner>,
    ) -> Self {
        Self {
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
        }
    }

    pub async fn load_config(&self) -> Result<LauncherConfig, AppError> {
        debug!("Loading launcher config");
        Ok(self.config_store.load().await?)
    }

    pub async fn save_config(&self, config: &LauncherConfig) -> Result<(), AppError> {
        debug!("Saving launcher config");
        Ok(self.config_store.save(config).await?)
    }

    pub fn engines_dir(&self) -> Result<PathBuf, AppError> {
        Ok(self.paths.engines_dir())
    }

    pub async fn list_installed_engines(&self) -> Result<Vec<EngineVersion>, AppError> {
        let platform = self.platform.current_platform()?;
        let engines_dir = self.paths.engines_dir();
        debug!("Listing installed engines in {}", engines_dir.display());
        if !self.fs.exists(&engines_dir).await {
            return Ok(Vec::new());
        }

        let mut versions = Vec::new();
        for entry in self.fs.read_dir(&engines_dir).await? {
            if !self.fs.is_dir(&entry).await {
                continue;
            }
            let version = entry.file_name().unwrap_or_default().to_string_lossy().to_string();
            if let Some(executable_path) = self.find_executable_in_dir(&entry, platform, 6).await {
                versions.push(EngineVersion { version, executable_path });
            }
        }
        Ok(versions)
    }

    pub async fn list_available_releases(&self) -> Result<Vec<EngineRelease>, AppError> {
        let releases = self.release_provider.list_releases().await?;
        Ok(releases)
    }

    pub async fn install_open_gothic(
        &self,
        version: &str,
        progress: Option<DownloadProgress>,
    ) -> Result<EngineInstall, AppError> {
        let platform = self.platform.current_platform()?;
        let release = self.release_provider.latest_release().await?;
        info!("Resolved latest OpenGothic release: {}", release.tag);

        if version != "latest" && version != release.tag {
            return Err(CoreError::InvalidState(format!(
                "Requested version '{}' is not available (latest is '{}')",
                version, release.tag
            )).into());
        }

        let asset = self
            .find_asset(&release, platform)
            .ok_or_else(|| CoreError::NotFound("No compatible engine asset".to_string()))?;
        debug!("Selected asset {} for {:?}", asset.name, platform);

        let version_dir = self.paths.engines_dir().join(&release.tag);
        self.fs.create_dir_all(&version_dir).await?;
        let archive_path = version_dir.join(&asset.name);

        self.downloader
            .download(&asset.download_url, &archive_path, progress)
            .await?;

        let install_dir = self.paths.engines_dir().join(&release.tag);
        self.extractor.extract_zip(&archive_path, &install_dir).await?;

        let executable_path = self
            .find_executable_in_dir(&install_dir, platform, 6).await
            .ok_or_else(|| CoreError::NotFound("Engine executable not found".to_string()))?;

        self.set_active_engine(&release.tag).await?;

        let _ = self.fs.remove_file(&archive_path).await;

        Ok(EngineInstall {
            version: release.tag,
            install_dir,
            executable_path,
        })
    }

    pub async fn set_active_engine(&self, version: &str) -> Result<(), AppError> {
        let mut cfg = self.config_store.load().await?;
        cfg.active_engine = Some(version.to_string());
        self.config_store.save(&cfg).await?;
        Ok(())
    }

    pub async fn scan_for_installations(
        &self,
        on_progress: DetectProgress,
    ) -> Result<Vec<GothicInstall>, AppError> {
        info!("Scanning for Gothic installations (fast)");
        let mut installs = Vec::new();
        for game in GothicGame::all_variants() {
            if let Some(found) = self.install_detector.detect(game, on_progress.clone()).await? {
                installs.push(found);
            }
        }
        Ok(installs)
    }

    pub async fn detect_installation(
        &self,
        game: GothicGame,
        on_progress: DetectProgress,
    ) -> Result<Option<GothicInstall>, AppError> {
        Ok(self.install_detector.detect(game, on_progress).await?)
    }

    pub async fn detect_installation_brute_force(
        &self,
        game: GothicGame,
        on_progress: DetectProgress,
    ) -> Result<Option<GothicInstall>, AppError> {
        Ok(self.install_detector.detect_brute_force(game, on_progress).await?)
    }

    pub async fn scan_for_installations_brute_force(
        &self,
        on_progress: DetectProgress,
    ) -> Result<Vec<GothicInstall>, AppError> {
        info!("Scanning for Gothic installations (brute force)");
        let mut installs = Vec::new();
        for game in GothicGame::all_variants() {
            if let Some(found) = self.install_detector.detect_brute_force(game, on_progress.clone()).await? {
                installs.push(found);
            }
        }
        Ok(installs)
    }

    pub async fn scan_mods(&self, game: GothicGame, gothic_root: &Path) -> Result<Vec<crate::domain::mods::ModInfo>, AppError> {
        let files = self.mod_files.list_mod_files(game, gothic_root).await?;
        ModManager::from_paths(files).map_err(|e| CoreError::InvalidState(e.to_string()).into())
    }

    pub async fn launch_profile(&self, profile_id: &str) -> Result<(), AppError> {
        info!("Launching profile {}", profile_id);
        let game: GothicGame = profile_id
            .parse()
            .map_err(|_| CoreError::InvalidState(format!("Unknown profile id '{}'", profile_id)))?;

        let cfg = self.config_store.load().await?;
        let key = game.profile_id();
        let state: GameState = cfg.games.get(&key).cloned().unwrap_or_default();
        let gothic_root = state.install_path.ok_or_else(|| {
            CoreError::NotFound(format!("Install path not set for {}", game.display_name()))
        })?;

        let engine = self.resolve_engine(&cfg).await?;
        let mods = self.mods_for_launch(game, &gothic_root).await?;

        let launch = GameLaunch {
            executable_path: engine.executable_path,
            gothic_root,
            mods,
        };
        self.runner.launch(&launch).await?;
        Ok(())
    }

    async fn resolve_engine(&self, cfg: &LauncherConfig) -> Result<EngineVersion, CoreError> {
        let installed = self.list_installed_engines().await.map_err(|e| match e {
            AppError::Core(core) => core,
        })?;

        if let Some(active) = cfg.active_engine.as_ref() {
            if let Some(found) = installed.iter().find(|e| &e.version == active) {
                return Ok(found.clone());
            }
        }

        installed
            .first()
            .cloned()
            .ok_or_else(|| CoreError::NotFound("No installed OpenGothic engines".to_string()))
    }

    fn find_asset(&self, release: &EngineRelease, platform: EnginePlatform) -> Option<EngineAsset> {
        release.assets.iter().find(|a| {
            let name = a.name.to_lowercase();
            match platform {
                EnginePlatform::Linux => name.contains("linux"),
                EnginePlatform::Windows => name.contains("win"),
                EnginePlatform::MacOS => name.contains("mac") || name.contains("osx"),
            }
        }).cloned()
    }

    async fn find_executable_in_dir(&self, root: &Path, platform: EnginePlatform, depth: usize) -> Option<PathBuf> {
        let candidates: &[&str] = match platform {
            EnginePlatform::Windows => &["Gothic2Notr.exe", "OpenGothic.exe"],
            EnginePlatform::MacOS => &["Gothic2Notr.sh", "Gothic2Notr", "OpenGothic"],
            EnginePlatform::Linux => &["Gothic2Notr.sh", "Gothic2Notr", "OpenGothic"],
        };

        let mut stack: Vec<(PathBuf, usize)> = vec![(root.to_path_buf(), 0)];
        while let Some((dir, current_depth)) = stack.pop() {
            for name in candidates {
                let path = dir.join(name);
                if self.fs.is_file(&path).await {
                    return Some(path);
                }
            }

            if current_depth >= depth {
                continue;
            }

            let entries = match self.fs.read_dir(&dir).await {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            for entry in entries {
                if self.fs.is_dir(&entry).await {
                    stack.push((entry, current_depth + 1));
                }
            }
        }
        None
    }

    async fn mods_for_launch(&self, game: GothicGame, gothic_root: &Path) -> Result<Vec<String>, CoreError> {
        if game != GothicGame::ChroniclesOfMyrtana {
            return Ok(Vec::new());
        }

        let system_dir = self.find_system_dir_ci(gothic_root).await?;
        let primary = system_dir.join("TheChroniclesOfMyrtana.ini");
        if self.fs.exists(&primary).await {
            return Ok(vec!["TheChroniclesOfMyrtana.ini".to_string()]);
        }

        let entries = self.fs.read_dir(&system_dir).await.unwrap_or_default();
        for entry in entries {
            if let Some(name) = entry.file_name().and_then(|n| n.to_str()) {
                let lower = name.to_lowercase();
                if lower.ends_with(".ini") && (lower.contains("myrtana") || lower.contains("chronicles")) {
                    return Ok(vec![name.to_string()]);
                }
            }
        }

        Err(CoreError::NotFound("Archolos INI not found in System/".to_string()))
    }

    async fn find_system_dir_ci(&self, root: &Path) -> Result<PathBuf, CoreError> {
        let direct = root.join("System");
        if self.fs.is_dir(&direct).await {
            return Ok(direct);
        }
        let entries = self.fs.read_dir(root).await?;
        for entry in entries {
            if let Some(name) = entry.file_name().and_then(|n| n.to_str()) {
                if name.to_lowercase() == "system" && self.fs.is_dir(&entry).await {
                    return Ok(entry);
                }
            }
        }
        Err(CoreError::NotFound("System directory not found".to_string()))
    }
}
