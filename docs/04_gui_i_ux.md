# 04. Interfejs Użytkownika (GUI & UX) - OpenGothicLauncher

Launcher korzysta z natywnego interfejsu opartego na **GTK4**, co zapewnia wysoką wydajność i integrację z systemami Linux/Windows.

## 1. Architektura Widoku (MVVM)
GUI jest zaimplementowane z podziałem na widoki (`src/*.rs`) oraz logikę stanu (`src/view_models/`).
- **Synchronizacja**: Dzięki `SharedUiState` oraz pętli odświeżania `glib`, interfejs jest zawsze zgodny ze stanem faktycznym w `ogl-core`, nawet podczas operacji asynchronicznych (np. pobierania).

## 2. Główne Komponenty UI

### Sidebar (Pasek Boczny)
- Pozwala na szybkie przełączanie się między wspieranymi grami.
- Wyświetla ikony-placeholdery sugerujące wariant gry.

### Game Panel (Panel Szczegółów)
To dynamiczna maszyna stanów, która przybiera formy:
- **Stan Detekcji**: Prośba o skanowanie lub wybór folderu.
- **Stan Instalacji Silnika**: Opcja pobrania i pasek postępu.
- **Stan Gotowości**: Wielki przycisk "Graj" oraz opcje dodatkowe.

### Engine Manager
Okno do zarządzania wieloma wersjami OpenGothic. Pozwala na:
- Podgląd aktualnie pobranych wersji.
- Ręczne wymuszenie odświeżenia wersji na GitHubie.
- Wybór konkretnego silnika do startu gry.

## 3. Asynchroniczność w UI
Dzięki integracji **Tokio** z pętlą zdarzeń GTK, długotrwałe operacje (jak pobieranie 200MB silnika czy brute-force skanowanie dysku) nie powodują "mrożenia" okna aplikacji. Postęp jest raportowany w czasie rzeczywistym na paskach postępu.
