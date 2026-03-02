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
pub fn detect(game: GothicGame) -> Result<GothicInstall, DetectorError> {
    // Stage 1 – fast
    if let Some(path) = stage1_fast(game) {
        return Ok(GothicInstall { game, root_path: path });
    }

    // Stage 2 – heuristic shallow scan
    if let Some(path) = stage2_heuristic(game) {
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

/// Check whether a candidate directory is a valid root for `game`.
fn is_valid_root(root: &Path, game: GothicGame) -> bool {
    match game {
        GothicGame::Gothic1 => {
            root.join("System").join("Gothic.exe").exists()
                || root.join("System").exists() && root.join("Data").exists()
        }
        GothicGame::Gothic2 => {
            let exe = root.join("System").join("Gothic2.exe");
            let addon = root.join("Data").join("Addon.vdf");
            exe.exists() && !addon.exists()
        }
        GothicGame::Gothic2NotR => {
            let exe = root.join("System").join("Gothic2.exe");
            let addon = root.join("Data").join("Addon.vdf");
            exe.exists() && addon.exists()
        }
        GothicGame::ChroniclesOfMyrtana => {
            // Archolos: has Gothic2.exe + GothicStarter.exe or GothicStarter.ini
            let exe = root.join("System").join("Gothic2.exe");
            let starter = root.join("System").join("GothicStarter.exe");
            let starter_ini = root.join("System").join("GothicStarter.ini");
            exe.exists() && (starter.exists() || starter_ini.exists())
        }
        GothicGame::Gothic3 => {
            root.join("Gothic3.exe").exists()
                || root.join("Gothic III.exe").exists()
        }
    }
}

/// Given a set of root prefixes, try appending each Steam subfolder and validate.
fn find_in_roots(game: GothicGame, roots: &[PathBuf]) -> Option<PathBuf> {
    for root in roots {
        for subfolder in game.steam_folders() {
            let candidate = root.join(subfolder);
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

fn stage1_fast(game: GothicGame) -> Option<PathBuf> {
    let roots = platform_known_roots(game);
    find_in_roots(game, &roots)
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

    // Lutris default wine prefix location
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

fn stage2_heuristic(game: GothicGame) -> Option<PathBuf> {
    let scan_roots = heuristic_scan_roots();
    for root in &scan_roots {
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = entry.file_name().to_string_lossy().to_lowercase();
                // Quick regex-like filter: folder must mention gothic / archolos / myrtana
                if !name.contains("gothic") && !name.contains("archolos") && !name.contains("myrtana") {
                    continue;
                }
                if is_valid_root(&path, game) {
                    return Some(path);
                }
                // One level deeper
                if let Ok(sub) = std::fs::read_dir(&path) {
                    for sub_entry in sub.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_dir() && is_valid_root(&sub_path, game) {
                            return Some(sub_path);
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
