use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// All Gothic game variants that OpenGothicLauncher can detect and launch.
///
/// Each variant carries a semantic meaning beyond just "Gothic 2" so that the
/// launcher can handle per-variant data directories and OpenGothic arguments
/// correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GothicGame {
    /// Gothic 1 (any edition)
    Gothic1,
    /// Gothic 2 – classic / vanilla (no Addon)
    Gothic2,
    /// Gothic 2: Night of the Raven (Noc Kruka) – Add-On DLC
    Gothic2NotR,
    /// The Chronicles of Myrtana: Archolos – total conversion for G2
    ChroniclesOfMyrtana,
    /// Gothic 3
    Gothic3,
}

impl GothicGame {
    /// Human-readable name for display in UI / logging.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Gothic1 => "Gothic",
            Self::Gothic2 => "Gothic II",
            Self::Gothic2NotR => "Gothic II: Night of the Raven",
            Self::ChroniclesOfMyrtana => "The Chronicles of Myrtana: Archolos",
            Self::Gothic3 => "Gothic 3",
        }
    }

    /// Typical Steam / GOG subfolder name(s) for this game.
    fn steam_folders(&self) -> &'static [&'static str] {
        match self {
            Self::Gothic1 => &["Gothic"],
            Self::Gothic2 | Self::Gothic2NotR => &["Gothic II"],
            Self::ChroniclesOfMyrtana => &[
                "The Chronicles Of Myrtana Archolos",
                "Gothic II", // Archolos ships as a G2 mod
            ],
            Self::Gothic3 => &["Gothic 3"],
        }
    }
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum DetectorError {
    #[error("Gothic installation not found for '{0}'")]
    NotFound(&'static str),
    #[error("I/O error during detection: {0}")]
    IoError(#[from] std::io::Error),
}

// ---------------------------------------------------------------------------
// Detection result
// ---------------------------------------------------------------------------

/// A validated installation found on disk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GothicInstall {
    /// Which game variant this install represents.
    pub game: GothicGame,
    /// Root folder that contains `System/`, `Data/`, etc.
    pub root_path: PathBuf,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Detect the installation folder for the given game variant.
///
/// Detection runs through three stages:
///
/// 1. **Fast** – Platform-specific registries / well-known paths  (~0ms)
/// 2. **Heuristic** – Shallow scan of user-writable roots          (~2s)
/// 3. **None found** – returns `DetectorError::NotFound`
///    (Stage 3 brute-force is opt-in via `detect_brute_force()`)
///
/// # Example
/// ```no_run
/// use ogl_core::install_detector::{detect, GothicGame};
/// let install = detect(GothicGame::Gothic2NotR).expect("G2 NotR not found");
/// println!("{}", install.root_path.display());
/// ```
pub fn detect<F>(game: GothicGame, mut on_progress: F) -> Result<GothicInstall, DetectorError>
where
    F: FnMut(&Path),
{
    // Stage 1 – fast
    if let Some(path) = stage1_fast(game, &mut on_progress) {
        return Ok(GothicInstall { game, root_path: path });
    }

    // Stage 2 – heuristic shallow scan
    if let Some(path) = stage2_heuristic(game, &mut on_progress) {
        return Ok(GothicInstall { game, root_path: path });
    }

    Err(DetectorError::NotFound(game.display_name()))
}

/// Opt-in Stage 3 brute-force scan of the entire disk (can take minutes).
///
/// Designed to be called in an async context via `tokio::task::spawn_blocking`.
/// Progress can optionally be tracked via a callback:
/// `on_progress(current_path: &Path)`.
pub fn detect_brute_force<F>(game: GothicGame, on_progress: F) -> Result<GothicInstall, DetectorError>
where
    F: Fn(&Path),
{
    if let Some(path) = stage3_brute_force(game, on_progress) {
        return Ok(GothicInstall { game, root_path: path });
    }
    Err(DetectorError::NotFound(game.display_name()))
}

// ---------------------------------------------------------------------------
// Internal: validation helpers
// ---------------------------------------------------------------------------

/// Helper for case-insensitive checking of path segments (fixes issues on Linux Wine/Proton)
fn path_exists_ci(root: &Path, parts: &[&str]) -> bool {
    let mut current = root.to_path_buf();
    for part in parts {
        let expected = part.to_lowercase();
        let mut found = false;
        if let Ok(entries) = std::fs::read_dir(&current) {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().to_lowercase() == expected {
                    current = entry.path();
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return false;
        }
    }
    true
}

/// Check whether a candidate directory is a valid root for `game`.
fn is_valid_root(root: &Path, game: GothicGame) -> bool {
    match game {
        GothicGame::Gothic1 => {
            path_exists_ci(root, &["System", "Gothic.exe"])
                || (path_exists_ci(root, &["System"]) && path_exists_ci(root, &["Data"]))
        }
        GothicGame::Gothic2 => {
            path_exists_ci(root, &["System", "Gothic2.exe"]) && !has_addon_vdf(root)
        }
        GothicGame::Gothic2NotR => {
            path_exists_ci(root, &["System", "Gothic2.exe"]) && has_addon_vdf(root)
        }
        GothicGame::ChroniclesOfMyrtana => {
            let exe = path_exists_ci(root, &["System", "Gothic2.exe"]);
            let starter = path_exists_ci(root, &["System", "GothicStarter.exe"]);
            let starter_ini = path_exists_ci(root, &["System", "GothicStarter.ini"]);
            
            // Standalone installations like GOG or Steam don't use GothicStarter, but have specific mod files/configs
            let standalone_ini = path_exists_ci(root, &["System", "TheChroniclesOfMyrtana.ini"]);
            let km_scripts = path_exists_ci(root, &["Data", "KM_Scripts.mod"]);
            let km_scripts_pl = path_exists_ci(root, &["Data", "KM_ScriptsPL.mod"]);
            
            exe && (starter || starter_ini || standalone_ini || km_scripts || km_scripts_pl)
        }
        GothicGame::Gothic3 => {
            path_exists_ci(root, &["Gothic3.exe"]) || path_exists_ci(root, &["Gothic III.exe"])
        }
    }
}

/// Helper to check if any file ending with _Addon.vdf or exactly Addon.vdf exists in Data/
fn has_addon_vdf(root: &Path) -> bool {
    let mut data_dir = root.to_path_buf();
    
    // Find the actual case-sensitive Data directory
    let mut found_data = false;
    if let Ok(entries) = std::fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().to_lowercase() == "data" {
                data_dir = entry.path();
                found_data = true;
                break;
            }
        }
    }
    
    if !found_data {
        return false;
    }

    if let Ok(entries) = std::fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name == "addon.vdf" || name.ends_with("_addon.vdf") {
                return true;
            }
        }
    }
    false
}

