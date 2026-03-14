use std::path::{Path, PathBuf};
use ogl_core::domain::install::{GothicGame, GothicInstall};
use ogl_core::CoreError;
use ogl_core::ports::{DetectProgress, InstallDetector};
use async_trait::async_trait;
use tracing::debug;
use directories::BaseDirs;

#[derive(Clone, Default)]
pub struct StdInstallDetector;

impl StdInstallDetector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl InstallDetector for StdInstallDetector {
    async fn detect(&self, game: GothicGame, on_progress: DetectProgress) -> Result<Option<GothicInstall>, CoreError> {
        debug!("Detecting install (fast) for {:?}", game);
        tokio::task::spawn_blocking(move || {
            let on_progress = on_progress.clone();
            if let Some(path) = stage1_fast(game, &on_progress) {
                return Ok(Some(GothicInstall { game, root_path: path }));
            }
            if let Some(path) = stage2_heuristic(game, &on_progress) {
                return Ok(Some(GothicInstall { game, root_path: path }));
            }
            Ok(None)
        })
        .await
        .map_err(|e| CoreError::External(e.to_string()))?
    }

    async fn detect_brute_force(&self, game: GothicGame, on_progress: DetectProgress) -> Result<Option<GothicInstall>, CoreError> {
        debug!("Detecting install (brute force) for {:?}", game);
        tokio::task::spawn_blocking(move || {
            let on_progress = on_progress.clone();
            if let Some(path) = stage3_brute_force(game, &on_progress) {
                return Ok(Some(GothicInstall { game, root_path: path }));
            }
            Ok(None)
        })
        .await
        .map_err(|e| CoreError::External(e.to_string()))?
    }
}

fn home_dir_opt() -> Option<PathBuf> {
    BaseDirs::new().map(|d| d.home_dir().to_path_buf())
}

fn home_dir() -> PathBuf {
    home_dir_opt().unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Internal: validation helpers
// ---------------------------------------------------------------------------

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

fn has_addon_vdf(root: &Path) -> bool {
    let mut data_dir = root.to_path_buf();

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

fn find_in_roots(game: GothicGame, roots: &[PathBuf], on_progress: &DetectProgress) -> Option<PathBuf> {
    for root in roots {
        on_progress.as_ref()(root);
        for subfolder in game_steam_folders(game) {
            let candidate = root.join(subfolder);
            on_progress.as_ref()(&candidate);
            if is_valid_root(&candidate, game) {
                return Some(candidate);
            }
        }
    }
    None
}

fn game_steam_folders(game: GothicGame) -> &'static [&'static str] {
    match game {
        GothicGame::Gothic1 => &["Gothic"],
        GothicGame::Gothic2 | GothicGame::Gothic2NotR => &["Gothic II"],
        GothicGame::ChroniclesOfMyrtana => &[
            "The Chronicles Of Myrtana Archolos",
            "Gothic II",
        ],
        GothicGame::Gothic3 => &["Gothic 3"],
    }
}

// ---------------------------------------------------------------------------
// Stage 1 – Fast (Registry / Well-known paths)
// ---------------------------------------------------------------------------

fn stage1_fast(game: GothicGame, on_progress: &DetectProgress) -> Option<PathBuf> {
    let roots = platform_known_roots(game);
    find_in_roots(game, &roots, on_progress)
}

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

#[cfg(target_os = "windows")]
fn windows_roots(_game: GothicGame, roots: &mut Vec<PathBuf>) {
    use windows_registry::CURRENT_USER;
    use windows_registry::LOCAL_MACHINE;

    if let Ok(key) = CURRENT_USER.open(r"SOFTWARE\Valve\Steam") {
        if let Ok(path) = key.get_string("SteamPath") {
            let steam_root = PathBuf::from(path).join("steamapps").join("common");
            roots.push(steam_root.clone());
            roots.push(PathBuf::from(path).join("SteamApps").join("common"));
        }
    }

    let gog_ids: &[(&str, GothicGame)] = &[
        ("1207658730", GothicGame::Gothic1),
        ("1207658924", GothicGame::Gothic2NotR),
        ("1207658742", GothicGame::Gothic3),
    ];
    for (id, gog_game) in gog_ids {
        if *gog_game != _game && !matches!((*gog_game, _game), (GothicGame::Gothic2NotR, GothicGame::Gothic2)) {
            continue;
        }
        let key_path = format!(r"SOFTWARE\WOW6432Node\GOG.com\Games\{}", id);
        if let Ok(key) = LOCAL_MACHINE.open(&key_path) {
            if let Ok(path) = key.get_string("path") {
                roots.push(PathBuf::from(&path).parent().unwrap_or(Path::new("")).to_path_buf());
                roots.push(PathBuf::from(path));
            }
        }
    }

    roots.push(PathBuf::from(r"C:\Games"));
    roots.push(PathBuf::from(r"D:\Games"));
    roots.push(PathBuf::from(r"C:\GOG Games"));
    roots.push(PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common"));
    roots.push(PathBuf::from(r"C:\Program Files\Steam\steamapps\common"));
}

#[cfg(not(target_os = "windows"))]
fn _windows_roots_stub() {}

#[cfg(target_os = "linux")]
fn linux_roots(roots: &mut Vec<PathBuf>) {
    let home = home_dir();

    roots.push(home.join(".steam").join("steam").join("steamapps").join("common"));
    roots.push(home.join(".local").join("share").join("Steam").join("steamapps").join("common"));

    roots.push(home.join(".wine").join("drive_c").join("GOG Games"));
    roots.push(
        home.join(".wine")
            .join("drive_c")
            .join("Program Files (x86)")
            .join("Steam")
            .join("steamapps")
            .join("common"),
    );

    let heroic_games = home.join("Games").join("Heroic");
    if heroic_games.exists() {
        if let Ok(entries) = std::fs::read_dir(&heroic_games) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    roots.push(entry.path().join("pfx").join("drive_c").join("GOG Games"));
                    roots.push(entry.path().join("pfx").join("drive_c").join("Program Files (x86)").join("Steam").join("steamapps").join("common"));
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
                    roots.push(entry.path().join("drive_c").join("GOG Games"));
                    roots.push(entry.path().join("drive_c").join("Program Files (x86)").join("Steam").join("steamapps").join("common"));
                }
            }
        }
    }

    let lutris_games = home.join(".local").join("share").join("lutris").join("runners").join("wine");
    if lutris_games.exists() {
        roots.push(lutris_games);
    }
}

