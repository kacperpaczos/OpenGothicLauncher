# 03. Struktura Plików i Katalogów - OpenGothicLauncher

Projekt jest zorganizowany jako **Rust Workspace**, co pozwala na modularność i szybką kompilację komponentów.

## Pełne Drzewo Projektu

```text
.
├── Cargo.toml                  # Konfiguracja Workspace
├── dev/                        # Skrypty deweloperskie
├── docs/                       # Dokumentacja (ten folder)
└── crates/                     # Moduły (Crates)
    ├── ogl-core/               # LOGIKA GŁÓWNA
    │   └── src/
    │       ├── domain/         # Czyste modele i reguły
    │       ├── ports/          # Interfejsy (Traity)
    │       └── services/       # Serwisy (Logika orkiestracji)
    ├── ogl-infra/              # INFRASTRUKTURA (ADAPTERY)
    │   └── src/
    │       ├── install_detector.rs # Realizacja szukania gier
    │       ├── filesystem.rs       # Dostęp do dysku
    │       └── config_store.rs     # Zapisywanie stanu do JSON/TOML
    ├── ogl-gui/                # INTERFEJS GRAFICZNY (GTK4)
    │   └── src/
    │       ├── view_models/    # Warstwa MVVM (logika UI)
    │       └── ...             # Widgety okien
    ├── ogl-cli/                # INTERFEJS TERMINALOWY
    ├── ogl-network/            # ADAPTER SIECIOWY (Reqwest)
    ├── ogl-executor/           # ADAPTER URUCHAMIANIA PROCESÓW
    └── ogl-mods/               # LOGIKA MODYFIKACJI (Legacy/Utils)
```

## Rola Głównych Modułów

### `ogl-core`
Centralny punkt projektu. Definiuje jak aplikacja powinna działać, nie dbając o to, "jak" dokładnie dane są pobierane czy zapisywane. Wszystkie inne moduły krążą wokół niego.

### `ogl-infra`
Implementuje porty zdefiniowane w `core`. To tutaj znajduje się kod specyficzny dla systemów operacyjnych (np. dostęp do rejestru Windows).

### `ogl-gui`
Największy frontend aplikacji. Wykorzystuje bibliotekę **GTK4**. Jest odcięty od logiki domenowej poprzez warstwę `view_models`, co ułatwia testowanie interakcji UI.

### `ogl-network` & `ogl-executor`
Specjalistyczne adaptery do zadań "brudnych": pobierania binariów z sieci i fizycznego uruchamiania zewnętrznych procesów systemowych.
