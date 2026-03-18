# Livery — Agentic Engineering Workflow
### A Repeatable, High-Quality Pipeline from Idea to v1.0

> **What this is:** The canonical workflow document for the Livery agentic engineering
> system. Livery is a reusable methodology and toolkit for building high-quality,
> maintainable software with AI coding agents (Claude Code, OpenCode, or equivalent).
> It is tool-agnostic at the procedure level. Tool-specific mechanics are noted where
> they differ.
>
> **Location:** This document lives in `livery/WORKFLOW.md` and is part of the Livery
> system. Project-specific files live in their own project folder (e.g. `mint/`).
>
> **Philosophy.** Every phase produces a durable artifact. Every session has a defined
> stopping condition. Validation is continuous, not deferred. Complexity is measured, not
> assumed. The goal is not fast code — it is correct, deep, maintainable code that stays
> that way as the project grows.

---

## Artifact Map

The Livery system separates reusable system files from project-specific files. Both
sets are created once, updated continuously, and never deleted.

### Livery system files (reused across all projects)

| Artifact | Phase Created | Owner |
|---|---|---|
| `livery/WORKFLOW.md` | — | Human (this file) |
| `livery/CLAUDE-base.md` | Phase 2 | Human |
| `livery/docs/` (7 files) | Phase 2 | Human |
| `livery/standards/ousterhout.md` | Phase 2 | Human |
| `livery/standards/readable-code.md` | Phase 2 | Human |
| `livery/standards/rust-specifics.md` | Phase 2 | Human |
| `livery/standards/user.md` | Phase 2 | Human (optional) |
| `livery/skills/` (8 generic skills) | Phase 2 | Human |
| `livery/adapter-superpowers.md` | Phase 2 | Human (if runtime used) |
| `livery/feedback/feedback-loop.md` | Phase 2 | Human |
| `livery/feedback/enforcement.md` | Phase 2 | Human |
| `livery/feedback/known-patterns.md` | Phase 3+ | Agent + Human |
| `livery/feedback/proposals/` | Phase 3+ | Agent (human reviews) |
| `livery/context-management.md` | — | Human (optional, activated when needed) |
| `livery/bin/prism` | Phase 2 | Human (built from source) |

### Project-specific files (one set per project, e.g. `mint/`)

| Artifact | Phase Created | Owner |
|---|---|---|
| `<project>/SPEC.md` | Phase 0 | Human |
| `<project>/SPEC-history.md` | Phase 0 | Human |
| `<project>/ARCHITECTURE.md` | Phase 1 | Human + Agent |
| `<project>/CLAUDE.md` | Phase 2 | Human |
| `<project>/skills/` (project skills) | Phase 2 | Human |
| `<project>/standards/project.md` | Phase 2 | Human (optional) |
| `<project>/scripts/validate.sh` | Phase 2 | Human + Agent |
| `<project>/prism.toml` | Phase 2 | Human |
| `<project>/SESSIONS.md` | Phase 3 | Agent (human reviews) |
| `<project>/.github/workflows/ci.yml` | Phase 5 | Human + Agent |
| `<project>/CHANGELOG.md` | Phase 6 | Agent (human reviews) |

---

## Phase 0 — Ideation & Specification

> *Turn a raw idea into a bounded, unambiguous specification that an agent can never
> silently expand.*

### Purpose
Produce a single document that defines what the software is, what it is not, and what
"done" looks like. This is the upstream contract that every subsequent phase references.
Without it, agents fill ambiguity with invention.

### Inputs
- Raw idea, user need, or problem statement (free-form notes, conversations)
- Prior art / comparable tools surveyed by the human
- Known constraints (target platform, language, license, performance requirements)

### Key Activities
1. Write a one-paragraph problem statement. Be ruthless: one problem, one user, one
   context.
2. Define the **feature scope boundary**: an explicit list of what v1 includes and — equally
   important — what it explicitly excludes. Non-features must be named, not just omitted.
3. Define **v1 success criteria** as a checkable list (see Phase 6). Write these now, even
   though the code does not exist. They become the v1 gate.
4. Identify **risk areas**: the parts of the design that are technically uncertain or likely
   to require iteration. Flag them explicitly so the architecture can isolate them.
