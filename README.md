# RealEstate

RealEstate Listing Optimizer is a Tauri desktop app for real-estate agents who
need one local workspace for property data, AI-assisted copy generation, export
packages, and simple delivery analytics.

## Product Snapshot

The current app supports:

- property creation, editing, and deletion
- photo import, photo ordering, and primary-photo selection
- listing, social, and email generation powered by Anthropic
- reusable brand-voice creation from sample listings
- CSV property import with a downloadable template
- dashboard analytics and setup-readiness guidance
- PDF and DOCX export with selectable templates
- local SQLite storage plus license-gated desktop access

AI generation requires an Anthropic API key configured in **Settings**. License
activation is required before the desktop app can be used.

## Quick Start

Prerequisites:

- Node.js 20+
- `pnpm`
- Rust stable toolchain
- Tauri system dependencies for your operating system:
  [Tauri prerequisites](https://tauri.app/start/prerequisites/)

Install dependencies:

```sh
pnpm install --frozen-lockfile
```

Run the frontend in the browser:

```sh
pnpm dev
```

Run the desktop app:

```sh
pnpm tauri dev
```

Run the desktop app in lean mode with temporary caches:

```sh
pnpm dev:lean
```

Optional: change the lean dev port if `1420` is busy:

```sh
LEAN_DEV_PORT=1422 pnpm dev:lean
```

## Verification

The repo uses `.codex/verify.commands` as the canonical gate list. Run the full
stack with:

```sh
pnpm verify
```

Useful focused commands:

```sh
pnpm lint
pnpm typecheck
pnpm test
pnpm test:coverage
pnpm build
pnpm test:rust
pnpm check:rust
```

## Build and Packaging

Frontend production build:

```sh
pnpm build
```

Desktop production build:

```sh
pnpm tauri build
```

Cross-platform release packaging is defined in GitHub Actions under
`.github/workflows/build.yml`. Public release publishing still depends on
external signing credentials and release secrets that are not stored in this
repository.

## Local Maintenance

Remove heavy build artifacts while keeping dependencies:

```sh
pnpm clean:heavy
```

Remove reproducible local caches including `node_modules`:

```sh
pnpm clean:full
```

Check heavy-directory sizes:

```sh
pnpm size:report
```

### Normal vs Lean Dev

- `pnpm tauri dev` is best when you want the fastest incremental rebuild loop.
- `pnpm dev:lean` is best when you want to avoid large persistent cache
  directories in the repo.
- Lean mode trades some startup speed for lower repo-local disk usage.

## Repo Guide

- [User Guide](./docs/USER_GUIDE.md)
- [CSV Import Reference](./docs/CSV_IMPORT_REFERENCE.md)
- [Architecture Notes](./docs/ARCHITECTURE.md)
- [Release Runbook](./docs/RELEASE_RUNBOOK.md)
- [Support Runbook](./docs/SUPPORT_RUNBOOK.md)
- [Troubleshooting](./docs/TROUBLESHOOTING.md)

## Project Layout

- `src/`: React frontend, stores, hooks, and desktop UI flows
- `src-tauri/`: Tauri backend, commands, database access, exports, and migrations
- `.codex/`: canonical verify commands plus local execution helpers
- `scripts/`: git, performance, cleanup, and local workflow utilities
