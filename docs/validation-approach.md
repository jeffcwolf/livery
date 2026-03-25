# Validation Approach — Rationale and Enforcement

> **The short version:** Correctness (tests pass) and design quality (modules are deep,
> complexity is controlled) are both necessary and neither is sufficient. The validation
> system measures both, continuously, at three granularities. By the time code reaches
> CI, it should already be clean. CI is a deployment gate, not a development tool.

---

## The Core Principle

Validation in an agentic codebase must answer two different questions:

1. **Is the code correct?** Does it do what it is supposed to do? Tests answer this.

2. **Is the design quality holding?** Are modules staying deep, complexity staying
   controlled, and the architecture staying honest? Tests cannot answer this. Only
   design-quality metrics, architectural review, and disciplined comparison against
   baselines can answer it.

Most CI systems answer question 1 and ignore question 2. An agentic codebase that is
validated only for correctness will pass CI indefinitely while its design quality
degrades — because correctness checks do not measure depth ratios, complexity scores,
or documentation coverage. By session 20, the tests are still green and the code is
unmaintainable.

This system validates both, at three granularities, continuously.

---

## The Three Granularities

### Granularity 1: Per-commit (fast, < 30 seconds)

**What runs:** `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`

**Purpose:** Catch mechanical failures instantly, before they reach any review.
Formatting violations, lint warnings, and compile errors should never accumulate.
Each commit must leave the codebase in a state where the next developer (or next
session) can pick up and continue without first cleaning up mechanical problems.

**Enforcement:** Pre-commit hook (via `cargo-husky` or equivalent). Not bypassed.
`cargo clippy -- -D warnings` with `-D warnings` means warnings are errors — there
is no "I'll fix this warning later" in this codebase.

**Why this runs at this granularity:** These checks are fast enough to run on every
commit without friction, and the cost of accumulating their violations is high. A
formatting violation found at session-end is a distraction; a formatting violation
found at commit time is a one-second fix.

### Granularity 2: Per-session (medium, 1–3 minutes)

**What runs:** `cargo test --workspace`, `prism check . --strict --fix-suggestions`,
`prism stats . --json` (saved for delta comparison)

**Purpose:** Confirm the session's deliverable is complete and that no design-quality
metric has degraded. The session does not close until all five gates pass (see
`livery/skills/run-validation.md`).

**The Prism delta is the key contribution of this granularity.** Running
`prism check` alone tells you whether the codebase passes the configured thresholds.
Comparing `prism stats --json` output against the previous session's saved baseline
tells you whether any metric moved — and in which direction. A metric that stayed
above the threshold but decreased is a design quality trend warning. Trend warnings
caught at session end are cheap to address. Trend warnings that compound across ten
sessions become structural problems.

**Enforcement:** `livery/skills/run-validation.md`. The session contract in `livery/CLAUDE-base.md`
requires all gates to pass before the session log entry is written.

### Granularity 3: Per-milestone (deep, human-led)

**What runs:** Human review of `<project>/ARCHITECTURE.md` vs actual code, `prism check --strict`,
full coverage report, `prism map --mermaid` vs ARCHITECTURE.md dependency graph,
manual module-depth assessment.

**Purpose:** Catch failures that per-session validation cannot catch, because they
only become visible at the level of accumulated sessions. A single session cannot
introduce architecture drift; ten sessions can. A single session cannot erode
module depth from deep to shallow; ten sessions of tactical programming can.

**The questions asked at a milestone review:**
- Does ARCHITECTURE.md accurately describe the code? (If not, which drifted?)
- Does `prism map --mermaid` match the documented dependency graph?
- Are all public API items documented with accurate contract descriptions?
- Have any modules drifted shallow since the last milestone? (`prism audit` flags)
- Is the property-test coverage adequate for all data-transforming public functions?

**Enforcement:** Human-led. Mandatory before any milestone is declared complete.
The milestone review is a dedicated session with no implementation work — its only
output is an updated ARCHITECTURE.md and a delta note in SESSIONS.md.

---

## Why Prism is Central

Prism is the tool that makes design quality measurable. Without it, "the design is
good" is a subjective claim. With it, "the module depth ratio has not decreased
in eight sessions" is an objective, measurable claim.

Prism's five subcommands each address a distinct validation concern:

| Command | What it measures | When used |
|---|---|---|
| `prism audit` | Module depth ratios, API surface width, complexity flags | Session-end, milestone |
| `prism check` | Composite release-readiness gate (doc coverage, complexity, tests, coverage) | Session-end (strict), CI, release gate |
| `prism stats --json` | Quantitative baseline snapshot for delta comparison | Session-end (saved and compared) |
| `prism map --mermaid` | Actual dependency graph for comparison against ARCHITECTURE.md | Milestone, architecture changes |
| `prism deps` | Dependency health (stale deps, duplicate versions) | Milestone, pre-release |
| `prism diff` | Compares two prism snapshots and reports regressions. Flags regressions with exit codes (0 = pass, 1 = warn, 2 = fail). | Session-end |

The **dogfood gate** — `mint check .` run on the `mint` repository itself — is the
ultimate integration test. The tool must be able to process its own codebase. This
requirement is in the v1.0 checklist in `<project>/SPEC.md` and is verified by
`mint/skills/prepare-release.md`.

---

## How It's Encoded

