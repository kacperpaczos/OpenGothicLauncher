# Interfejs Graficzny (GUI) - OpenGothicLauncher

Ten dokument opisuje strukturę i zachowanie okien aplikacji OpenGothicLauncher (moduł `ogl-gui`).

## 1. Główne Okno (Main Window)
Główne okno aplikacji (`window.rs`) podzielone jest na dwie sekcje za pomocą pionowego separatora:
- **Lewa część (Sidebar)**: Lista gier dostępna w launcherze.
- **Prawa część (Game Panel)**: Dynamicznie zmieniająca się treść zależna od wybranej gry i jej stanu.

## 2. Pasek Boczny (Sidebar)
Lokalizacja: `sidebar.rs`
- Wykorzystuje widget `GtkListBox`.
- Wyświetla listę wszystkich wspieranych gier:
    - **Gothic**
    - **Gothic II**
    - **Gothic II: NK**
    - **Archolos**
    - **Gothic 3**
- Każdy element zawiera ikonę (emoji jako placeholder) oraz skróconą nazwę gry.

## 3. Panel Gry (Game Panel)
Lokalizacja: `game_panel.rs`
To najbardziej dynamiczny element interfejsu. Jego zawartość zależy od wyników detekcji i stanu silnika.

### Stany Panelu:
1.  **Stan A: Gra nieodnaleziona**
    - Wyświetla ostrzeżenie "Installation not found".
    - Zawiera przycisk **"Scan for installation"** (startuje asynchroniczne wyszukiwanie).
    - Zawiera przycisk **"Select folder manually"** (otwiera dialog wyboru folderu).
2.  **Stan B: Gra znaleziona, brak silnika**
    - Wyświetla ścieżkę do gry.
    - Zawiera przycisk **"Download latest engine"** (pobiera binaria OpenGothic z GitHuba).
    - Podczas pobierania wyświetla pasek postępu (`ProgressBar`).
3.  **Stan C: Gotowość do startu**
    - Wyświetla aktywną wersję silnika.
    - Zawiera główny przycisk **"Launch OpenGothic"**.
    - Zawiera przycisk **"Manage engines"**.

## 4. Manager Silników (Engine Manager)
Lokalizacja: `engine_window.rs`
Osobne okno typu `Transient` (podrzędne wobec głównego), pozwalające na:
- Podgląd katalogu, w którym składowane są wersje silnika.
- Listowanie wszystkich dostępnych wersji (z oznaczeniem, które są już pobrane).
- Ustawianie aktywnej wersji silnika (jeśli zainstalowano więcej niż jedną).

## 5. Mechanizmy Odświeżania
UI nie blokuje się podczas operacji sieciowych czy dyskowych dzięki:
- Wykorzystaniu **Tokio Runtime** do zadań asynchronicznych.
- Funkcji `glib::timeout_add_local`, która co 500ms odświeża widoki, pobierając aktualny stan z modułu `ogl-core` (Shared UI State).

## 6. Maszyna Stanów Panelu Gry
Panel gry (`GamePanel`) działa jako wizualna maszyna stanów, reagująca na dane z `ogl-core`:

| Stan Typu | Warunek (Core) | Dostępne Akcje UI |
|-----------|----------------|-------------------|
| **Brak Gry** | `game_state.detected == false` | Scan, Browse (Ręczny wybór) |
| **Skanowanie**| `state.detection_running == true` | Spinner, Przycisk Anuluj |
| **Brak Silnika**| `detected == true` AND `installed_engines.is_empty()` | Download latest engine |
| **Pobieranie** | `state.download_progress.is_some()` | Pasek postępu |
| **Gotowość** | `detected == true` AND `engines.len() > 0` | **Launch**, Manage Engines |