5. Record **non-negotiable constraints** (e.g. "must run offline," "must embed in a macOS
   app bundle," "must parse arbitrary UTF-8 input without panicking").

If a runtime with structured brainstorming is present (see the project's adapter
document, e.g. `livery/adapter-superpowers.md`), the runtime's brainstorming skill may
lead the specification conversation. The output must still be written into SPEC.md with
all required sections. The runtime assists the conversation; SPEC.md is the artifact.

### Outputs
**`SPEC.md`** containing:
- Problem statement (≤ 1 paragraph)
- User persona and usage context
- Feature list (v1 scope, explicitly bounded)
- Non-feature list (explicitly excluded)
- v1 success checklist (objective, checkable)
- Risk register (uncertain areas flagged)
- Non-negotiable constraints

### Rules
- The agent does **not** participate in Phase 0 unless a runtime brainstorming skill is
  used to assist the conversation. Even then, the human makes all scope decisions.
- `SPEC.md` is the only place scope is defined. If a feature is not in `SPEC.md`, it
  does not exist.
- Amending `SPEC.md` requires a deliberate decision and a dated changelog entry at the
  top of the file. Scope creep is a process failure, not a natural evolution.

---

## Phase 1 — Architecture & Module Design

> *Make the structural decisions that are expensive to reverse before the agent writes a
> single line of code.*

### Purpose
Produce an architectural document that defines the module/crate structure, the public API
surface of each component, the key types, and the data flow. Agents that lack this
document will make structurally coherent but locally-optimised decisions that accumulate
into a tangled global design.

### Inputs
- `<project>/SPEC.md` (Phase 0)
- `livery/standards/ousterhout.md` — the design checklist is applied during and after drafting
- `livery/standards/rust-specifics.md` — Rust type-system rules applied to API stubs
- Language and ecosystem constraints (for Rust: workspace structure, crate granularity)
- Known risk areas from `<project>/SPEC.md`

> **Note on standards availability:** The `livery/standards/` files are created in Phase 2 of
> the *first* project. For every subsequent project, they already exist and are inputs
> to Phase 1 from the start. On the first project, draft the architecture, then create
> the standards files in Phase 2, then run the audit (step 7 below) before coding begins.

### Key Activities
1. **Decompose into modules.** Apply Ousterhout's deep-module test to every proposed
   component: what complexity does this hide? If the answer is "not much," collapse the
   component into its parent. Aim for a small number of deep modules over a large number
   of shallow ones.
2. **Design it twice.** For every non-trivial module interface, sketch two fundamentally
   different alternatives. State the tradeoffs explicitly. The first design is rarely
   optimal; the act of designing an alternative almost always improves both options.
3. **Define the public API of each module** before any implementation exists. Write the
   function signatures, the key types, and the error types in pseudo-code or real stubs.
   The API is the contract; the implementation is a detail.
4. **Draw the dependency graph.** Every dependency between modules must be one-directional
   and explicitly justified. Circular dependencies are an architectural failure. For a Rust
   workspace: library crates must never depend on the CLI crate; library crates depend on
   each other only when the dependency is documented and directional.
5. **Identify information hiding boundaries.** For each module, list what it conceals from
   its callers. If a module exposes its internal representation, redesign it.
6. **Define the error handling strategy** project-wide. In Rust: `thiserror` for library
   errors, `anyhow` for application-level propagation. Error variants must carry enough
   context to diagnose without reading source.
7. **Run the standards audit.** Before declaring Phase 1 complete, work through the
   Design Process Checklist in `livery/standards/ousterhout.md` (Part III) against every
   module in the architecture. Then apply the Types and Traits checklists in
   `livery/standards/rust-specifics.md` (Part VI) to every public API stub. Fix every
   violation found — do not carry Red Flags into Phase 2. Record any close calls or
   intentional tradeoffs as ADR entries.
8. **Record every significant architectural decision as an ADR** (Architecture Decision
   Record): a numbered entry with context, decision, and consequences. These are permanent.
   They explain why the structure is what it is to every future agent session. ADRs
   generated during the standards audit (step 7) are labelled with their source.

### Outputs
**`ARCHITECTURE.md`** containing:
- Module/crate map with responsibilities
- Public API stubs for each module (signatures, not implementation)
- Dependency graph (can be a Mermaid diagram — `prism map --mermaid` generates this once
  code exists)
- Information-hiding inventory per module
- Error handling strategy
- Numbered ADR log (including audit-sourced ADRs)

### Rules
- No agent writes implementation code until `<project>/ARCHITECTURE.md` exists **and
  the standards audit (step 7) has been completed and all findings resolved.**
- `<project>/ARCHITECTURE.md` is a living document. After every session that changes
  structure, the agent must update it to reflect reality. If the doc and the code
  diverge, the code is wrong — or the doc needs a deliberate, dated amendment.
- The standards files (`livery/standards/ousterhout.md`, `livery/standards/rust-specifics.md`)
  apply to `<project>/ARCHITECTURE.md` itself, not only to code. Architectural decisions
  are subject to the same Red Flag checks as implementation. A shallow module in the
  architecture is a design problem, not a code problem — and it is far cheaper to fix
  before coding begins.
- For large projects (Scribe-scale), `<project>/ARCHITECTURE.md` may split into
  per-subsystem files. A top-level `ARCHITECTURE.md` always exists as the index.

---

## Phase 2 — Project Constitution

> *Encode the rules, philosophy, and reusable procedures so that every agent session starts
> from the same baseline.*

### Purpose
Produce the documents and artefacts that govern agent behaviour across all sessions. This
is the difference between a workflow that degrades over time and one that stays consistent
at session 200 as it was at session 1.

### Inputs
- `<project>/SPEC.md` and `<project>/ARCHITECTURE.md` (Phases 0–1, fully audited)
- `livery/standards/` (created here if first project; already exist on subsequent projects)
- `livery/docs/` (created here if first project; already exist on subsequent projects)
- Testing strategy
- Known AI failure modes for the language/ecosystem

### Key Activities

#### 2a. Create the Livery System (first project only)

The `livery/` directory is the most reusable artifact in the entire workflow. It is
written once and inherited by every subsequent project with no modification (or only
minor language-specific adaptations to the standards files).

**`livery/docs/`** — seven rationale documents explaining *why* the system is designed
as it is. Human-facing. See `livery/docs/INDEX.md` for the complete list.

**`livery/standards/`** — three executable rule documents plus optional user conventions:

| File | Contents |
|---|---|
| `livery/standards/ousterhout.md` | APOSD principles as executable rules + all Red Flags with Rust violation/correction examples + the Design Process Checklist |
| `livery/standards/readable-code.md` | ARC naming, commenting, and control flow rules with Rust examples + the Surface Area Checklist |
| `livery/standards/rust-specifics.md` | Rust applications of both: newtypes, type-state, error handling, trait design, naming conventions + the Rust Standards Checklist |
| `livery/standards/user.md` | Optional user-specific conventions that apply to all projects |

**`livery/skills/`** — eight generic procedural skill files (session-open,
review-for-red-flags, review-docs, naming-review, run-validation, update-architecture,
add-crate, bug-fix). These contain no project-specific references.

**`livery/CLAUDE-base.md`** — the base constitution that all project `CLAUDE.md` files
extend. Contains the generic TDD workflow, testing standards, Rust rules, runtime
interface, automated quality gate protocol, and anti-patterns. Contains no
project-specific content.

**`livery/feedback/`** — the self-correction system: `feedback-loop.md` (pattern
detection and proposal protocol), `enforcement.md` (rule escalation register),
`known-patterns.md` (append-only pattern log), and `proposals/` (proposed changes
awaiting human review). See `livery/feedback/feedback-loop.md` for the full protocol.

**`livery/bin/prism`** — pre-compiled Prism binary for automated quality gates. See
`livery/bin/README.md` for build instructions.

**These files apply to design and architecture artifacts, not only to code.** Running
the `livery/standards/ousterhout.md` Design Process Checklist against
`<project>/ARCHITECTURE.md` before coding begins is mandatory (Phase 1, step 7).

#### 2b. Write the Project Constitution

Two files constitute the project constitution:

**`livery/CLAUDE-base.md`** — already exists (created once, reused). Contains:
all generic rules, TDD workflow, testing standards, Rust standards, runtime interface,
automated quality gate protocol, anti-patterns, and the naming-as-design-signal rule.
References `livery/standards/` for detail.

**`<project>/CLAUDE.md`** — project-specific extension. Must contain:
1. An explicit statement that it extends `livery/CLAUDE-base.md` and the agent
   must read that file first
2. If a runtime execution engine is used, a runtime declaration header (see the
   adapter document, e.g. `livery/adapter-superpowers.md`, for the exact text)
3. Project reference document paths (pointing to both `livery/` and `<project>/`)
4. Crate/module responsibility boundaries for this project
5. Project-specific constraints (e.g., mint's offline requirement, credential protection)
6. The project-specific session contract commands
7. Project-specific anti-patterns
8. Key types quick-reference for the project

**OpenCode note.** OpenCode reads `AGENTS.md` instead of `CLAUDE.md`. Keep one
canonical file and maintain a symlink or copy script. Content is identical.

#### 2c. Build the Project Skills Directory

The `livery/skills/` directory contains eight generic skills reused across all
projects. The project's own `<project>/skills/` directory contains project-specific
skills — procedures that reference project-specific types, crates, and commands.

Generic skills (in `livery/skills/`, no project-specific content):

| Skill | Purpose |
|---|---|
| `session-open.md` | How to open a session: verify baseline, load context, state scope |
| `review-for-red-flags.md` | Post-session design audit against `livery/standards/ousterhout.md` |
| `review-docs.md` | Documentation coverage and quality audit |
| `naming-review.md` | Full naming audit against ARC and Rust naming rules |
| `run-validation.md` | The complete validation pipeline |
| `update-architecture.md` | Keep `<project>/ARCHITECTURE.md` honest after structural changes |
| `add-crate.md` | Add a new workspace crate correctly |
| `bug-fix.md` | Reproduce → red test → fix → green |

Project-specific skills (in `<project>/skills/`, reference project types and crates):

| Skill | Purpose |
|---|---|
| `add-subcommand.md` | Add a CLI subcommand (references project CLI structure) |
| `write-proptest.md` | Property test patterns for this project's core types |
| `prepare-release.md` | Complete release gate including project-specific dogfood check |

For Scribe-scale projects, add further domain skills to `<project>/skills/`:
`add-ui-component.md`, `add-parser-rule.md`, etc.

Skills are referenced in session prompts by name and path:
"Follow `livery/skills/add-crate.md`" or "Follow `mint/skills/add-subcommand.md`."

#### 2d. Set Up Project Standards

Copy `livery/standards/project.md.template` to `<project>/standards/project.md`.
Add any project-specific conventions that extend or override the shipped standards.
This file is optional — leave it empty if the project has no conventions beyond
CLAUDE-base.md and user.md.

Standards precedence (later overrides earlier):
1. `livery/standards/ousterhout.md`, `readable-code.md`, `rust-specifics.md` (shipped)
2. `livery/standards/user.md` (user-specific)
3. `<project>/standards/project.md` (project-specific)

#### 2e. Initialise the Validation Script

`<project>/scripts/validate.sh` is the local validation pipeline. It must be
deterministic and exit non-zero on any failure. Minimum contents:

```bash
#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy -- -D warnings
cargo test --workspace
livery/bin/prism check . --strict
```

Extend per project. The agent must run this at the end of every session. It is the
definition of "session complete."

If `livery/bin/prism` is not available in the environment, substitute `prism check
. --strict` (assuming Prism is on PATH) or list the command for the human to run
manually.

#### 2f. Configure Prism

Create `<project>/prism.toml` with project-appropriate thresholds. These are not
defaults — they are deliberate decisions made now, before the code exists. Set them
at a level that reflects the design philosophy: high doc coverage, low cyclomatic
complexity, mandatory integration tests.

#### 2g. Confirm Runtime Integration (if applicable)

If the project uses a runtime execution engine (e.g. Superpowers):
- Confirm the runtime is installed and responding
- Confirm the adapter document exists (e.g. `livery/adapter-superpowers.md`)
- Add the runtime declaration header to `<project>/CLAUDE.md`
- Confirm `livery/bin/prism` is executable in the runtime environment

### Outputs
- `livery/` directory (complete, if first project)
- `<project>/CLAUDE.md`
- `<project>/skills/` (project-specific skills)
- `<project>/standards/project.md` (if project-specific conventions exist)
- `<project>/scripts/validate.sh`
- `<project>/prism.toml`

### Rules
- `CLAUDE.md` is read by the agent at the start of every session. If the agent is not
  reading it, make it explicit in the session opening prompt.
- Skills are updated when a procedure changes. A stale skill is worse than no skill.
- `scripts/validate.sh` is the ground truth for "passing." CI mirrors it; it does not
  replace it.

---

## Phase 3 — Session Execution

> *The repeatable unit of work. Every session is a complete, bounded, verifiable cycle.*

### Purpose
Execute implementation work in well-defined sessions that start from a clear scope,
follow the TDD workflow, and end in a verified, documented state. Sessions are the
atomic unit of the agentic workflow — they should be small enough to complete fully
and large enough to be meaningful.

### Inputs
- `livery/CLAUDE-base.md` and `<project>/CLAUDE.md` (always in context — read base first)
- `<project>/SPEC.md` and `<project>/ARCHITECTURE.md` (always in context)
- The relevant skill file(s) for this session's task (from `livery/skills/` or
  `<project>/skills/`)
- `<project>/SESSIONS.md` (the log of prior sessions — read last 2–3 entries)
- Prism baseline from the previous session (captured automatically — see CLAUDE-base.md
  §Automated Quality Gate Protocol)
- The specific scope for this session (human-defined)

### Session Prompt Template

Every session opening should follow this template. Do not deviate; agents that receive
unstructured prompts produce unstructured work.

```
## Session [N] — [Date]

**Scope:** [One sentence. What is being built or fixed.]

**Entry state:** [What exists. What is passing. Paste last SESSIONS.md entry or key facts.]

**Read first:**
1. livery/CLAUDE-base.md
2. <project>/CLAUDE.md
3. <project>/ARCHITECTURE.md §[relevant crate/module]
4. <project>/SESSIONS.md (last 2-3 entries)
5. [relevant skill file if applicable]

**Task:**
[The specific implementation task. Reference ARCHITECTURE.md module if applicable.
 Reference skill file with full path: "Follow livery/skills/add-crate.md" or
 "Follow <project>/skills/add-subcommand.md".]

**Stopping condition:** This session is complete when:
- [ ] [Specific deliverable 1]
- [ ] [Specific deliverable 2]
- [ ] All tests pass (`cargo test --workspace`)
- [ ] `<project>/scripts/validate.sh` exits 0
- [ ] `livery/skills/review-for-red-flags.md` completed
- [ ] `livery/skills/review-docs.md` completed (if public items added/modified)
- [ ] `livery/skills/naming-review.md` completed (if new names introduced)
- [ ] `<project>/SESSIONS.md` updated with this session's entry

**Constraints:**
- Follow livery/CLAUDE-base.md and <project>/CLAUDE.md in full.
- Do not implement anything outside this session's scope.
- Do not refactor code outside the files touched by this task unless it directly
  enables the task (document why in SESSIONS.md if you do).
```

### Key Activities

1. **Agent reads context.** `livery/CLAUDE-base.md`, then `<project>/CLAUDE.md`,
   relevant `<project>/ARCHITECTURE.md` sections, last 2–3 `<project>/SESSIONS.md`
   entries, the applicable skill file. Agent captures the Prism baseline
   (`livery/bin/prism stats . --json > /tmp/prism-session-before.json`).
2. **Red phase.** Agent writes the failing test(s) for the target behaviour. Human
   reviews and confirms they are testing the right contract before implementation begins.
3. **Green phase.** Agent writes minimum implementation to pass tests. No anticipatory
   code. No "while I'm here" additions.
4. **Refactor phase.** Three passes: Ousterhout structure (Pass 1), ARC names (Pass 2),
   ARC expression (Pass 3). All tests remain green. See `livery/CLAUDE-base.md` for the
   full three-pass procedure.
5. **Validation.** Agent runs `<project>/scripts/validate.sh` then the three review
   skills. All gates must pass before the session is declared complete.
6. **Session log.** Agent writes a `<project>/SESSIONS.md` entry (see below) including
   the Prism baseline delta computed from the before/after snapshots.

### SESSIONS.md Entry Format

```markdown
## Session [N] — [YYYY-MM-DD]

**Scope:** [What was built or fixed]

**Decisions made:**
- [Any architectural or design decision taken during this session]
- [Any deviation from ARCHITECTURE.md and why]

**Files changed:** [list]

**Prism baseline delta:**
- Before: [key metrics from prism-session-before.json]
- After:  [key metrics from prism-session-after.json]
- Notes:  [Any metric that moved — positive or negative — explained]

**Open issues / deferred items:**
- [Anything that came up but was intentionally deferred]
```

### Rules
- Sessions must be scoped to a single coherent task. If a session prompt covers two
  independent features, split it.
- The human defines scope. The agent implements it. Scope expansion during a session
  requires explicit human approval and a scope amendment in the session log.
- The red phase is mandatory. If the agent produces implementation before a failing test,
  stop the session and restart.
- Session logs are permanent. Do not edit past entries.

---

## Phase 4 — Continuous Validation

> *Validation is not a phase that happens once — it is a continuous signal that runs at
> three granularities throughout the project.*

### Purpose
Catch both correctness failures (tests, types, lints) and design-quality failures (module
depth degradation, complexity growth, coverage erosion) before they compound. The
validation stack has three layers that run at different frequencies.

---

### Layer 1: Per-Commit (Fast, < 30s)

**What runs:** `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`

**When:** Pre-commit hook (enforced locally via `git hooks` or `cargo-husky`).

**Purpose:** Catch mechanical failures instantly. These should never reach CI.

**Failure response:** Fix immediately. Do not bypass.

---

### Layer 2: Per-Session (Medium, 1–3 min)

**What runs:** `<project>/scripts/validate.sh` — the full local pipeline including
`cargo test --workspace` and `livery/bin/prism check .`

**When:** End of every session, before writing the SESSIONS.md entry.

**Purpose:** Confirm the session's deliverable is complete and that no design-quality
metric has degraded.

**Key Prism commands:**
```bash
livery/bin/prism check . --strict --fix-suggestions   # Quality gate
livery/bin/prism audit .                              # Module depth report
livery/bin/prism stats . --json > /tmp/prism-session-after.json  # Save for delta
livery/bin/prism map . --mermaid                      # Visual structure check
```

**Baseline delta tracking:** Compare `livery/bin/prism stats --json` output against the
session-start baseline (captured automatically per CLAUDE-base.md §Automated Quality
Gate Protocol). Any metric that worsened requires a `<project>/SESSIONS.md` note
explaining why — or it must be fixed before the session closes.

**Failure response:** The session is not complete until `<project>/scripts/validate.sh`
exits 0.

---

### Layer 3: Per-Milestone (Deep, human-led)

**What runs:** Human review of `<project>/ARCHITECTURE.md` vs actual code,
`livery/bin/prism check --strict`, full coverage report, manual module-depth assessment.
At milestones, also run the feedback loop's milestone retrospective (see
`livery/feedback/feedback-loop.md` §3.2).

