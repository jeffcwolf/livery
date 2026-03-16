# INDEX.md — The Agentic Engineering System

> **Purpose:** This is the single document that maps the entire system. Read this first.
> It explains what every file does, why it exists, and how it connects to everything else.
> If you want to understand the system as a whole — or explain it to someone else —
> start here.

---

## What This System Is

This is a disciplined agentic coding workflow for producing high-quality, maintainable
software using AI coding agents (Claude Code, OpenCode, or equivalent). It is not a
set of prompting tips. It is a complete engineering system with four layers:

**Layer 0 — Project artifacts** (what is being built)
The design and architecture documents for the specific project being built. These define
scope, structure, and decisions. They are project-specific and live at the repo root.

**Layer 1 — Rationale** (why the system is designed this way)
Narrative documents explaining the principles, reasoning, and connections behind every
major decision in the system. Human-facing. This `docs/` directory. Read these to
understand *why*.

**Layer 2 — Standards** (what the rules are)
Executable rule documents derived from the principles. Agent-facing. The `livery/standards/`
directory. Read these to know *what* the rules are, with concrete examples.

**Layer 3 — Procedures** (how to carry out specific tasks)
Step-by-step skill documents for recurring tasks. Agent-facing. The `livery/skills/` directory.
Read these to know *how* to do a specific task correctly.

**The constitution** (`livery/CLAUDE-base.md`) bridges layers 1–3: it is the short, always-loaded
agent document that points to the right layer for every situation.

---

## Complete File Map

### Layer 0 — Project Artifacts

| File | Purpose | Phase created |
|---|---|---|
| `<project>/DESIGN.md` | Scope contract: what is being built, what is not, what v1.0 means | Phase 0 |
| `DESIGN-history.md` | Audit trail: all design drafts, amendments, and rationale | Phase 0 |
| `<project>/ARCHITECTURE.md` | Structure contract: crate map, public APIs, dependency graph, ADRs | Phase 1 |
| `<project>/SESSIONS.md` | Session log: decisions, Prism deltas, open items, per-session history | Phase 3+ |
| `CHANGELOG.md` | User-facing release history | Phase 6 |
| `prism.toml` | Quality gate thresholds for this project | Phase 2 |
| `<project>/scripts/validate.sh` | Local validation pipeline: the definition of "session complete" | Phase 2 |
| `.github/workflows/ci.yml` | CI/CD pipeline mirroring local validation | Phase 5 |

### Layer 1 — Rationale (`docs/`)

| File | Covers |
|---|---|
| `docs/INDEX.md` | This file. The complete system map. |
| `docs/design-philosophy.md` | Ousterhout's principles: rationale, encoding, enforcement, failure modes |
| `docs/testing-approach.md` | TDD, three test layers, property tests, shared-assumption problem |
| `docs/documentation-approach.md` | Why vs what, contracts, ARC's role, documentation as design probe |
| `docs/naming-approach.md` | ARC naming philosophy, design-probe connection, Rust conventions |
| `docs/session-discipline.md` | Session structure rationale, scope control, institutional memory |
| `docs/validation-approach.md` | Continuous validation, three granularities, Prism's role |

### Layer 2 — Standards (`livery/standards/`)

| File | Covers | Load when |
|---|---|---|
| `livery/standards/ousterhout.md` | APOSD principles as executable rules + all Red Flags with Rust examples | Designing or reviewing any module |
| `livery/standards/readable-code.md` | ARC naming, comments, control flow rules with Rust examples | Writing or reviewing names, comments, control flow |
| `livery/standards/rust-specifics.md` | Rust applications of APOSD and ARC: types, traits, errors, naming | Writing any Rust code |

### Layer 3 — Procedures (`livery/skills/`)

| File | Purpose | When to invoke |
|---|---|---|
| `livery/skills/session-open.md` | How to open a session: verify baseline, load context, state scope | Start of every session |
| `livery/skills/review-for-red-flags.md` | Post-session design audit against ousterhout.md | End of every session |
| `livery/skills/review-docs.md` | Documentation coverage and quality audit | End of every session with new public items |
| `livery/skills/naming-review.md` | Full naming audit against readable-code.md and rust-specifics.md | End of every session with new names |
| `livery/skills/run-validation.md` | The complete validation pipeline | End of every session |
| `livery/skills/update-architecture.md` | Keep ARCHITECTURE.md honest after structural changes | Any session that changes structure |
| `livery/skills/add-crate.md` | Add a new workspace crate correctly | When a new crate is needed |
| `mint/mint/skills/add-subcommand.md` | Add a new CLI subcommand end-to-end | When implementing a subcommand |
| `mint/mint/skills/write-proptest.md` | Property-based test patterns for project types | When writing property tests |
| `livery/skills/bug-fix.md` | Reproduce → red test → fix → green | When fixing a defect |
| `mint/mint/skills/prepare-release.md` | Complete v1.0 release gate | When preparing a release |

