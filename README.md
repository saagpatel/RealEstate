# RealEstate

[![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript)](#) [![Rust](https://img.shields.io/badge/Rust-dea584?style=flat-square&logo=rust)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> One local workspace for every listing — property data, AI-generated copy, branded exports, and analytics without juggling tabs or SaaS subscriptions.

RealEstate Listing Optimizer is a native desktop app for real estate agents who need to produce polished listing materials quickly. Manage properties, import from CSV, generate listing copy for web, social, and email via Anthropic AI, export PDF and DOCX with selectable templates, and track delivery analytics — all from a single local app with SQLite storage.

## Features

- **Property management** — create, edit, delete listings with photo import, ordering, and primary-photo selection
- **AI copy generation** — listing, social media, and email copy powered by Anthropic Claude
- **Brand voice profiles** — create reusable voice profiles trained from your sample listings
- **CSV import** — bulk import properties with a downloadable mapping template
- **PDF & DOCX export** — professional exports with selectable layout templates
- **Dashboard analytics** — setup-readiness guidance and delivery analytics
- **Local-first** — all data stored in SQLite on your machine; no cloud sync required

## Quick Start

### Prerequisites

- Node.js 24.x
- `pnpm`
- Rust stable toolchain (`rustup`)
- Tauri system dependencies: [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)

```bash
# If using nvm
nvm use
```

### Installation

```bash
git clone https://github.com/saagpatel/RealEstate
cd RealEstate
pnpm install --frozen-lockfile
```

### Usage

```bash
# Run frontend in browser
pnpm dev

# Run full desktop app
pnpm tauri dev
```

AI copy generation requires an Anthropic API key configured in **Settings**.

## Verification

Run from the repository root with the prerequisites above. For frontend changes,
start with mocked smoke tests and the configured static checks:

```bash
pnpm test:smoke:ui
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```

A focused test can be selected with, for example,
`pnpm exec vitest run src/__tests__/stores/propertyStore.test.ts`.
`pnpm test:smoke` adds focused native property/PDF/DOCX fixtures; Rust and Tauri
platform dependencies are required for that lane. Tests use temporary data and
mock services; an Anthropic key and the personal app database are not prerequisites.

`pnpm verify` runs the broader authoritative list in
[.codex/verify.commands](.codex/verify.commands): frontend checks, build,
performance fixtures, native test compilation, and native integration tests.
[The runner](.codex/scripts/run_verify_commands.sh) creates caches and logs;
`CODEX_CACHE_ROOT` can point them at a disposable directory. The install step
configures Husky hooks by default. For isolated checks in shared Git worktrees,
set `HUSKY=0` for installation and verification to preserve shared hook settings.
Keep the normal branch/commit guards enabled; do not set `CI` merely to skip them.

For UI changes, check the affected flow with `pnpm dev` and synthetic listings;
frontend preview does not prove native exports or persistence. No automated
interaction browser suite is configured. Full desktop checks should use a disposable OS
account, synthetic data, and no live AI generation unless that external effect is
separately intended. Report untested native/provider/human behavior explicitly.

## Tech Stack

| Layer         | Technology                                 |
| ------------- | ------------------------------------------ |
| Desktop shell | Tauri 2                                    |
| Frontend      | React, TypeScript, Vite, Tailwind CSS      |
| Backend       | Rust — property storage, export generation |
| AI            | Anthropic Claude API                       |
| Storage       | SQLite (local app data dir)                |
| Exports       | PDF, DOCX                                  |

## Architecture

The Rust backend owns all property data and file I/O — PDF and DOCX generation happen natively without external services. AI calls go directly to Anthropic from the Rust layer. Brand voice profiles are stored as structured data alongside listings, so copy generation can stay consistent across a portfolio without re-prompting from scratch each time.

## License

MIT