**When:** At the end of each defined milestone (e.g., a feature group, a subsystem
completion, a version increment).

**Key questions:**
1. Does `<project>/ARCHITECTURE.md` still accurately describe the code structure?
   If not, which diverged — the intent or the implementation?
2. Does `prism map --mermaid` show the intended dependency graph? Are there unexpected
   edges?
3. Are all public API items documented with accurate doc comments and doctests?
4. Have any modules drifted shallow? Does `prism audit` flag items that weren't flagged
   at the last milestone?
5. Is the property-test coverage adequate? Are there public data-transforming functions
   without proptest strategies?
6. Are there recurring patterns in `livery/feedback/known-patterns.md` that warrant a
   proposal?

**Failure response:** A milestone does not close until the human architectural review
passes. Fix-before-close is non-negotiable.

---

### The Shared-Assumption Problem

The most dangerous validation failure in agentic development: the agent writes both the
implementation and its tests, so both can share the same wrong assumption. A test suite
can be 100% green and completely wrong.

**Countermeasures (all encoded in `livery/CLAUDE-base.md`):**
1. **Reference models.** For any complex algorithm or critical business logic, write a
   naive, obviously-correct `_reference` function independently. Use property-based tests
   to verify the optimised implementation matches the reference on arbitrary inputs.
