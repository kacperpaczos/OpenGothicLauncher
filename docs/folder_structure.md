# Pełna Struktura Katalogów - OpenGothicLauncher

Poniżej znajduje się szczegółowe drzewo plików projektu wraz z opisem kluczowych komponentów.

## Widok Plików

```text
.
├── crates/
│   ├── ogl-cli/                # Interfejs wiersza poleceń
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   ├── ogl-core/               # Serce aplikacji (logika biznesowa)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app_dirs.rs         # Zarządzanie ścieżkami systemowymi
│   │       ├── config_manager.rs   # Konfiguracja i profile gier
│   │       ├── engine_manager.rs   # Instalacja silników OpenGothic
│   │       ├── install_detector.rs # Wykrywanie instalacji Gothica
│   │       └── sandbox_manager.rs  # Izolacja plików gry i modów
│   ├── ogl-executor/           # Uruchamianie procesów gry
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── ogl-gui/                # Graficzny interfejs (GTK4)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── window.rs           # Główne okno
│   │       ├── game_panel.rs       # Widok szczegółów gry
│   │       ├── sidebar.rs          # Lista gier/profili
│   │       ├── app_state.rs        # Globalny stan UI
│   │       ├── engine_window.rs    # Menadżer wersji silnika
│   │       └── runtime.rs          # Integracja z pętlą zdarzeń
│   ├── ogl-mods/               # Zarządzanie modami (VDF/MOD)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── ogl-network/            # Komunikacja z siecią
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── releases.rs        # Pobieranie danych z GitHuba
│           └── downloads.rs       # Logika pobierania plików
├── dev/                        # Skrypty deweloperskie
│   ├── build.sh
│   └── run_gui.sh
├── docs/                       # Dokumentacja techniczna
│   ├── architecture_map.md
│   ├── folder_structure.md
│   ├── game_detection_strategy.md
│   ├── ogl-cli.md
│   ├── ogl-core.md
│   ├── ogl-executor.md
│   ├── ogl-gui.md
│   ├── ogl-mods.md
│   └── ogl-network.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml                  # Konfiguracja Workspace
├── LICENSE
└── README.md
```

## Opis Funkcjonalny

- **ogl-core**: Biblioteka "head-less". Nie zawiera żadnego kodu związanego z wyświetlaniem okien. Dzięki temu logika wykrywania i zarządzania grą może być używana zamiennie przez CLI i GUI.
- **ogl-gui**: Zaimplementowane w Rust przy użyciu bindingów do biblioteki **GTK4**. Pliki w `src/` definiują poszczególne widoki i logikę odświeżania interfejsu.
- **ogl-executor**: Odpowiada za bezpieczne wywołanie procesu OpenGothic. Przekazuje parametry linii komend i monitoruje, czy gra nie zakończyła się błędem.
- **ogl-network**: Abstrakcja nad HTTP, która dba o to, by pobierane pakiety binarne silnika miały poprawne sumy kontrolne SHA256.
