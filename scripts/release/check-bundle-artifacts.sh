#!/usr/bin/env bash
set -euo pipefail

bundle_dir="${1:-}"
expected_suffixes="${2:-}"

if [[ -z "$bundle_dir" || -z "$expected_suffixes" ]]; then
  echo "Usage: $0 <bundle_dir> <suffix1,suffix2,...>"
  exit 1
fi

if [[ ! -d "$bundle_dir" ]]; then
  echo "Bundle directory not found: $bundle_dir"
  exit 1
fi

IFS=',' read -r -a suffixes <<< "$expected_suffixes"
matches=()

while IFS= read -r file; do
  for suffix in "${suffixes[@]}"; do
    if [[ "$file" == *"$suffix" ]]; then
      matches+=("$file")
      break
    fi
  done
done < <(find "$bundle_dir" -type f | sort)

if [[ "${#matches[@]}" -eq 0 ]]; then
  echo "No bundle artifacts matched expected suffixes: $expected_suffixes"
  exit 1
fi

printf 'Bundle smoke OK:\n'
printf ' - %s\n' "${matches[@]}"