2. **Property tests over example tests.** Example tests can be satisfied by implementations
   that share the test author's assumptions. Property tests over random inputs expose
   assumptions the author didn't know they were making.
3. **Human review of test contracts.** In the red phase, the human reviews the failing test
   before implementation starts. The review question is: "Does this test capture the actual
   contract, or just the implementation I'm about to write?"
4. **Mutation testing** (periodic). Run `cargo-mutants` on critical modules. If a mutation
   survives, a test is weak.

---

## Phase 5 — CI/CD

> *CI is a deployment gate, not a development tool. By the time code reaches CI, it should
> already be clean.*

### Purpose
Provide an objective, environment-independent confirmation that the codebase meets all
quality gates before any code is merged or released. CI mirrors the local validation
pipeline; it does not replace it.

### CI Pipeline

Three sequential jobs. All must pass for a merge to proceed.

#### Job 1: `check`
```yaml
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo check --workspace
```
Fast. Fails instantly on mechanical issues.

#### Job 2: `test`
```yaml
- cargo test --workspace
- cargo test --workspace --release   # Catch release-mode-only bugs
```
Full test suite including unit tests, property tests (with fixed seed for
reproducibility — set `PROPTEST_CASES` to a meaningful number, e.g. 1000), and
integration tests.

#### Job 3: `prism`
```yaml
- prism check . --json --strict
```
The design-quality gate. Fails if any configured threshold is breached. This is the
gate that most CI pipelines lack — correctness without design quality is not enough.