/// Given a set of root prefixes, try appending each Steam subfolder and validate.
fn find_in_roots<F>(game: GothicGame, roots: &[PathBuf], on_progress: &mut F) -> Option<PathBuf>
where
    F: FnMut(&Path),
{
    for root in roots {
        on_progress(root);
        for subfolder in game.steam_folders() {
            let candidate = root.join(subfolder);
            on_progress(&candidate);
            if is_valid_root(&candidate, game) {
                return Some(candidate);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Stage 1 – Fast (Registry / Well-known paths)
// ---------------------------------------------------------------------------

fn stage1_fast<F>(game: GothicGame, on_progress: &mut F) -> Option<PathBuf>
where
    F: FnMut(&Path),
{
    let roots = platform_known_roots(game);
    find_in_roots(game, &roots, on_progress)
}

/// Build a list of platform-specific root prefixes for fast detection.
#[allow(unused_variables)]
fn platform_known_roots(game: GothicGame) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();

    #[cfg(target_os = "windows")]
    windows_roots(game, &mut roots);

    #[cfg(target_os = "linux")]
    linux_roots(&mut roots);

    #[cfg(target_os = "macos")]
    macos_roots(&mut roots);

    roots
}

// --- Windows ----------------------------------------------------------------

#[cfg(target_os = "windows")]
fn windows_roots(_game: GothicGame, roots: &mut Vec<PathBuf>) {
    use windows_registry::CURRENT_USER;
    use windows_registry::LOCAL_MACHINE;

    // Steam: HKCU\SOFTWARE\Valve\Steam -> SteamPath
    if let Ok(key) = CURRENT_USER.open(r"SOFTWARE\Valve\Steam") {
        if let Ok(path) = key.get_string("SteamPath") {
            let steam_root = PathBuf::from(path).join("steamapps").join("common");
            roots.push(steam_root.clone());
            // Some Steam installs use 'SteamApps' (capital)
            roots.push(PathBuf::from(path).join("SteamApps").join("common"));
        }
    }

    // GOG per-game registry keys
    let gog_ids: &[(&str, GothicGame)] = &[
        ("1207658730", GothicGame::Gothic1),
        ("1207658924", GothicGame::Gothic2NotR), // G2 Gold = NK
        ("1207658742", GothicGame::Gothic3),
    ];
    for (id, gog_game) in gog_ids {
        if *gog_game != game && !matches!((*gog_game, game), (GothicGame::Gothic2NotR, GothicGame::Gothic2)) {
            continue;
        }
        let key_path = format!(r"SOFTWARE\WOW6432Node\GOG.com\Games\{}", id);
        if let Ok(key) = LOCAL_MACHINE.open(&key_path) {
            if let Ok(path) = key.get_string("path") {
                // GOG key points directly to the game folder, not a common root
                roots.push(PathBuf::from(&path).parent().unwrap_or(Path::new("")).to_path_buf());
                // Also try the path itself as a root (e.g. C:\GOG Games\Gothic II)
                roots.push(PathBuf::from(path));
            }
        }
    }

    // Hardcoded common roots
    roots.push(PathBuf::from(r"C:\Games"));
    roots.push(PathBuf::from(r"D:\Games"));
    roots.push(PathBuf::from(r"C:\GOG Games"));
    roots.push(PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common"));
    roots.push(PathBuf::from(r"C:\Program Files\Steam\steamapps\common"));
}

// Stub for non-windows builds to keep it compiling
#[cfg(not(target_os = "windows"))]
fn _windows_roots_stub() {}

// --- Linux ----------------------------------------------------------------

#[cfg(target_os = "linux")]
fn linux_roots(roots: &mut Vec<PathBuf>) {
    let home = dirs::home_dir().unwrap_or_default();

    // Steam native
    roots.push(home.join(".steam").join("steam").join("steamapps").join("common"));
    roots.push(home.join(".local").join("share").join("Steam").join("steamapps").join("common"));

    // Wine / Lutris prefixes
    roots.push(home.join(".wine").join("drive_c").join("GOG Games"));
    roots.push(
        home.join(".wine")
            .join("drive_c")
            .join("Program Files (x86)")
            .join("Steam")
            .join("steamapps")
            .join("common"),
    );

    // Additional common Wine/Lutris/Heroic prefix locations for GOG and Steam on Linux
    let heroic_games = home.join("Games").join("Heroic");
    if heroic_games.exists() {
        if let Ok(entries) = std::fs::read_dir(&heroic_games) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    // E.g. ~/Games/Heroic/Prefixes/Gothic/pfx/drive_c/GOG Games
                    roots.push(entry.path().join("pfx").join("drive_c").join("GOG Games"));
                    roots.push(entry.path().join("pfx").join("drive_c").join("Program Files (x86)").join("Steam").join("steamapps").join("common"));
                    // E.g. ~/Games/Heroic/Gothic 2
                    roots.push(entry.path());
                }
            }
        }
    }

    let gog_games_linux = home.join("Games").join("gog");
    if gog_games_linux.exists() {
        if let Ok(entries) = std::fs::read_dir(&gog_games_linux) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    // Common prefix layout: ~/Games/gog/{game_slug}/drive_c/GOG Games/
                    roots.push(entry.path().join("drive_c").join("GOG Games"));
                    roots.push(entry.path().join("drive_c").join("Program Files (x86)").join("Steam").join("steamapps").join("common"));
                }
            }
        }
    }

    // Lutris default wine prefix location / Steam / GOG
    let lutris_games = home.join(".local").join("share").join("lutris").join("runners").join("wine");
    if lutris_games.exists() {
        roots.push(lutris_games);
    }
}

