#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

pnpm release:check:versions
pnpm test:smoke
pnpm verify
pnpm test:coverage
pnpm test:rust
pnpm tauri build --debug

bash scripts/release/check-bundle-artifacts.sh src-tauri/target/debug/bundle ".app,.dmg"

if [[ "${OSTYPE:-}" == darwin* ]]; then
  dmg_path="$(find src-tauri/target/debug/bundle/dmg -maxdepth 1 -name '*.dmg' | head -n 1)"

  if [[ -z "$dmg_path" ]]; then
    echo "No DMG artifact found under src-tauri/target/debug/bundle/dmg"
    exit 1
  fi

  hdiutil imageinfo "$dmg_path"

  tmp_mount="$(mktemp -d)"
  hdiutil attach -readonly -nobrowse -mountpoint "$tmp_mount" "$dmg_path" >/dev/null
  find "$tmp_mount" -maxdepth 2 -print | sort
  hdiutil detach "$tmp_mount" >/dev/null
  rmdir "$tmp_mount"
else
  echo "Skipping DMG inspection because this is not macOS."
fi

echo "Local release smoke OK."
