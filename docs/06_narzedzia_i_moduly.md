# 06. Moduły i Narzędzia - OpenGothicLauncher

Pozostałe komponenty systemu, które realizują wyspecjalizowane zadania techniczne.

## 1. Moduł Sieciowy (`ogl-network`)
Adapter dla portów sieciowych.
- **API GitHub**: Odpowiada za odpytywanie serwerów GitHub o najnowsze wydania binarne.
- **Download Engine**: Optymalny download strumieniowy dużych archiwów archiwów z raportowaniem postępu.

## 2. Moduł Uruchamiania (`ogl-executor`)
Odpowiada za bezpieczny start procesu gry.
- **Izolacja**: Uruchamia OpenGothic jako proces podrzędny (child process).
- **Argumenty**: Buduje skomplikowane flagi `-g <dir>` oraz `-game:<ini/mod>` na podstawie wybranego profilu.
- **Monitoring**: Przechwytuje kody wyjścia i opcjonalnie logi silnika do celów debugowania.

## 3. Zarządzanie Modami (`ogl-mods`)
Podstawowa logika obsługująca pliki VDF.
- Skanuje katalogi gry w poszukiwaniu rozszerzeń.
- Pomaga w ustalaniu priorytetów ładowania modyfikacji.

## 4. CLI (`ogl-cli`)
Pełnoprawny, bezokienkowy interfejs aplikacji. Pozwala na:
- Instalację silnika z terminala.
- Wykonywanie detekcji w skryptach.
- Zarządzanie profilami bez uruchamiania ciężkiego GUI.