// --- macOS ----------------------------------------------------------------

#[cfg(target_os = "macos")]
fn macos_roots(roots: &mut Vec<PathBuf>) {
    let home = dirs::home_dir().unwrap_or_default();

    // Steam on macOS
    roots.push(
        home.join("Library")
            .join("Application Support")
            .join("Steam")
            .join("steamapps")
            .join("common"),
    );
}

// ---------------------------------------------------------------------------
// Stage 2 – Heuristic (shallow scan, depth <= 2)
// ---------------------------------------------------------------------------

fn stage2_heuristic<F>(game: GothicGame, on_progress: &mut F) -> Option<PathBuf>
where
    F: FnMut(&Path),
{
    let mut scan_roots = platform_known_roots(game);
    scan_roots.extend(heuristic_scan_roots());
    
    // Deduplicate to avoid scanning the same root twice
    scan_roots.sort();
    scan_roots.dedup();

    for root in &scan_roots {
        on_progress(root);
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                
                let name = entry.file_name().to_string_lossy().into_owned();
                
                // Skip hidden folders in heuristic roots (like ~/.cache) to speed up `/home` scanning
                // Wait, if root is explicitly `.steam` (from known_roots), then `root` is `.steam`.
                // Here `name` is the child of `root`. If `root` is `/home/user`, `name` is `.wine`.
                // We should skip hidden folders when scanning general heuristic roots.
                if name.starts_with('.') {
                    continue;
                }
                // We used to filter by name here (`name.contains("gothic")`), but that 
                // prevents finding games inside intermediate store folders like `~/Games/gog/gothic 2`.
                // So we just check the path and its immediate subdirectories.
                if is_valid_root(&path, game) {
                    return Some(path);
                }
                // One level deeper
                if let Ok(sub) = std::fs::read_dir(&path) {
                    for sub_entry in sub.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_dir() {
                            on_progress(&sub_path);
                            if is_valid_root(&sub_path, game) {
                                return Some(sub_path);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn heuristic_scan_roots() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // All drive roots A-Z
        for letter in b'A'..=b'Z' {
            let root = PathBuf::from(format!("{}:\\", letter as char));
            if root.exists() {
                roots.push(root.clone());
                roots.push(root.join("Games"));
            }
        }
        if let Some(home) = dirs::home_dir() {
            roots.push(home.join("Games"));
            roots.push(home.join("Desktop"));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let home = dirs::home_dir().unwrap_or_default();
        roots.push(home.clone());
        roots.push(home.join("Games"));
        roots.push(PathBuf::from("/mnt"));
        roots.push(PathBuf::from("/media"));
        roots.push(PathBuf::from("/opt"));
        #[cfg(target_os = "macos")]
        roots.push(PathBuf::from("/Volumes"));
    }

    roots
}

// ---------------------------------------------------------------------------
// Stage 3 – Brute force (full disk, opt-in, with progress callback)
// ---------------------------------------------------------------------------

fn stage3_brute_force<F>(game: GothicGame, on_progress: F) -> Option<PathBuf>
where
    F: Fn(&Path),
{
    let scan_roots = brute_force_roots();
    let skip_dirs = brute_force_skip_dirs();

    for root in &scan_roots {
        if let Some(found) = walk_dir(root, game, &skip_dirs, &on_progress, 0, 12) {
            return Some(found);
        }
    }
    None
}

fn walk_dir<F>(
    dir: &Path,
    game: GothicGame,
    skip_dirs: &[&str],
    on_progress: &F,
    depth: usize,
    max_depth: usize,
) -> Option<PathBuf>
where
    F: Fn(&Path),
{
    if depth > max_depth {
        return None;
    }

    on_progress(dir);

    let dir_name = dir.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
    if skip_dirs.iter().any(|s| dir_name == *s) {
        return None;
    }

    // Check current directory first (avoid unnecessary subtree descent)
    if is_valid_root(dir, game) {
        return Some(dir.to_path_buf());
    }

    // Skip symlinks to avoid loops
    if dir.is_symlink() {
        return None;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return None,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = walk_dir(&path, game, skip_dirs, on_progress, depth + 1, max_depth) {
                return Some(found);
            }
        }
    }
    None
}

fn brute_force_roots() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();

    #[cfg(target_os = "windows")]
    {
        for letter in b'A'..=b'Z' {
            let r = PathBuf::from(format!("{}:\\", letter as char));
            if r.exists() {
                roots.push(r);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let home = dirs::home_dir().unwrap_or_default();
        roots.push(home);
        roots.push(PathBuf::from("/mnt"));
        roots.push(PathBuf::from("/media"));
        roots.push(PathBuf::from("/opt"));
        #[cfg(target_os = "macos")]
        roots.push(PathBuf::from("/Volumes"));
    }

    roots
}

fn brute_force_skip_dirs() -> Vec<&'static str> {
    vec![
        // Windows
        "windows",
        "$recycle.bin",
        "system volume information",
        // Linux
        "proc",
        "sys",
        "dev",
        "run",
        // macOS
        "system",
        "private",
        "cores",
        // Generic
        "node_modules",
        ".git",
        "target",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::{tempdir, TempDir};

    fn setup_test_dir() -> TempDir {
        tempdir().unwrap()
    }

    #[test]
    fn test_is_valid_root_gothic2() {
        let temp_dir = setup_test_dir();
        let dir = temp_dir.path();
        let sys = dir.join("System");
        let data = dir.join("Data");
        fs::create_dir_all(&sys).unwrap();
        fs::create_dir_all(&data).unwrap();
        
        fs::write(sys.join("Gothic2.exe"), "dummy").unwrap();
        
        // Should be recognized as Gothic2, but NOT Gothic2NotR
        assert!(is_valid_root(&dir, GothicGame::Gothic2));
        assert!(!is_valid_root(&dir, GothicGame::Gothic2NotR));
        
        // Now add the NotR addon
        fs::write(data.join("Addon.vdf"), "dummy").unwrap();
        
        // Now it should be recognized as Gothic2NotR, but NOT Gothic2 vanilla
        assert!(is_valid_root(&dir, GothicGame::Gothic2NotR));
        assert!(!is_valid_root(&dir, GothicGame::Gothic2));
    }

    #[test]
    fn test_is_valid_root_archolos() {
        let temp_dir = setup_test_dir();
        let dir = temp_dir.path();
        let sys = dir.join("System");
        fs::create_dir_all(&sys).unwrap();
        
        fs::write(sys.join("Gothic2.exe"), "dummy").unwrap();
        fs::write(sys.join("GothicStarter.exe"), "dummy").unwrap();
        
        assert!(is_valid_root(&dir, GothicGame::ChroniclesOfMyrtana));
    }

    #[test]
    fn test_is_valid_root_gothic3() {
        let temp_dir = setup_test_dir();
        let dir = temp_dir.path();
        fs::write(dir.join("Gothic3.exe"), "dummy").unwrap();
        
        assert!(is_valid_root(&dir, GothicGame::Gothic3));
    }
}

