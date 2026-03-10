# Troubleshooting

## App opens to license activation

That is expected when the desktop app does not yet have a valid license on the
machine.

Check:

- the license key was pasted correctly
- the device has network access for first-time validation
- the LemonSqueezy configuration in the build is pointed at the intended
  production environment

## The app opens but generation is disabled

Generation requires a valid Anthropic API key in **Settings**.

Check:

- the API key is present
- the key starts with the expected Anthropic prefix
- the key belongs to the environment you intend to use

## CSV import fails

The CSV importer expects the provided template shape.

Check:

- required columns are present
- property type values match supported options
- numeric values such as beds, baths, sqft, and price are valid
- multi-value fields use JSON arrays instead of comma-separated free text

Use [CSV Import Reference](./CSV_IMPORT_REFERENCE.md) for the exact header and
value rules.

## Export fails

If PDF or DOCX export fails:

- verify the property has generated content to export
- verify imported photos still exist on disk if the template includes photos
- try the **Minimal** template to isolate photo-related issues

## Photos are not appearing

Check:

- the original imported files still exist if a flow references them directly
- the app has permission to read the chosen photo files
- the property has not already reached the photo-count limit

## Verify fails locally

Run the canonical path:

```sh
pnpm verify
```

If it fails, check the failing stage directly:

```sh
pnpm lint
pnpm typecheck
pnpm test
pnpm build
pnpm check:rust
```

## Frozen install fails

This repo treats `pnpm install --frozen-lockfile` as a real health check.

If it fails:

- update the lockfile intentionally
- review why the dependency graph changed
- include the lockfile rationale in the PR

Do not bypass frozen installs as a convenience fix.

## Desktop packaging fails in CI

Common causes:

- missing Linux system dependencies in the runner
- missing Apple signing or notarization secrets
- missing Tauri signing secrets
- version drift between `package.json`, Cargo, and `tauri.conf.json`

Use the release workflow logs plus the release runbook to isolate the failing
platform-specific requirement.

## Before escalating to engineering

Use [Support Runbook](./SUPPORT_RUNBOOK.md) to collect the version, operating
system, reproduction steps, evidence, and any property- or CSV-specific context
before filing a bug.