### The Constitution

| File | Purpose |
|---|---|
| `livery/CLAUDE-base.md` | Base agent constitution. Extended by project CLAUDE.md. |
| `livery/conversion.md` | Workflow deviations for rebuilding or migrating an existing codebase. | Points to standards and skills. Never restates their content. |

---

## The Workflow Phases

The system is applied in six phases. Each phase has a defined purpose, inputs, outputs,
and gate condition.

| Phase | Name | Purpose | Key output |
|---|---|---|---|
| 0 | Ideation & Design | Bound scope; define success | `<project>/DESIGN.md` |
| 1 | Architecture | Define structure before code | `<project>/ARCHITECTURE.md` (standards-audited) |
| 2 | Project Constitution | Encode rules and procedures | `livery/CLAUDE-base.md`, `livery/standards/`, `livery/skills/` |
| 3 | Session Execution | Build the software | Code + tests + `<project>/SESSIONS.md` |
| 4 | Continuous Validation | Catch degradation early | Prism baseline deltas |
| 5 | CI/CD | Objective deployment gate | Green CI on every merge |
| 6 | Release Gate | Declare v1.0 objectively | All `<project>/DESIGN.md` checklist items checked |

For the full workflow narrative, see `livery/WORKFLOW.md`.

---

## The Seven Major Concerns

The system addresses seven concerns, each documented in `docs/` and enforced through
standards and skills. This table shows the complete enforcement chain for each.

| Concern | Rationale doc | Standards file(s) | Primary skills | CLAUDE.md section |
|---|---|---|---|---|
| Module design | `design-philosophy.md` | `ousterhout.md` | `review-for-red-flags` | Design Philosophy, Refactor Pass 1 |
| Testing | `testing-approach.md` | `rust-specifics.md` | `write-proptest`, `bug-fix` | Testing Standards |
| Documentation | `documentation-approach.md` | `readable-code.md` | `review-docs` | Refactor Pass 3 |
| Naming | `naming-approach.md` | `readable-code.md`, `rust-specifics.md` | `naming-review` | Refactor Pass 2 |
| Session discipline | `session-discipline.md` | — | `session-open`, `run-validation` | Session Contract |
| Validation | `validation-approach.md` | — | `run-validation`, `prepare-release` | Session Contract |
| Architecture integrity | `design-philosophy.md` | `ousterhout.md` | `update-architecture` | Crate Responsibilities |

---

## How to Navigate This System

**If you want to understand why the system is designed a certain way:**
→ Read the relevant file in `docs/`

**If you want to know what the rules are:**
→ Read the relevant file in `livery/standards/`

**If you want to know how to do a specific task:**
→ Read the relevant file in `livery/skills/`

**If you are an agent starting a session:**
→ Read `livery/CLAUDE-base.md` first. It tells you what to load for this session's tasks.

**If you want to understand the project being built:**
→ Read `<project>/DESIGN.md` (scope) and `<project>/ARCHITECTURE.md` (structure)

**If you want to understand what happened in previous sessions:**
→ Read `<project>/SESSIONS.md`

**If you want to understand the full workflow:**
→ Read `livery/WORKFLOW.md`

---

## Design Principles for the System Itself

The system is designed by the same principles it enforces.

**Deep modules.** Each file hides a distinct concern. `livery/CLAUDE-base.md` is short because it
delegates to standards and skills — it does not restate their content. Standards files
contain rules; they do not contain procedures. Skills contain procedures; they do not
contain rules.

**No repetition.** Each rule, principle, and procedure is stated in exactly one place.
Cross-references point to the source; they do not duplicate it. If the same content
appears in two files, one of them is wrong.

**Honest naming.** Files are named for what they contain. `review-for-red-flags.md`
is a procedure for reviewing Red Flags. `ousterhout.md` contains Ousterhout's rules.
No file is named `utils`, `helpers`, or `misc`.

**Explicit over implicit.** Every rule has a violation example. Every procedure has a
stopping condition. Every phase has a gate. Nothing is left to the agent's judgment
when a rule or procedure could cover it.