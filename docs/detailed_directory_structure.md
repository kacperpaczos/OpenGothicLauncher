# Szczegółowa Struktura Katalogów - OpenGothicLauncher

Ten dokument zawiera kompletną listę plików projektu wraz z opisem roli każdego z nich, podzieloną na logiczne warstwy.

## 1. Korzeń Projektu (Root)
- `Cargo.toml` - Główna konfiguracja Workspace Rusta (definicje wspólnych zależności).
- `Cargo.lock` - Zamrożone wersje zależności.
- `README.md` - Opis projektu dla użytkowników.
- `CONTRIBUTING.md` - Wytyczne dla deweloperów.
- `LICENSE` - Licencja GPL-3.0.
- `dev/` - Skrypty pomocnicze (`build.sh`, `run_gui.sh`).
- `docs/` - Dokumentacja techniczna projektu.

## 2. Biblioteka Główna (ogl-core)
Lokalizacja: `crates/ogl-core/src/`

Serce aplikacji, implementujące logikę niezależną od technologii wyświetlania.

### Warstwa Logiki (Legacy/Direct)
- `lib.rs` - Punkt wejścia i re-eksporty.
- `install_detector.rs` - Logika 3-etapowego wykrywania gier (Steam/GOG/bruteforce).
- `config_manager.rs` - Serializacja stanu do `state.json`.
- `engine_manager.rs` - Pobieranie i rozpakowywanie silników.
- `sandbox_manager.rs` - Przygotowanie izolowanych środowisk.
- `app_dirs.rs` - Standaryzacja ścieżek (`~/.OpenGothicLauncher`).
- `errors.rs` - Definicje błędów domenowych.

### Nowa Architektura (Hexagonal/Ports & Adapters)
- `domain/` - Modele danych i reguły biznesowe:
    - `config.rs`, `engine.rs`, `install.rs`, `launch.rs`, `mods.rs`.
- `services/` - Logika orkiestracji:
    - `launcher_service.rs` - Główny serwis koordynujący start gry.
- `ports/` - Interfejsy (abstrakcje) dla zewnętrznych komponentów:
    - `config.rs`, `executor.rs`, `filesystem.rs`, `install.rs`, `network.rs`, `paths.rs`, `platform.rs`.

## 3. Infrastruktura (ogl-infra)
Lokalizacja: `crates/ogl-infra/src/`

Realne implementacje portów z `ogl-core`.

- `install_detector.rs` - Specyficzne dla OS mechanizmy szukania (np. Registry na Windows).
- `config_store.rs` - Zapisywanie plików.
- `archive.rs` - Obsługa ZIP (rozpakowywanie silnika).
- `filesystem.rs` - Operacje na dysku.
- `platform.rs` - Detekcja systemu operacyjnego.

## 4. Narzędzia (Utilities)
- `ogl-network` - Wrapper na `reqwest`:
    - `releases.rs` - API GitHuba.
    - `downloads.rs` - Pasek postępu pobierania.
    - `adapters.rs` - Integracja z portami core.
- `ogl-executor` - Uruchamianie procesów systemowych.
- `ogl-mods` - Parsowanie plików `.vdf` i `.mod`.

## 5. Frontendy (Prezentacja)

### Interfejs Graficzny (ogl-gui)
Lokalizacja: `crates/ogl-gui/src/`
- `main.rs` - Inicjalizacja GTK4 i Tokio runtime.
- `window.rs` - Definicja głównego kontenera UI.
- `app_state.rs` - Zarządzanie stanem reaktywnym.
- `sidebar.rs` - Lista profili/gier po lewej stronie.
- `game_panel.rs` - Szczegóły wybranej gry (przycisk Graj, opcje).
- `engine_window.rs` - Okno wyboru/pobierania wersji silnika.
- `view_models/` - Logika odseparowana od widgetów (MVVM pattern):
    - `game_panel_vm.rs`, `engine_manager_vm.rs`.

### Interfejs Terminalowy (ogl-cli)
Lokalizacja: `crates/ogl-cli/src/`
- `main.rs` - Parsowanie argumentów za pomocą `clap` i wywoływanie `ogl-core`.
