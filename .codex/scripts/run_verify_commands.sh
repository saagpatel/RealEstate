#!/usr/bin/env bash
set -euo pipefail

COMMANDS_FILE="${1:-.codex/verify.commands}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if [[ ! -f "$COMMANDS_FILE" ]]; then
  echo "Missing $COMMANDS_FILE"
  exit 2
fi

cd "$REPO_ROOT"

if [[ -f ".codex/actions/_artifact_env.sh" ]]; then
  # Keep build caches stable across worktrees and repeated local runs.
  # shellcheck disable=SC1091
  source ".codex/actions/_artifact_env.sh"
fi

while IFS= read -r cmd || [[ -n "$cmd" ]]; do
  [[ -z "${cmd//[[:space:]]/}" ]] && continue
  [[ "$cmd" =~ ^[[:space:]]*# ]] && continue

  echo ">> $cmd"
  eval "$cmd"
done < "$COMMANDS_FILE"
