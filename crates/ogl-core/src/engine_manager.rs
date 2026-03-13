use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::app_dirs::AppDirs;
use ogl_network::{GitHubRelease, GitHubAsset};
use crate::config_manager::ConfigManager;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Failed accessing engine directory: {0}")]
    IoError(#[from] std::io::Error),
    #[error("App directories error: {0}")]
    DirsError(#[from] crate::app_dirs::AppDirsError),
    #[error("Release error: {0}")]
    ReleaseError(#[from] ogl_network::ReleaseError),
    #[error("Download error: {0}")]
    DownloadError(#[from] ogl_network::DownloadError),
    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Config error: {0}")]
    ConfigError(#[from] crate::config_manager::ConfigError),
    #[error("No compatible engine asset found for this platform")]
    NoCompatibleAsset,
    #[error("Unsupported platform")]
    UnsupportedPlatform,
    #[error("Engine executable not found after extraction")]
    ExecutableNotFound,
}

#[derive(Debug, Clone)]
pub struct EngineVersion {
    pub version: String,
    pub executable_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum EnginePlatform {
    Linux,
    Windows,
    MacOS,
}

impl EnginePlatform {
    pub fn current() -> Option<Self> {
        if cfg!(target_os = "linux") {
            Some(Self::Linux)
        } else if cfg!(target_os = "windows") {
            Some(Self::Windows)
        } else if cfg!(target_os = "macos") {
            Some(Self::MacOS)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineDownload {
    pub version: String,
    pub archive_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct EngineInstall {
    pub version: String,
    pub install_dir: PathBuf,
    pub executable_path: PathBuf,
}

pub struct EngineManager {
    engines_dir: PathBuf,
}

impl EngineManager {
    pub fn new() -> Result<Self, EngineError> {
        let app_dirs = AppDirs::new()?;
        let engines_dir = app_dirs.engines_dir();
        Self::with_dir(engines_dir)
    }

    pub fn with_dir(engines_dir: PathBuf) -> Result<Self, EngineError> {
        if !engines_dir.exists() {
            std::fs::create_dir_all(&engines_dir)?;
        }
        Ok(Self { engines_dir })
    }

    pub fn engines_dir(&self) -> &PathBuf {
        &self.engines_dir
    }

    pub async fn fetch_latest_release(&self) -> Result<GitHubRelease, EngineError> {
        Ok(ogl_network::fetch_latest_release_from_html(None).await?)
    }

    pub async fn download_latest(
        &self,
        platform: EnginePlatform,
        progress: Option<Box<dyn FnMut(u64, u64) + Send>>,
    ) -> Result<EngineDownload, EngineError> {
        let release = self.fetch_latest_release().await?;
        self.download_release(&release, platform, progress).await
    }

    pub async fn download_release(
        &self,
        release: &GitHubRelease,
        platform: EnginePlatform,
        progress: Option<Box<dyn FnMut(u64, u64) + Send>>,
    ) -> Result<EngineDownload, EngineError> {
        let asset = self.find_asset(release, platform).ok_or(EngineError::NoCompatibleAsset)?;
        let version_dir = self.engines_dir.join(&release.tag_name);
        std::fs::create_dir_all(&version_dir)?;
        let dest_path = version_dir.join(&asset.name);

        ogl_network::download_file(&asset.browser_download_url, &dest_path, None, progress).await?;
        info!("Engine archive saved to {}", dest_path.display());

        Ok(EngineDownload {
            version: release.tag_name.clone(),
            archive_path: dest_path,
        })
    }

    pub async fn install_latest(
        &self,
        platform: EnginePlatform,
        progress: Option<Box<dyn FnMut(u64, u64) + Send>>,
    ) -> Result<EngineInstall, EngineError> {
        let release = self.fetch_latest_release().await?;
        self.install_release(&release, platform, progress).await
    }

    pub async fn install_release(
        &self,
        release: &GitHubRelease,
        platform: EnginePlatform,
        progress: Option<Box<dyn FnMut(u64, u64) + Send>>,
    ) -> Result<EngineInstall, EngineError> {
        let download = self.download_release(release, platform, progress).await?;
        let install_dir = self.engines_dir.join(&download.version);

        self.extract_archive(&download.archive_path, &install_dir)?;
        let executable_path = self
            .find_executable_in_dir(&install_dir, 6)
            .ok_or_else(|| {
                self.log_install_dir_preview(&install_dir);
                EngineError::ExecutableNotFound
            })?;

        self.set_active_engine(&download.version)?;

        info!("Engine installed to {}", install_dir.display());
        info!("Engine executable at {}", executable_path.display());

        let _ = std::fs::remove_file(&download.archive_path);

        Ok(EngineInstall {
            version: download.version,
            install_dir,
            executable_path,
        })
    }

    pub fn set_active_engine(&self, version: &str) -> Result<(), EngineError> {
        let mgr = ConfigManager::new()?;
        let mut cfg = mgr.load()?;
        cfg.active_engine = Some(version.to_string());
        mgr.save(&cfg)?;
        Ok(())
    }

    fn find_asset<'a>(&self, release: &'a GitHubRelease, platform: EnginePlatform) -> Option<&'a GitHubAsset> {
        release.assets.iter().find(|a| {
            let name = a.name.to_lowercase();
            match platform {
                EnginePlatform::Linux => name.contains("linux"),
                EnginePlatform::Windows => name.contains("win"),
                EnginePlatform::MacOS => name.contains("mac") || name.contains("osx"),
            }
        })
    }

    fn extract_archive(&self, archive_path: &Path, dest_dir: &Path) -> Result<(), EngineError> {
        let file = std::fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let Some(rel_path) = entry.enclosed_name() else {
                continue;
            };
            let out_path = dest_dir.join(rel_path);

            if entry.is_dir() {
                std::fs::create_dir_all(&out_path)?;
                continue;
            }

            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut out_file = std::fs::File::create(&out_path)?;
            std::io::copy(&mut entry, &mut out_file)?;

            #[cfg(unix)]
            if let Some(mode) = entry.unix_mode() {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(mode))?;
            }
        }

        Ok(())
    }

    fn find_executable_in_dir(&self, root: &Path, depth: usize) -> Option<PathBuf> {
        let candidates: &[&str] = if cfg!(windows) {
            &["Gothic2Notr.exe", "OpenGothic.exe"]
        } else if cfg!(target_os = "macos") {
            &["Gothic2Notr.sh", "Gothic2Notr", "OpenGothic"]
        } else {
            &["Gothic2Notr.sh", "Gothic2Notr", "OpenGothic"]
        };

        for name in candidates {
            let path = root.join(name);
            if path.is_file() {
                return Some(path);
            }
        }
        if depth == 0 {
            return None;
        }

        let entries = std::fs::read_dir(root).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = self.find_executable_in_dir(&path, depth - 1) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn log_install_dir_preview(&self, root: &Path) {
        warn!("Engine executable not found after extraction.");
        warn!("Install dir: {}", root.display());
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    warn!("Dir: {}", path.display());
                } else {
                    warn!("File: {}", path.display());
                }
            }
        }
    }

    pub fn list_installed(&self) -> Result<Vec<EngineVersion>, EngineError> {
        let mut versions = Vec::new();
        for entry in std::fs::read_dir(&self.engines_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(executable_path) = self.find_executable_in_dir(&entry.path(), 6) {
                    versions.push(EngineVersion { version: name, executable_path });
                }
            }
        }
        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::write::FileOptions;

    #[test]
    fn test_list_installed_engines() {
        let temp_dir = tempdir().unwrap();
        let manager = EngineManager::with_dir(temp_dir.path().to_path_buf()).unwrap();
        assert!(manager.list_installed().unwrap().is_empty());
        
        let engine_ver_dir = temp_dir.path().join("v1.0.4");
        fs::create_dir_all(&engine_ver_dir).unwrap();
        let exe_name = if cfg!(windows) { "Gothic2Notr.exe" } else { "Gothic2Notr" };
        
        // Exists dir but no exe
        assert!(manager.list_installed().unwrap().is_empty());
        
        // Exists dir and exe
        fs::write(engine_ver_dir.join(exe_name), "dummy exe file").unwrap();
        
        let installed = manager.list_installed().unwrap();
        assert_eq!(installed.len(), 1);
        assert_eq!(installed[0].version, "v1.0.4");
    }

    #[test]
    fn test_extract_and_find_executable() {
        let temp_dir = tempdir().unwrap();
        let manager = EngineManager::with_dir(temp_dir.path().to_path_buf()).unwrap();

        let archive_path = temp_dir.path().join("engine.zip");
        let file = std::fs::File::create(&archive_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default();

        zip.add_directory("bin/", options).unwrap();
        zip.start_file("bin/Gothic2Notr", options).unwrap();
        zip.write_all(b"fake exe").unwrap();
        zip.finish().unwrap();

        let install_dir = temp_dir.path().join("install");
        std::fs::create_dir_all(&install_dir).unwrap();
        manager.extract_archive(&archive_path, &install_dir).unwrap();

        let exe = manager.find_executable_in_dir(&install_dir, 6).unwrap();
        assert!(exe.ends_with("Gothic2Notr"));
    }

    #[test]
    fn test_set_active_engine_updates_config() {
        let temp_dir = tempdir().unwrap();
        let original = std::env::var("HOME").ok();
        std::env::set_var("HOME", temp_dir.path());

        let manager = EngineManager::new().unwrap();
        manager.set_active_engine("opengothic-v9.9.9").unwrap();

        let cfg = ConfigManager::new().unwrap().load().unwrap();
        assert_eq!(cfg.active_engine, Some("opengothic-v9.9.9".to_string()));

        if let Some(value) = original {
            std::env::set_var("HOME", value);
        }
    }
}
