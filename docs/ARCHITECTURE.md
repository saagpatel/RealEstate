# Architecture Notes

## High-level shape

This is a Tauri desktop application with a React frontend and a Rust backend.

- `src/` contains the UI, local state stores, routing, and Tauri bridge calls.
- `src-tauri/` contains commands, persistence, AI integrations, export logic,
  migrations, and desktop-specific behavior.

## Frontend responsibilities

The frontend is responsible for:

- routing and page composition
- property forms and workspace UI
- dashboard analytics and setup guidance
- generation controls and streaming output display
- settings persistence through Tauri command calls
- photo ordering interactions and CSV import triggers

Notable frontend layers:

- `src/pages/` for primary app surfaces
- `src/components/` for reusable UI pieces
- `src/stores/` for Zustand-based app state
- `src/hooks/` for async UX workflows such as generation, export, and photos
- `src/lib/tauri.ts` for command-level frontend/backend contracts

## Backend responsibilities

The Rust side is responsible for:

- database setup and migrations
- property, listing, photo, brand voice, and settings persistence
- Anthropic-backed content generation
- license validation and local cache behavior
- CSV parsing and property import validation
- PDF and DOCX export generation
- analytics event recording for generation and export actions

Notable backend layers:

- `src-tauri/src/commands/` for Tauri command handlers
- `src-tauri/src/db/` for SQLite access
- `src-tauri/src/ai/` for generation and prompt logic
- `src-tauri/src/export/` for PDF, DOCX, and template behavior
- `src-tauri/src/import/` for CSV import logic

## Persistence model

The repo currently uses local SQLite plus local file storage.

Primary persisted domains include:

- properties
- listings and generated outputs
- photos and sort order
- brand voices
- settings
- analytics records
- cached license state

## Export architecture

The export path now supports template-aware package generation.

Current templates:

- Professional
- Luxury
- Minimal

Current formats:

- PDF via `genpdf`
- DOCX via `docx-rs`

Analytics are recorded after successful exports so dashboard metrics can reflect
real package usage rather than schema-only placeholders.

## Verification model

The canonical repo verification list lives in `.codex/verify.commands`.

Current verify coverage includes:

- frozen install
- git guard scripts
- formatting and shell-script checks
- frontend typecheck
- frontend tests
- frontend build
- bundle, build-time, asset, and memory checks
- Rust test compilation

The `pnpm verify` script should be treated as the primary confidence signal for
repo health.

## Release path

GitHub Actions currently splits release work into:

- quality checks in `.github/workflows/test.yml`
- release verification and cross-platform packaging in `.github/workflows/build.yml`
- version-prep PR creation in `.github/workflows/version-bump.yml`

Public release publication still depends on external signing and secret inputs
that are intentionally not stored in this repository.
