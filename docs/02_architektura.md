# 02. Architektura Systemu - OpenGothicLauncher

Projekt OpenGothicLauncher wykorzystuje nowoczesne wzorce projektowe: **Clean Architecture** oraz **Architekturę Hexagonalną (Ports & Adapters)**.

## 1. Filozofia Projektowa
Głównym celem jest całkowita separacja logiki biznesowej od technologii zewnętrznych (GUI, sieć, system plików). Dzięki temu "serce" aplikacji (`ogl-core`) jest łatwe do testowania i niezależne od użytych bibliotek czy systemu operacyjnego.

```mermaid
graph TD
    subgraph Adapters ["Warstwa Zewnętrzna (Adaptery)"]
        UI[ogl-gui / ogl-cli]
        Net[ogl-network / reqwest]
        FS[ogl-infra / std::fs]
        Exec[ogl-executor / std::process]
    end

    subgraph Core ["Serce Systemu (ogl-core)"]
        Services[Services - Koordynacja]
        Ports[Ports - Kontrakty/Traity]
        Domain[Domain - Modele Danych]
    end

    UI --> |Wywołuje| Services
    Services --> |Korzysta z| Domain
    Services --> |Rozmawia przez| Ports
    Net -.-> |Implementuje| Ports
    FS -.-> |Implementuje| Ports
    Exec -.-> |Implementuje| Ports
```

## 2. Warstwy w `ogl-core`

### Domena (`src/domain/`)
Czyste modele danych bez żadnych zależności zewnętrznych.
- `GothicInstall`: Reprezentuje znalezioną grę.
- `EngineVersion`: Reprezentuje konkretne wydanie silnika.
- `LauncherConfig`: Stan konfiguracji (profile, aktywny silnik).

### Porty (`src/ports/`)
Definiują "czego aplikacja potrzebuje od świata". To interfejsy (traits) Rusta.
- `InstallDetector`: Szukanie gier.
- `FileSystem`: Dostęp do dysku.
- `EngineDownloader`: Komunikacja sieciowa.

### Serwisy (`src/services/`)
Logika orkiestracji. Serwisy przyjmują Porty (DI) i realizują zadania.
- **LauncherService**: Główny punkt zarządzania. Odpowiada za instalację silnika, skanowanie modów i przygotowanie gry do startu.

## 3. Kluczowe Przepływy (Sequence)

Przykład pobierania i aktywacji silnika:
```mermaid
sequenceDiagram
    participant UI as ogl-gui
    participant LS as LauncherService
    participant PN as Port: Network
    participant FS as Port: FileSystem
    
    UI->>LS: install_open_gothic("latest")
    LS->>PN: fetch_latest_release()
    PN-->>LS: Metadata
    LS->>PN: download_zip(url)
    LS->>FS: extract(zip)
    LS->>LS: set_active_engine()
    LS-->>UI: OK (EngineInstall)
```

## 4. Zasada Zależności (DIP)
Zależności zawsze kierują się **do wewnątrz**. Adaptery (np. `ogl-network`) zależą od `ogl-core` (bo implementują jej traity), ale `ogl-core` nie wie nic o istnieniu `reqwest` czy konkretnego adaptera.
