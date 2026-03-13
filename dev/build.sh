#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

MODE="dev"
CARGO_ARGS=("build")

if [[ ${1-} == "--release" ]]; then
  MODE="release"
  CARGO_ARGS=("build" "--release")
fi

printf "==============================================\n"
printf "🛠  Building OpenGothicLauncher (%s)\n" "$MODE"
printf "==============================================\n"

cd "$REPO_ROOT"
cargo "${CARGO_ARGS[@]}"
