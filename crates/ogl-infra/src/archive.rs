use std::path::Path;
use async_trait::async_trait;
use ogl_core::CoreError;
use ogl_core::ports::ArchiveExtractor;
use tracing::debug;

#[derive(Clone, Default)]
pub struct ZipArchiveExtractor;

impl ZipArchiveExtractor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ArchiveExtractor for ZipArchiveExtractor {
    async fn extract_zip(&self, archive_path: &Path, dest_dir: &Path) -> Result<(), CoreError> {
        let archive_path = archive_path.to_path_buf();
        let dest_dir = dest_dir.to_path_buf();
        debug!("Extracting archive {} to {}", archive_path.display(), dest_dir.display());
        tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(&archive_path).map_err(|e| CoreError::Io(e.to_string()))?;
            let mut archive = zip::ZipArchive::new(file).map_err(|e| CoreError::External(e.to_string()))?;

            for i in 0..archive.len() {
                let mut entry = archive.by_index(i).map_err(|e| CoreError::External(e.to_string()))?;
                let Some(rel_path) = entry.enclosed_name() else {
                    continue;
                };
                let out_path = dest_dir.join(rel_path);

                if entry.is_dir() {
                    std::fs::create_dir_all(&out_path).map_err(|e| CoreError::Io(e.to_string()))?;
                    continue;
                }

                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| CoreError::Io(e.to_string()))?;
                }

                let mut out_file = std::fs::File::create(&out_path).map_err(|e| CoreError::Io(e.to_string()))?;
                std::io::copy(&mut entry, &mut out_file).map_err(|e| CoreError::Io(e.to_string()))?;

                #[cfg(unix)]
                if let Some(mode) = entry.unix_mode() {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(mode))
                        .map_err(|e| CoreError::Io(e.to_string()))?;
                }
            }

            Ok(())
        })
        .await
        .map_err(|e| CoreError::External(e.to_string()))?
    }
}
