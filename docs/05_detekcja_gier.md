# 05. Strategia Detekcji Gier - OpenGothicLauncher

Skuteczne znalezienie plików Gothica to jedno z najtrudniejszych zadań launchera ze względu na różnorodność instalatorów (Steam, GOG, stare CD, Retail).

## 3-Etapowy Algorytm

### Etap 1: Fast Detect (~0 ms)
Przeszukujemy miejsca, gdzie szansa na znalezienie gry jest najwyższa:
- **Windows Registry**: Klucze GOG (`WOW6432Node\GOG.com\Games`) oraz ścieżki Steam (`Valve\Steam\SteamPath`).
- **Linux paths**: Standardowe lokalizacje Steam (`~/.steam/steamapps/common`) oraz prefixy Wine/Lutris/Heroic.

### Etap 2: Heuristic Scan (~2 s)
Jeśli rejestr zawiedzie, skanujemy najpopularniejsze foldery użytkownika:
- Pulpit, folder `Games`, korzenie dysków (np. `C:\Games`).
- Skanowanie ograniczone jest do głębokości 2, aby uniknąć zbędnego obciążenia IO.

### Etap 3: Brute Force (Opcjonalne)
Pełne przeszukiwanie wybranych dysków/katalogów. Użytkownik widzi pasek postępu z aktualnie skanowaną ścieżką. Skomplikowane reguły pomijają foldery systemowe (np. `/proc`, `/sys`, `Windows/`), aby przyspieszyć proces.

## Walidacja Instalacji
Samo znalezienie folderu o nazwie "Gothic" nie wystarcza. Launcher weryfikuje:
- Obecność pliku wykonywalnego (`Gothic2.exe`).
- Specyficzne pliki dla wariantu (np. `Addon.vdf` dla Nocy Kruka).
- Case-insensitivity (kluczowe na Linuxie, gdzie system plików odróżnia małe i duże litery).
