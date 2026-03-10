# Support Runbook

## Goal

Use this runbook to collect the minimum useful context before escalating a user
issue to engineering.

## Collect this for every report

- app version
- operating system and version
- what the user expected to happen
- what actually happened
- exact reproduction steps
- screenshot or screen recording when available

## Collect this when relevant

### License issues

- whether this is first activation or a previously working machine
- whether the app is offline
- the visible license error message

### API generation issues

- whether an Anthropic API key is configured
- which generation flow failed: listing, social, or email
- whether the failure is consistent or intermittent

### Property-specific issues

- property address or internal property ID if available
- whether photos are attached
- whether a brand voice was selected

### CSV import issues

- the CSV file used for import
- the row numbers that failed
- whether the file came from the app template

### Export issues

- export format used: PDF or DOCX
- template used: Professional, Luxury, or Minimal
- whether the property has generated content
- whether the property has imported photos

## Triage framing

Classify the issue before handoff:

- `P0`: app cannot open or core data is at risk
- `P1`: major workflow blocked for most users
- `P2`: degraded workflow with workaround
- `P3`: polish issue or narrow edge case

## Escalation package

Before sending to engineering, assemble:

- short summary
- severity
- reproduction steps
- collected evidence
- affected version
- any workaround already confirmed