### CD Pipeline

For a Rust CLI: triggered on version tag push (`v*`).

```yaml
- Build release binaries for target platforms (matrix: linux-x86_64, macos-arm64, macos-x86_64, windows-x86_64)
- Run full test suite on each target
- Publish to crates.io (if applicable)
- Create GitHub Release with binaries attached
- Update CHANGELOG.md release date
```

**Rule:** Automate CD from day one. Even if v1.0 is months away, having the CD pipeline
in place means "release" is a one-command operation — a tag push. This eliminates
release-day scrambles.

### Branch Strategy

- `main` — always deployable. Protected. Requires CI passing + human review for merges.
- Feature branches — the unit of work. One session or one coherent task per branch is
  ideal, though not mandatory.
- No long-lived feature branches. Merge frequently. An agent working on a stale branch
  loses context of what `main` has become.

---

## Phase 6 — Version 1.0 Gate

> *v1.0 is defined in SPEC.md, not discovered at the end.*

### Purpose
Provide an objective checklist that determines when the project has reached v1.0. This
is not a feeling — it is a list, created in Phase 0, evaluated here.

### The v1.0 Checklist

#### Functional completeness
- [ ] All features listed in `SPEC.md` v1 scope are implemented
- [ ] All features pass their integration tests (the CLI contract tests are the ground truth)
- [ ] All non-features are confirmed absent from the codebase

