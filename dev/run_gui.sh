#!/bin/bash

# Skrypt ułatwiający uruchomienie GUI w trybie deweloperskim z pełnym logowaniem.
# Rozwiązuje problem z literówkami w konsoli, np. "cargo run -p ogl-gu"

# Przejście do korzenia projektu (jeden level wyżej od folderu dev/)
cd "$(dirname "$0")/.." || exit 1

LOG_FILE="dev/gui_log.txt"

echo "=============================================="
echo "🚀 Uruchamiam OpenGothicLauncher GUI (Tryb DEV)"
echo "📄 Pełne logi są zapisywane do: $LOG_FILE"
echo "=============================================="

# Zbuduj i uruchom z poziomem logowania DEBUG
# Strumień błędów i wyjścia standardowego jest łączony i zapisywany do pliku przez `tee`
RUST_LOG=debug cargo run -p ogl-gui 2>&1 | tee "$LOG_FILE"
