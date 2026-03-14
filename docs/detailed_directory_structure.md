# Eksplozywna Struktura Katalogów - OpenGothicLauncher

Ten dokument zawiera kompletny, drzewiasty widok wszystkich plików w projekcie wraz z ich rolami.

## 1. Pełne Drzewo Plików (Tree)

```text
.
├── Cargo.lock
├── Cargo.toml                  # Konfiguracja Workspace (wspólne zależności)
├── CONTRIBUTING.md             # Wytyczne dla autorów
├── LICENSE                     # Licencja GPL-3.0
├── README.md                   # Główny opis projektu
├── dev/                        # Narzędzia deweloperskie
│   ├── build.sh                # Skrypt budujący cały projekt
│   └── run_gui.sh              # Skrypt do szybkiego startu GUI
├── docs/                       # Dokumentacja techniczna
│   ├── architecture_map.md     # Ogólna mapa komponentów
│   ├── folder_structure.md     # Podstawowy opis folderów
│   ├── game_detection_strategy.md # Opis algorytmu wykrywania gier
│   ├── detailed_architecture.md # Głęboka analiza Clean Architecture
│   ├── detailed_directory_structure.md # Ten plik
│   ├── ogl-cli.md              # Dokumentacja modułu CLI
│   ├── ogl-core.md             # Dokumentacja modułu Core
│   ├── ogl-executor.md         # Dokumentacja modułu Executor
│   ├── ogl-gui.md              # Dokumentacja modułu GUI
│   ├── ogl-mods.md             # Dokumentacja modułu Mods
│   └── ogl-network.md          # Dokumentacja modułu Network
└── crates/                     # Kod źródłowy (Rust Crates)
    ├── ogl-cli/                # Interfejs Terminalowy
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs         # Parsowanie flag i wywołania serwisu
    ├── ogl-core/               # Serce Aplikacji (Clean Architecture)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs          # Agregacja modułów i re-eksporty
    │       ├── app_dirs.rs     # Ścieżki systemowe (~/.OpenGothicLauncher)
    │       ├── errors.rs       # Centralna definicja błędów
    │       ├── install_detector.rs # [Legacy] Wykrywanie gier
    │       ├── config_manager.rs   # [Legacy] Zarządzanie configami
    │       ├── engine_manager.rs   # [Legacy] Zarządzanie silnikami
    │       ├── sandbox_manager.rs  # Logika izolacji profilów
    │       ├── domain/         # Logika Biznesowa (Czyste Modele)
    │       │   ├── mod.rs
    │       │   ├── config.rs   # Struktury LauncherConfig, GameState
    │       │   ├── engine.rs   # Definicje EngineVersion, EngineAsset
    │       │   ├── install.rs  # Definicje GothicGame, GothicInstall
    │       │   ├── launch.rs   # Kontekst uruchomienia gry
    │       │   └── mods.rs     # Informacje o modyfikacjach
    │       ├── ports/          # Interfejsy (Abstrakcje Zewnętrzne)
    │       │   ├── mod.rs
    │       │   ├── config.rs   # Trait ConfigStore
    │       │   ├── executor.rs # Trait GameProcessRunner
    │       │   ├── filesystem.rs # Trait FileSystem (mockowalny!)
    │       │   ├── install.rs  # Trait InstallDetector
    │       │   ├── mods.rs     # Trait ModFilesProvider
    │       │   ├── network.rs  # Traity EngineDownloader, ReleaseProvider
    │       │   ├── paths.rs    # Trait AppPaths
    │       │   └── platform.rs # Trait PlatformProvider
    │       └── services/       # Orkiestracja Przepływów
    │           ├── mod.rs
    │           └── launcher_service.rs # Główny mózg aplikacji
    ├── ogl-executor/           # Adapter: Uruchamianie Procesów
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs          # Implementacja std::process::Command
    ├── ogl-gui/                # Adapter: Interfejs Graficzny (GTK4)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs         # Setup Tokio i GTK
    │       ├── app_state.rs    # Stan aplikacji widoczny przez UI
    │       ├── window.rs       # Główne okno aplikacji
    │       ├── sidebar.rs      # Pasek boczny z grami
    │       ├── game_panel.rs   # Główny panel sterowania grą
    │       ├── engine_window.rs # Zarządzanie wersjami silnika
    │       ├── runtime.rs      # Pomocnicy pętli zdarzeń
    │       └── view_models/    # Warstwa MVVM (Logika UI)
    │           ├── mod.rs
    │           ├── engine_manager_vm.rs
    │           └── game_panel_vm.rs
    ├── ogl-infra/              # Adapter: System Operacyjny i Pliki
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── archive.rs      # Rozpakowywanie silników (zip/tar)
    │       ├── config_store.rs # Zapisywanie stanu do JSON
    │       ├── filesystem.rs   # Implementacja IO na dysku
    │       ├── install_detector.rs # Specyficzna dla platformy detekcja
    │       ├── mod_files.rs    # Skanowanie plików VDF
    │       ├── paths.rs        # Konfiguracja bazowych katalogów
    │       └── platform.rs     # Rozpoznawanie Linux/Windows/macOS
    ├── ogl-mods/               # Narzędzia do Modyfikacji
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs          # Parsery i skanery VDF (minimalizm)
    └── ogl-network/            # Adapter: Komunikacja Sieciowa
        ├── Cargo.toml
        └── src/
            ├── lib.rs
            ├── downloads.rs    # Mechanika pobierania z paskiem postępu
            ├── releases.rs     # Pobieranie danych z GitHub API
            └── adapters.rs     # Implementacja portów Network z Core
```

## 2. Kluczowe Zależności Projektowe

- **Core -> Domain**: Pełna zależność. Domain to fundament.
- **Services -> Ports**: Serwisy znają tylko interfejsy.
- **Infra/Network/Executor -> Ports**: Implementują interfejsy (tzw. Dependency Inversion).
- **GUI/CLI -> Core**: Wywołują Serwisy i korzystają z Domain do wyświetlania danych.

## 3. Dlaczego tak dużo plików?

Struktura ta pozwala na:
1. **Mockowanie**: Możemy uruchomić testy `launcher_service` całkowicie "w pamięci", podstawiając `MockFileSystem` zamiast `std::fs`.
2. **Niezależność od GTK**: Jeśli kiedyś zdecydujesz się na przejście z GTK na np. Iced lub Slint, zmienisz tylko pliki w `ogl-gui`, a cała logika pobierania i uruchamiania gry pozostanie nietknięta.
3. **Czysty Domain**: Pliki w `src/domain/` nie mają żadnych zależności zewnętrznych, co czyni je najbezpieczniejszą i najstabilniejszą częścią kodu.
