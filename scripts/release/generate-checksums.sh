#!/usr/bin/env bash
set -euo pipefail

artifacts_dir="${1:-artifacts}"

if [[ ! -d "$artifacts_dir" ]]; then
  echo "Artifacts directory not found: $artifacts_dir"
  exit 1
fi

find "$artifacts_dir" -type f \
  \( -name "*.AppImage" -o -name "*.app.tar.gz" -o -name "*.dmg" -o -name "*.exe" -o -name "*.msi" \) \
  | while IFS= read -r file; do
      shasum -a 256 "$file" > "${file}.sha256"
    done
