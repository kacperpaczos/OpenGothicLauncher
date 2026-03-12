# OpenGothicLauncher: Strategia Wykrywania Gier

Proces detekcji zainstalowanych gier z serii Gothic przebiega w oparciu o hierarchiczne, trzyetapowe sprawdzanie katalogów w systemie gracza oraz precyzyjne oznaczanie poprawności plików (w tym z pominięciem różnic wielkości liter, czyli case-insensitive). System ten został zoptymalizowany, by najpierw celować w "pewniaki" (oficjalne ścieżki Steam/GOG/Heroic), a dopiero następnie posiłkować się analizą heurystyczną i całkowitym przeszukiwaniem dysku.

## Etapy Wyszukiwania

### Stage 1: Fast (Błyskawiczny)
Skaner odpytuje system o z góry znane lokalizacje i klucze rejestru, aby uniknąć przeczesywania plików na dysku. Jest to najszybsza i najbardziej optymalna ścieżka. Składa się z tzw. **Prefiksów systemowych** (`platform_known_roots`), do których następnie dopisywana jest nazwa docelowego katalogu danej gry (podfoldery Steam, np. `Gothic II`).

### Stage 2: Heuristic (Płytkie przeszukiwanie - depth 2)
W przypadku, gdy gry nie ma w standardowej lokalizacji (np. z powodu instalacji przez pobranie ręczne lub niestandardowy launcher), system przeszukuje listę "korzeni heurystycznych" (`heuristic_scan_roots`) oraz listę znaną ze Stage 1, opuszczając skanowanie głęboko ukrytych katalogów systemowych (np. omijając foldery zaczynające się od `.`).
Skanuje te korzenie przeglądając je na poziom 1 oraz poziom 2 w głąb (np. `~/Games` -> `~/Games/gog` -> `~/Games/gog/gothic 2 gold`). Dzięki temu gry ukryte pod mylącymi lub zagęszczonymi nazwami (jak nowsze instalatory pod Heroic lub polskie tłumaczenia "Gothic 2 Gold") nadal są wykrywane błyskawicznie bez zamrażania UI.

### Stage 3: Brute-force (Głębokie skanowanie)
*Opcjonalny tryb wywoływany np. argumentem lini poleceń (flaga `--scan-disk`).*
Przeszukuje w pełni rekursywnie wskazane dyski systemowe we wszystkich dostępnych lokalizacjach za pomocą bardzo szybkiego silnika sprawdzającego `jwalk`.

---

## Wyłapywacze według Platform ("Katalogi Bazowe")

W zależności od systemu operacyjnego zdefiniowane zostały następujące "wyłapywacze" (prefiksowe punkty startowe). Skrypty Steam automatycznie dobudowywują na nich odpowiednie "Ogonek" (np. dla Gothica Noc Kruka dorzuca katalagi `/Gothic II` lub `/The Chronicles Of Myrtana Archolos` na sam koniec tych ścieżek).

### 🐧 Linux
System bazuje głównie na katalogu domowym usera (`~`).
* **Steam Native**:
  * `~/.steam/steam/steamapps/common`
  * `~/.local/share/Steam/steamapps/common`
* **GOG w standardowych Wine Prefixes**:
  * `~/.wine/drive_c/GOG Games`
  * `~/.wine/drive_c/Program Files (x86)/Steam/steamapps/common`
* **Lutris / Heroic / GOG Linux Native**:
  * `~/Games/Heroic` (rekursywne sprawdzanie w poszukiwaniu `pfx/drive_c/GOG Games` oraz `Steam`)
  * `~/Games/gog` (rekursywne poszukiwania podkatalogu Wine w `drive_c/GOG Games` oraz `Steam`)
  * `~/.local/share/lutris/runners/wine`
* **Dodatkowe punkty heurystyki (Stage 2)**:
  * `~/Games`, `/mnt`, `/media`, `/opt`

### 🪟 Windows
Bazujemy silnie na rejestrze systemu Windows (karnacje `HKCU` oraz `HKLM/WOW6432Node`).
* **Steam**:
  * Rejestr Windows: Zmienna `SteamPath` wpisana w z klucza `HKCU\SOFTWARE\Valve\Steam`. Skrót rozwija się potem dodając po drodze `steamapps\common` do wynikowej ściężki na dysku.
* **GOG Galaxy**:
  * Rejestr Windows: Ścieżki instalacji w kluczach rejestru `HKLM\SOFTWARE\WOW6432Node\GOG.com\Games\[ID_GRY]`. Gdzie "ID_GRY" to specyficzny numerek ze sklepu GOG.
* **Typowe hardkodowane ścieżki**:
  * `C:\Games`, `D:\Games`, `C:\GOG Games`, `C:\Program Files (x86)\Steam\steamapps\common`
* **Dodatkowe punkty heurystyki (Stage 2)**:
  * Przeszukiwanie głównych dysków od `A:\` do `Z:\` (oraz z dopiskiem `\Games`) i folderu Pulpitu.

### 🍏 macOS (Intel/M-series Mac)
Aktualnie podstawowe katalogi obejmują instalacje w przestrzeni użytkownika.
* **Steam**: 
  * `~/Library/Application Support/Steam/steamapps/common`
* **Oraz powszechne punkty heurystyczne (Stage 2)**:
  * `~/Games`, `/Volumes`, Pulpit.

---

## Sygnatury poprawnej instalacji (is_valid_root)

O tym czy znaleziona ścieżka rzeczywiście posiada Gothic, decyduje weryfikator `is_valid_root()`, który sprawdza sygnatury w sposób **uniezależniony od wielkości liter (case-insensitive)**. 

Każda gra ma swoje specyficzne wymagania plikowe:

| Gra | Sprawdzanie Plikowe | Powód |
| :--- | :--- | :--- |
| **Gothic 1** | `System\Gothic.exe` LUB katalogi `System` + `Data` | Standardowy EXEK oryginalnej jedynki. |
| **Gothic II (Vanilla)** | `System\Gothic2.exe` ORAZ BRAK pliku `*_Addon.vdf` | Musi być Gothic2.exe, ale nie może mieć plików VDF rozszerzenia w folderze Data (Vanilla nie ma Nocy Kruka). |
| **Gothic II: Noc Kruka** | `System\Gothic2.exe` ORAZ ZNALEZIONY plik `*_Addon.vdf` / `Addon.vdf` | Jeśli w `/Data` istnieje jakikolwiek addon VDF (np. `Speech_Addon.vdf` z GOG lub samo `Addon.vdf`), jest to pełnoprawna edycja (Gold) Noc Kruka. |
| **Kroniki Myrtany: Archolos** | `System\Gothic2.exe` ORAZ (`System\GothicStarter.exe` LUB `GothicStarter.ini` LUB `TheChroniclesOfMyrtana.ini` LUB `Data\KM_Scripts.mod`) | Wersje standardowe bazujące na modzie potrzebują GothicStarter, natomiast samodzielne wydatki sklepowe (jak w edycjach GOG/Steam) nie posiadają go, używając własnego pod-ini (`TheChroniclesOfMyrtana.ini`) i/bądź hermetycznych struktur z przedrostkami modeli twórców pod `KM_`. |
| **Gothic 3** | `Gothic3.exe` LUB `Gothic III.exe` | Trójka instaluje plik wykonywalny luzem na twardo w głównym koncie. |
