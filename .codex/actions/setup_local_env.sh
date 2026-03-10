#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# shellcheck disable=SC1091
source "$REPO_ROOT/.codex/actions/_artifact_env.sh"

cd "$REPO_ROOT"

pnpm install --frozen-lockfile
cargo fetch --manifest-path src-tauri/Cargo.toml
