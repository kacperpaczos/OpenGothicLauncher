# 01. Wizja i Funkcje - OpenGothicLauncher

OpenGothicLauncher to nowoczesny, wieloplatformowy launcher dla silnika OpenGothic, zaprojektowany z myślą o prostocie obsługi i bezpieczeństwie oryginalnych plików gry.

## Główna Wizja
Aplikacja ma za zadanie zautomatyzować proces instalacji i aktualizacji silnika OpenGothic oraz umożliwić wygodne zarządzanie profilami gier i modyfikacjami bez konieczności ręcznego edytowania plików konfiguracyjnych.

## Kluczowe Funkcje

### 1. Inteligentne Wykrywanie Gier (Game Detection)
Launcher automatycznie lokalizuje instalacje gier Gothic 1, Gothic 2 oraz Gothic 2: Noc Kruka.
- **Steam & GOG**: Integracja z rejestrami i bibliotekami cyfrowymi.
- **Brute Force**: Zaawansowane przeszukiwanie dysku dla niestandardowych lokalizacji.

### 2. Zarządzanie Silnikiem OpenGothic
- **Automatyczna Instalacja**: Pobieranie najnowszych binariów bezpośrednio z GitHub Release.
- **Wersjonowanie**: Możliwość posiadania wielu wersji silnika i łatwego przełączania się między nimi.
- **Weryfikacja Sum Kontrolnych**: Gwarancja spójności pobranych plików (SHA256).

### 3. Piaskownica i Profile (Sandboxing)
- **Ochrona Oryginału**: Silnik OpenGothic i jego pliki tymczasowe są odseparowane od katalogu głównego gry.
- **Profile Użytkownika**: Savy, ustawienia i mody są przechowywane w izolowanych folderach, co pozwala na bezproblemowe granie w kilka modyfikacji jednocześnie.

### 4. Wsparcie dla Modyfikacji
- Automatyczne skanowanie plików `.vdf` oraz `.mod`.
- Zarządzanie kolejnością ładowania i parametrami startowymi.

## Wspierane Platformy
- **Linux** (Natywnie, wsparcie dla XDG i ścieżek Protona).
- **Windows** (Integracja z Rejestrem Systemowym).
- **macOS** (Wsparcie dla natywnych struktur Apple).
