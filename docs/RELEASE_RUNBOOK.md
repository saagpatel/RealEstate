# Release Runbook

## Goal

Use this runbook to prepare and ship a desktop release candidate without
guessing which commands or workflows matter.

## What must be true before release prep

These repo-side conditions should be true before you tag anything:

- `pnpm verify` passes locally
- GitHub quality workflows are green
- the app version is ready to be bumped
- release notes content exists for the new version
- performance baselines are intentionally current

## External inputs still required

Public release publication is still blocked until these external inputs exist:

- Apple signing certificate and notarization credentials
- Windows code-signing plan or certificate
- production Anthropic credentials
- LemonSqueezy production credentials and confirmed SKU/purchase flow
- GitHub release secrets and permissions

## Recommended release flow

### 1. Prepare the version PR

Use the **Version Bump** workflow to create a guarded release-prep PR.

That workflow updates:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `CHANGELOG.md`

The workflow now opens a PR from a `codex/chore/...` branch instead of writing
directly to the default branch.

### 2. Review the release-prep PR

Before merging, confirm:

- version numbers are in sync
- changelog text matches the intended release scope
- required repo checks pass
- no unexpected lockfile or artifact drift is included

### Changelog format expected by automation

Use this heading format so the release workflow can extract notes correctly:

```md
## [0.1.0] - 2026-03-10

### Added

- ...
```

### 3. Merge and tag

After the release-prep PR is approved and merged:

1. create the Git tag in `vX.Y.Z` format
2. push the tag
3. let the **Build & Release** workflow run from that exact tag

### 4. Validate draft release artifacts

The release workflow currently:

- reruns the canonical verify suite
- builds Tauri bundles for Linux, macOS, and Windows
- smoke-checks bundle artifact presence by target platform
- generates SHA-256 checksum files for bundled artifacts
- uploads artifacts
- creates a draft GitHub release

Before publishing the draft, manually confirm:

- expected artifact types are present
- installer names and versions are correct
- release notes are populated correctly
- platform-specific signing or notarization status is acceptable

## Required local commands

Use these before or during release prep:

```sh
pnpm verify
pnpm test:coverage
pnpm test:rust
pnpm tauri build --debug
```

## Pre-signing package smoke

Before Apple signing and notarization are available, you can still prove that
the local macOS packaging path is healthy.

Recommended current-platform checks:

```sh
pnpm tauri build --debug
bash scripts/release/check-bundle-artifacts.sh src-tauri/target/debug/bundle ".app,.dmg"
hdiutil imageinfo "src-tauri/target/debug/bundle/dmg/RealEstate Listing Optimizer_0.1.0_aarch64.dmg"
```

Optional DMG mount check:

```sh
tmp=$(mktemp -d)
hdiutil attach -readonly -nobrowse -mountpoint "$tmp" "src-tauri/target/debug/bundle/dmg/RealEstate Listing Optimizer_0.1.0_aarch64.dmg"
find "$tmp" -maxdepth 2 -print | sort
hdiutil detach "$tmp"
rmdir "$tmp"
```

Expected outcome before signing:

- the app bundle exists under `src-tauri/target/debug/bundle/macos/`
- the DMG exists under `src-tauri/target/debug/bundle/dmg/`
- `hdiutil imageinfo` reports a readable UDIF/UDZO disk image
- mounting the DMG shows the app bundle and `Applications` shortcut

Note:

- strict `codesign --verify` is not a pre-signing gate for the local debug
  bundle. Treat signing and notarization validation as a later release step
  once Apple credentials are available.

## Recommended smoke checklist

For the final release-candidate build, validate at least:

- app launch
- license activation or cached-license startup
- API key entry in Settings
- property create and edit flow
- photo import and primary-photo reordering
- listing generation
- social generation
- email generation
- CSV import
- PDF export
- DOCX export

## Rollback posture

If a tagged release draft is bad:

1. do not publish it
2. fix on a new `codex/*` branch
3. open a corrective PR
4. retag only after the corrected commit passes verify and packaging

Bad draft release checklist:

- leave the draft unpublished
- note the failing artifact or workflow in the release PR
- close or replace the bad draft after the corrected tag is ready
- update release notes if the scope changed during the fix
- record what failed in the support or PM handoff notes

If a release is already public, create a superseding release rather than
rewriting the published history.

Already-public release checklist:

- document the customer-facing impact
- decide whether support needs a temporary workaround message
- ship the corrective release from a new `codex/*` branch
- publish a superseding release instead of rewriting the original tag
- include the rollback and follow-up context in release notes or support handoff