| Concern | Where encoded | How enforced |
|---|---|---|
| Three granularities | `AGENTIC_WORKFLOW.md` Phase 4; this document | Described as non-optional layers |
| Per-commit gates | `AGENTIC_WORKFLOW.md` Phase 4 Layer 1 | Pre-commit hook; `scripts/validate.sh` |
| Per-session validation pipeline | `livery/skills/run-validation.md` | Session contract in `livery/CLAUDE-base.md` |
| Prism baseline delta | `livery/skills/run-validation.md` Step 5 | Saved per session; compared at session end |
| Prism diff regression check | `livery/CLAUDE-base.md` §Automated Quality Gate Protocol; `livery/skills/run-validation.md` Step 5 | `prism diff` exit code gates session completion |
| Milestone architectural review | `AGENTIC_WORKFLOW.md` Phase 4 Layer 3 | Human-led; mandatory before milestone close |
| CI mirrors local validation | `AGENTIC_WORKFLOW.md` Phase 5 | GitHub Actions: check → test → prism jobs |
| Prism design-quality gate in CI | `AGENTIC_WORKFLOW.md` Phase 5 Job 3 | `prism check . --json --strict` as a CI job |
| `prism.toml` thresholds | Project root | Configured in Phase 2; not lowered to pass |
| Dogfood gate | `<project>/SPEC.md` v1.0 checklist; `mint/skills/prepare-release.md` Step 5 | `mint check .` on the mint repo |

---

## Why CI is a Deployment Gate, Not a Development Tool

CI should never be the place where failures are caught and fixed. By the time a
commit reaches CI, it should already be clean — per the per-commit and per-session
validation that ran before it was pushed. CI is an objective, environment-independent
confirmation that the codebase meets all quality gates before code is merged or
released.

This distinction matters because treating CI as a development tool creates a workflow
where developers push broken code to CI to find out what is wrong. Each such push
wastes CI resources, delays feedback, and signals that local validation is not being
run. In an agentic workflow, it also creates a pattern where the agent is not verifying
its own work — it is delegating verification to CI. That is the opposite of the session
discipline this system enforces.

The CI pipeline for `mint` has three sequential jobs:

**Job 1: `check`** — `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`.
Fast. Fails immediately on mechanical issues. If this job is ever triggered by a
commit that fails, local validation was not run before pushing.

**Job 2: `test`** — `cargo test --workspace` (debug and release). Full test suite
including property tests with a fixed seed for reproducibility. The release build
catches optimisation-mode-only bugs that the debug build misses.

**Job 3: `prism`** — `prism check . --json --strict`. The design-quality gate. Most CI
pipelines have no equivalent. This is what makes the CI pipeline complete: it does not
just verify that the code is correct; it verifies that the design quality is maintained.

---

## The Shared-Assumption Problem in Validation

Tests can pass while the code is wrong. This is not a theoretical concern — it is the
default outcome when an agent writes both the implementation and the tests. The
validation approach addresses this through:

**Property tests.** Generated inputs expose assumptions that hand-written examples
never surface. A property test generator will eventually produce the input that
breaks the implementation's hidden assumption.

**Reference models.** An independently-written naive implementation provides a ground
truth that neither the agent's implementation nor the agent's examples can contaminate.

**Human review of the red phase.** The human reads the failing test before
implementation begins and asks whether it captures the actual contract. This is the
only validation step that is not mechanically automatable — and it is the most
important one.

**`cargo-mutants` at milestones.** Mutation testing verifies test strength: if a
mutation (changing `<` to `<=`, removing a branch, returning a default) survives
the test suite, the test suite has a gap. Run at milestones on critical modules.

---

## What Failure Looks Like

**Green CI on a degrading codebase.** All tests pass. All lint gates pass. The
design is slowly becoming shallower — modules are gaining responsibilities, the
dependency graph is acquiring unexpected edges, documentation coverage is eroding.
None of this is visible in CI unless `prism check` is a CI job. This is the most
insidious failure mode: everything looks fine from the outside.

**Thresholds lowered to pass.** `prism.toml` min_doc_coverage changed from 80%
to 60% because a session's documentation was insufficient. The gate was lowered
instead of the code being improved. This is the validation equivalent of commenting
out a failing test. The threshold records what was agreed acceptable; lowering it
without deliberate design decision is a quality regression disguised as compliance.

**Prism delta ignored.** Module depth ratio for `mint-meta` decreased from 0.72 to
0.61 between sessions 8 and 12. Each individual session's decrease was small enough
to pass the threshold. The cumulative decrease is a design quality trend that was
never addressed because the per-session delta was never tracked and compared.

---

## Connection to Other Concerns

**Design.** Design quality validation (Prism audit, module depth ratios) is the
measurement arm of the design philosophy. Ousterhout's principles are the statement
of what we want; Prism's metrics are how we verify we achieved it. See
`docs/design-philosophy.md`.

**Testing.** Tests are the foundation of the per-session validation layer. The
`cargo test --workspace` gate is only meaningful if the tests are well-written —
if the shared-assumption problem has been addressed through TDD discipline and
property tests. Validation and testing are interdependent. See `docs/testing-approach.md`.

**Session discipline.** The per-session validation layer is enforced by the session
contract. Validation is not a separate phase — it is the closing act of every session.
The session discipline (defined scope, defined stopping condition) is what makes
per-session validation possible. See `docs/session-discipline.md`.