#### Code quality (automated)
- [ ] `livery/bin/prism check . --strict` passes with no failures
- [ ] `cargo test --workspace` passes on all target platforms
- [ ] `cargo clippy -- -D warnings` produces zero warnings
- [ ] Line coverage meets the `prism.toml` threshold
- [ ] Zero `#[allow(...)]` suppressions without explanatory comments
- [ ] Zero `.unwrap()` or `.expect()` without documented invariant justification in library code

#### Documentation
- [ ] Every public item (`pub fn`, `pub struct`, `pub enum`, `pub trait`) has a doc comment
- [ ] Every doc comment with a non-obvious function has a `# Examples` doctest that passes
- [ ] `<project>/ARCHITECTURE.md` accurately describes the current code structure (human-verified)
- [ ] `<project>/SPEC.md` non-feature list is confirmed accurate
- [ ] `<project>/CHANGELOG.md` has a v1.0 entry with the complete feature list

#### Process
- [ ] `<project>/SESSIONS.md` is complete and up to date
- [ ] All open issues in `<project>/SESSIONS.md` are either resolved or explicitly deferred to v1.1
- [ ] CD pipeline tested: a release build completes successfully on all target platforms
- [ ] The project passes its own tooling (for Prism: `prism check .` run on the Prism
      codebase itself — the dogfood gate)