#[cfg(target_os = "macos")]
fn macos_roots(roots: &mut Vec<PathBuf>) {
    let home = home_dir();

    roots.push(
        home.join("Library")
            .join("Application Support")
            .join("Steam")
            .join("steamapps")
            .join("common"),
    );
}

// ---------------------------------------------------------------------------
// Stage 2 – Heuristic
// ---------------------------------------------------------------------------

fn stage2_heuristic(game: GothicGame, on_progress: &DetectProgress) -> Option<PathBuf> {
    let mut scan_roots = platform_known_roots(game);
    scan_roots.extend(heuristic_scan_roots());

    scan_roots.sort();
    scan_roots.dedup();

    for root in &scan_roots {
        on_progress.as_ref()(root);
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with('.') {
                    continue;
                }
                if is_valid_root(&path, game) {
                    return Some(path);
                }
                if let Ok(sub) = std::fs::read_dir(&path) {
                    for sub_entry in sub.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_dir() {
                            on_progress.as_ref()(&sub_path);
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
        for letter in b'A'..=b'Z' {
            let root = PathBuf::from(format!("{}:\\", letter as char));
            if root.exists() {
                roots.push(root.clone());
                roots.push(root.join("Games"));
            }
        }
        if let Some(home) = home_dir_opt() {
            roots.push(home.join("Games"));
            roots.push(home.join("Desktop"));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let home = home_dir();
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
// Stage 3 – Brute force
// ---------------------------------------------------------------------------

fn stage3_brute_force(game: GothicGame, on_progress: &DetectProgress) -> Option<PathBuf> {
    let scan_roots = brute_force_roots();
    let skip_dirs = brute_force_skip_dirs();

    for root in &scan_roots {
        if let Some(found) = walk_dir(root, game, &skip_dirs, on_progress, 0, 12) {
            return Some(found);
        }
    }
    None
}

fn walk_dir(
    dir: &Path,
    game: GothicGame,
    skip_dirs: &[&str],
    on_progress: &DetectProgress,
    depth: usize,
    max_depth: usize,
) -> Option<PathBuf> {
    if depth > max_depth {
        return None;
    }

    on_progress.as_ref()(dir);

    let dir_name = dir.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
    if skip_dirs.iter().any(|s| dir_name == *s) {
        return None;
    }

    if is_valid_root(dir, game) {
        return Some(dir.to_path_buf());
    }

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
    let home = home_dir();
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
        "windows",
        "$recycle.bin",
        "system volume information",
        "proc",
        "sys",
        "dev",
        "run",
        "system",
        "private",
        "cores",
        "node_modules",
        ".git",
        "target",
    ]
}
