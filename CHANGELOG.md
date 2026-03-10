# Changelog

All notable changes to this project should be tracked in this file.

## [Unreleased]

### Added

- dashboard analytics summary and CSV import entry point
- dashboard setup-readiness checklist for first-run guidance
- AI model selection in Settings
- export template selection for PDF and DOCX packages
- photo ordering and primary-photo controls in the property workspace
- canonical verify flow and worktree-safe local environment helpers

### Changed

- split production type/build validation from test-only validation
- aligned CI quality and release workflows with the current repo scripts
- moved analytics recording from schema-only support to live generation/export paths

### Fixed

- Rust export, analytics, and CSV-import compile blockers
- stale frontend test fixtures and type drift
- lockfile and tooling drift that broke frozen installs