- [ ] Milestone retrospective completed (see `livery/feedback/feedback-loop.md` §3.2)

### Rules
- v1.0 is not declared until every box is checked. Partial credit does not exist.
- Items deferred to v1.1 must be written into a `<project>/ROADMAP.md` before v1.0 is tagged.
- The version tag triggers the CD pipeline. The changelog entry is created by the agent
  but reviewed by the human before the tag is pushed.

---

## Tooling Reference

| Tool | Role | Phase |
|---|---|---|
| `cargo fmt` | Formatting gate | 3, 4, 5 |
| `cargo clippy` | Lint gate | 3, 4, 5 |
| `cargo test` | Correctness gate | 3, 4, 5 |
| `proptest` / `quickcheck` | Property-based correctness | 3, 4 |
| `cargo-mutants` | Test strength validation | 4 (milestone) |
| `cargo-tarpaulin` | Coverage measurement | 4, 5 |
| `prism audit` | Module depth / design quality | 3, 4 |
| `prism stats` | Baseline delta tracking | 3, 4 |
| `prism map` | Structural visualisation | 1, 4, 6 |
| `prism check` | Composite release-readiness gate | 4, 5, 6 |
| `assert_cmd` | CLI integration testing | 3 |
| GitHub Actions | CI/CD automation | 5 |

**On runtime execution engines (e.g. Superpowers):** If a runtime is declared in the
project's CLAUDE.md, it handles session execution (brainstorming, planning, subagent
dispatch, code review, git workflow). Livery's constitution and quality gates still
govern — the runtime executes, Livery judges. See `livery/adapter-superpowers.md` for
the integration details. If no runtime is used, WORKFLOW.md governs the full session
flow directly.

---

## Scaling to Large Projects (Scribe-Scale)

For projects exceeding ~20 000 LOC or with multiple major subsystems, apply these
extensions:

