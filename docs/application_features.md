# Funkcje Aplikacji - OpenGothicLauncher

Ten dokument opisuje główne funkcjonalności oferowane przez OpenGothicLauncher.

## 1. Inteligentne Wykrywanie Gier (Game Detection)
Launcher potrafi automatycznie zlokalizować instalacje gier z serii Gothic bez ingerencji użytkownika.
- **Etap Szybki**: Sprawdza rejestry Steam, GOG oraz standardowe ścieżki instalacji w systemach Windows i Linux.
- **Etap Heurystyczny**: Przeszukuje popularne foldery (np. `Games`, `Desktop`) do określonej głębokości.
- **Brute Force**: Opcjonalne, głębokie skanowanie dysków w poszukiwaniu rzadkich lokalizacji.
- **Weryfikacja**: Każdy znaleziony folder jest sprawdzany pod kątem obecności kluczowych plików (np. `Gothic2.exe`, `Addon.vdf`), aby uniknąć błędnych detekcji.

## 2. Zarządzanie Silnikiem OpenGothic
Aplikacja automatyzuje proces instalacji silnika OpenGothic, eliminując potrzebę ręcznego kopiowania plików.
- **Automatyczne Pobieranie**: Integracja z GitHub API pozwala na pobranie najnowszej wersji binariów dla konkretnego systemu (Linux/Windows).
- **Weryfikacja Spójności**: Wykorzystuje sumy kontrolne SHA256, aby upewnić się, że pobrane archiwum nie jest uszkodzone.
- **Wiele Wersji**: Możliwość przetrzymywania kilku wersji silnika jednocześnie i szybkiego przełączania się między nimi.

## 3. Izolacja i Profile (Sandboxing)
Jedną z kluczowych funkcji jest separacja plików OpenGothic od oryginalnej instalacji gry.
- **Ochrona plików gry**: Launcher nie nadpisuje plików w folderze `System/` oryginalnego Gothica.
- **Profile**: Pozwala na tworzenie osobnych "środowisk" dla różnych modyfikacji, dzięki czemu savy i konfiguracje modów nie mieszają się ze sobą.

## 4. Obsługa Modyfikacji (Mods)
- **Skanowanie VDF**: Automatyczne wykrywanie paczek `.vdf` i plików `.mod` w katalogu `Data/`.
- **Kolejność ładowania**: Mechanizm pozwalający na definiowanie, w jakiej kolejności pliki modyfikacji mają być przekazywane do silnika.

## 5. Wieloplatformowość
Dzięki użyciu Rusta i warstwy abstrakcji (Ports & Adapters), launcher oferuje identyczne doświadczenie na:
- **Linux** (Natywne wsparcie, obsługa ścieżek Proton/Steam).
- **Windows** (Integracja z Rejestrem Systemowym).
- **macOS** (Wsparcie dla natywnych lokalizacji plików).
