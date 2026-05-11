# RealEstate — Portfolio Disposition

**Status:** Release Frozen — Tauri 2 + Rust real-estate tool on
`origin/main` with **LemonSqueezy license validation and activation
modal** (a commercial product). Phases 1-5 shipped on canonical
main, plus a `feat(release): finish desktop workflows and repo
hardening` commit indicating release pipeline maturity. Has a
shipped `docs/RELEASE_RUNBOOK.md`. Joins the signing cluster as the
17th member.

> Disposition uses strict `origin/main` verification.
> **First commercial (paid) product in the cluster** — license
> validation changes the signing/distribution calculus.

---

## Verification posture

This repo has both `origin` (`saagpatel/RealEstate`) and
`legacy-origin` (`saagar210/RealEstate`) remotes. **Local clone's
`main` is tracking `origin/main` correctly** — no trap here.

Specifically verified on `origin/main`:

- Tip: `68a5292` (HEAD)
- Substantive commits on `origin/main`:
  - `b428ee9` feat(release): harden local readiness and export smoke
  - `c453fa5` feat(release): finish desktop workflows and repo hardening (#4)
  - `599c7e6` feat: Phase 4 & 5 - Advanced features and comprehensive documentation
  - `6b4402f` feat: Phase 3 - Feature expansion with exports, AI enhancements, and analytics
  - `4689f2c` feat: Add Phase 2 CI/CD infrastructure and release automation
  - `3072def` feat: add LemonSqueezy license validation and activation modal
  - `ac7d9cd` feat(dev): add lean dev mode and cleanup scripts
- **Already-shipped release docs on canonical main:**
  - `docs/RELEASE_RUNBOOK.md` (explicit "prepare and ship a desktop
    release candidate without guessing which commands matter")
  - `docs/SUPPORT_RUNBOOK.md`
  - `docs/TROUBLESHOOTING.md`
  - `docs/USER_GUIDE.md`
  - `docs/CSV_IMPORT_REFERENCE.md`
  - `docs/ARCHITECTURE.md`
  - `CHANGELOG.md` (root)
- Release workflows on `origin/main`:
  - `.github/workflows/build.yml`
  - `.github/workflows/test.yml`
  - `.github/workflows/version-bump.yml`
  - `.github/workflows/git-hygiene.yml`
  - `.github/workflows/lockfile-rationale.yml`
  - `.github/workflows/perf-enforced.yml`
  - `.github/workflows/quality-gates.yml`
- Tree on `origin/main`:
  - `src-tauri/src/ai/brand_voice.rs` (and other AI files)
  - `src-tauri/migrations/{20240101000001_initial,20240201000001_phase3_enhancements,20260310000001_normalize_settings_defaults}.sql`
- Default branch: `main`

---

## Legacy-origin orphan note

`legacy-origin/main` has **zero commits** not on `origin/main`. Clean
state.

---

## Current state in one paragraph

RealEstate is a Tauri 2 + Rust + TypeScript commercial desktop app
for real-estate workflows. The product surface evolved across five
explicit phases ending with "Phase 4 & 5 — Advanced features and
comprehensive documentation," with subsequent release-pipeline
hardening commits. **LemonSqueezy license validation + activation
modal** is on canonical main — this is paid software, not OSS, and
license activation is part of the user onboarding flow. SQLite
schema has three migrations including phase 3 enhancements and a
settings defaults normalization (March 2026). AI features include a
`brand_voice.rs` module suggesting LLM-assisted listing copy or
client communications generation. CSV import is a documented
first-class feature.

For full detail see:

- `README.md` on `origin/main`
- `docs/RELEASE_RUNBOOK.md`
- `docs/USER_GUIDE.md`
- `docs/ARCHITECTURE.md`

---

## Why "Release Frozen" instead of other dispositions

- **Active** — wrong. The operator wrote `RELEASE_RUNBOOK.md` and
  hardened release workflows. The product is positioned for ship,
  not for ongoing scoped feature work.
- **Cold Storage / Archived** — wrong. Commercial product with
  license validation — too high-value to archive.
- **Release Frozen** — correct. But this is the **first commercial
  product** in the cluster, which adds material concerns.

This is the **17th signing cluster member**: DesktopPEt / ContentEngine
/ AIGCCore / Relay / FreeLanceInvoice / Nexus / DeepTank / OPscinema /
ShipKit / SignalFlow / PixelForge / DatabaseSchema / LegalDocsReview /
WorkdayDebrief / TicketDashboard / EarthPulse / **RealEstate**.

---

## Unblock trigger (operator)

When ready to ship:

1. Wire Apple Developer ID + notarization credentials.
2. **Confirm LemonSqueezy posture is production-ready** — production
   store ID, webhook secrets, license key rotation policy, refund
   handling, support workflow. Commercial release blocker.
3. **Audit AI provider posture for `brand_voice.rs`** — if the AI
   layer calls Claude/OpenAI/Ollama, confirm v1 distribution model
   (bundled keys for paid users? per-user keys? hybrid?).
4. Run `pnpm verify` per the runbook; confirm quality workflows
   green.
5. Tag per `version-bump.yml` workflow.
6. Cut release.

Estimated operator time once credentials are in hand: ~4 hours
including LemonSqueezy production cutover audit and notarization
round-trip.

---

## Portfolio operating system instructions

| Aspect               | Posture                                                                                                                                                                                                                                                                     |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Portfolio status     | `Release Frozen`                                                                                                                                                                                                                                                            |
| Commercial posture   | **Paid product** with LemonSqueezy license validation                                                                                                                                                                                                                       |
| Review cadence       | Suspend overdue counting                                                                                                                                                                                                                                                    |
| Resurface conditions | (a) Apple signing credentials wired, (b) LemonSqueezy production cutover audit complete, (c) AI provider strategy decided for `brand_voice.rs`, or (d) operator opens a v1.1 scope packet                                                                                   |
| Co-batch with        | Signing cluster: DesktopPEt / ContentEngine / AIGCCore / Relay / FreeLanceInvoice / Nexus / DeepTank / OPscinema / ShipKit / SignalFlow / PixelForge / DatabaseSchema / LegalDocsReview / WorkdayDebrief / TicketDashboard / EarthPulse / **RealEstate** — **now 17 repos** |
| Special concern      | **First commercial product.** License webhook, refund flow, support runbook all materially affect release-readiness.                                                                                                                                                        |
| Special concern      | **AI integration.** `brand_voice.rs` implies LLM dependency — provider decision.                                                                                                                                                                                            |

---

## Why this row is the first commercial product in the cluster

Every other cluster member (DesktopPEt through EarthPulse) is
either free OSS or operator-personal. RealEstate is **paid software
with license validation**. The release-readiness calculus changes:

- **License webhook reliability matters** more than typical user-app
  reliability — a failed activation blocks a paid customer.
- **Refund / dispute / support paths** need to exist before public
  release, not as v1.1 follow-up. The `SUPPORT_RUNBOOK.md` on
  canonical main suggests the operator already knows this.
- **Apple signing is necessary but not sufficient** — Gatekeeper
  warning kills sale conversion, but so does license-activation
  failure.

This row should be **flagged differently** in any portfolio review:
it's the first row where shipping has commercial revenue exposure,
not just developer-friction exposure.

---

## Reactivation procedure (for the next code session)

1. Verify `git branch -vv` shows `main` tracking `origin/main`.
   Already correct as of this disposition pass.
2. **Re-read `docs/RELEASE_RUNBOOK.md` first** — the operator's own
   runbook is authoritative.
3. Review the local stash (`r9-realestate-stash`) — contains mods
   to `.codex/verify.commands`, perf scripts, AGENTS.md,
   `vite.config.ts`. Decide what belongs on `origin/main`.
4. Delete stale `codex/*` branches that pre-date the
   `feat(release): finish desktop workflows` commit.
5. Run `pnpm verify` per the runbook.
6. **Audit LemonSqueezy production cutover and AI provider strategy
   before signing.**

---

## Last known reference

| Field                   | Value                                                                                                           |
| ----------------------- | --------------------------------------------------------------------------------------------------------------- |
| `origin/main` tip       | `68a5292` (HEAD)                                                                                                |
| Last substantive commit | `b428ee9` feat(release): harden local readiness and export smoke                                                |
| Release runbook         | `docs/RELEASE_RUNBOOK.md` on `origin/main`                                                                      |
| Support runbook         | `docs/SUPPORT_RUNBOOK.md` on `origin/main`                                                                      |
| User guide              | `docs/USER_GUIDE.md` on `origin/main`                                                                           |
| Architecture doc        | `docs/ARCHITECTURE.md` on `origin/main`                                                                         |
| Default branch          | `main`                                                                                                          |
| Build system            | Tauri 2 + Rust + TypeScript + Vite + SQLite (3 migrations including phase 3 enhancements)                       |
| Phases shipped          | 1 (initial) through 5 (advanced features + comprehensive docs)                                                  |
| Release scaffolding     | **Already shipped** — runbook + workflows + version-bump automation                                             |
| Commercial integration  | **LemonSqueezy license validation + activation modal**                                                          |
| AI integration          | `brand_voice.rs` (likely LLM-assisted listing copy)                                                             |
| Blocker                 | Apple signing + LemonSqueezy production cutover + AI provider decision (operator-only)                          |
| Migration state         | `legacy-origin` present but local tracking is correct; **zero orphans on `legacy-origin/main`**                 |
| Distinguishing feature  | **First commercial product in cluster.** License-validation reliability is a release-blocker, not just signing. |
