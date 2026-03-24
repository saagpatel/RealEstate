#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PACKAGE_VERSION="$(node -p "require('./package.json').version")"
CARGO_VERSION="$(awk -F ' = ' '/^version = / { gsub(/"/, "", $2); print $2; exit }' src-tauri/Cargo.toml)"
TAURI_VERSION="$(node -p "require('./src-tauri/tauri.conf.json').version")"

if [[ "$PACKAGE_VERSION" != "$CARGO_VERSION" || "$PACKAGE_VERSION" != "$TAURI_VERSION" ]]; then
  echo "Version drift detected:"
  echo "  package.json: $PACKAGE_VERSION"
  echo "  src-tauri/Cargo.toml: $CARGO_VERSION"
  echo "  src-tauri/tauri.conf.json: $TAURI_VERSION"
  exit 1
fi

echo "Version sync OK: $PACKAGE_VERSION"