1. **Split `<project>/ARCHITECTURE.md`.** One top-level index; per-subsystem files
   referenced from it. The agent for a session involving the parser subsystem only loads
   the parser architecture doc, not the entire document.

2. **Subsystem-scoped sessions.** Session prompts explicitly name the subsystem boundary.
   Agents must not cross subsystem boundaries without explicit instruction.

3. **Subsystem-level Prism baselines.** Run `livery/bin/prism check <subsystem-path>` in
   addition to the workspace-level check. Subsystem quality can degrade even when
   workspace metrics look stable.

4. **Skill proliferation management.** At Scribe scale, the `<project>/skills/` directory
   will grow large. Add a `<project>/skills/INDEX.md` that categorises skills and is
   referenced in `<project>/CLAUDE.md`. An agent that cannot find the right skill will
   improvise — the index prevents this.

5. **Milestone-gated architecture reviews.** At this scale, the per-milestone architectural
   review (Layer 3 validation) becomes mandatory and time-boxed. Schedule it as a
   deliberate session with no implementation work — its only output is an updated
   `<project>/ARCHITECTURE.md` and a delta note in `<project>/SESSIONS.md`.

6. **Session log search.** `<project>/SESSIONS.md` becomes unwieldy past ~50 sessions.
   Consider splitting into yearly or milestone-scoped files, with an index. The agent
   should be able to find the session where any key decision was made.

---

## Summary

The Livery workflow is built on a single insight: **agentic coding tools are powerful
but amnesiac**. Each session starts from near-zero context, agents fill ambiguity with
plausible-but-wrong invention, and the failure modes of AI-generated code are systematic
rather than random — shallow modules, trait proliferation, tests that share assumptions
with the implementation, documentation that drifts from reality. A naive agentic workflow
produces fast code that becomes unmaintainable. Livery exists to prevent that.

The system separates into two concerns: the reusable engineering methodology (`livery/`)
and the project-specific application of it (`<project>/`). The methodology — standards,
skills, base constitution, feedback system, rationale docs — is written once and inherited
by every subsequent project. The project-specific harness — specification, architecture,
CLAUDE.md extension, project skills, session log — is written fresh for each project,
grounded in the reusable methodology.

The pipeline runs in six phases. Phases 0 and 1 are primarily human: writing the
specification that bounds scope and the architecture document that defines structure.
A runtime execution engine may assist Phase 0 brainstorming, but the human makes all
scope decisions. These two documents are the upstream contracts that every subsequent
session references. Skipping them is the single most common cause of agentic codebases
that grow brittle — the agent never had a reliable model of what it was building.
Phase 2 encodes the project constitution: `livery/CLAUDE-base.md` and `<project>/CLAUDE.md`
together give every session the same design philosophy and coding rules; the skills
directories give it the same reusable procedures; `<project>/scripts/validate.sh` gives
it the same definition of done. Phase 3 is the repeatable unit of work: bounded sessions
with structured opening prompts, mandatory TDD discipline (no implementation before a
failing test), and a session log entry that records every decision made and every metric
that moved. Sessions are small by design — a large task that cannot be scoped to a session
is a design problem, not a scheduling problem.

Phase 4 is the spine of the workflow: continuous validation at three granularities. Fast
mechanical gates on every commit; a full design-quality pipeline (`prism check`) at the
end of every session; and a human-led architectural review at every milestone. The Prism
tooling is the differentiator here — it makes design quality measurable rather than
subjective, and the baseline delta tracking across sessions means gradual degradation is
caught before it compounds. The shared-assumption problem — where agent-generated tests
share wrong assumptions with agent-generated implementations — is addressed through
property-based tests, reference models, and human review of test contracts before the
green phase begins. Phase 5 is a lightweight CI/CD layer that mirrors the local pipeline
and automates release builds from day one. Phase 6 is not a phase you reach — it is a
checklist you wrote in Phase 0, and v1.0 is declared when every box is checked, not when
the work feels done.

The feedback system (`livery/feedback/`) closes the loop: patterns detected across
sessions feed into proposals for improving the standards, skills, and quality gates. The
methodology gets better with use. The principles are stable; the skill at following them
improves over time.

The result, applied consistently, is a codebase that stays as clean at session 200 as it
was at session 10: deep modules with narrow public APIs, property-tested business logic,
an architecture document that reflects the code, and a session log that makes every
design decision traceable. The same workflow that produces a tight 500-line CLI produces
an 85 000-line desktop application — the scale changes the session count and the skill
library, but not the structure of the pipeline itself.