# adapter-superpowers.md — Mapping Superpowers to Livery's Runtime Interface

> **What this document is:** An adapter that maps a specific execution engine —
> Superpowers v5.x — to Livery's abstract runtime interface (defined in
> CLAUDE-base.md §Runtime Interface). This is the only document in the Livery
> system that references Superpowers by name. If Superpowers changes, update
> this file. If Livery's core changes, update this file. Neither system's
> core documents reference the other.
>
> **Tested with:** Superpowers v5.0.2, Livery as of 2026-03-18.
>
> **What this document is not:** A description of Livery's design philosophy,
> testing standards, or quality gates. Those live in CLAUDE-base.md. This
> document only explains *how Superpowers provides the runtime capabilities
> that Livery expects*.
>
> **Prerequisite:** Superpowers must be installed in the coding agent's
> environment. For Claude Code:
> `/plugin marketplace add obra/superpowers-marketplace` then
> `/plugin install superpowers@superpowers-marketplace`.
> For other platforms, follow the Superpowers installation documentation.

---

## 1. Livery's Runtime Interface (summary)

CLAUDE-base.md §Runtime Interface defines five capabilities that Livery
expects from an execution engine. They are summarised here for convenience;
CLAUDE-base.md is authoritative.

| Capability | What Livery needs |
|---|---|
| **Structured brainstorming** | A Socratic design conversation before code, producing a specification artifact |
| **Task-level planning** | Session scope broken into ordered, bite-sized tasks with file paths and verification steps |
| **TDD-enforced execution** | Red/green/refactor cycle, with the ability to dispatch tasks to subagents |
| **Between-task code review** | Review of each completed task against the plan before proceeding |
| **Git workflow management** | Branch/worktree creation, merge/PR decisions, cleanup |

Livery does not prescribe *how* these capabilities are delivered. It
defines what happens *after* them: quality gates, process auditing,
session continuity. The runtime handles execution; Livery handles quality.

---

## 2. How Superpowers Satisfies Each Capability

### 2.1 Structured Brainstorming → `brainstorming` skill

Superpowers' brainstorming skill leads a Socratic conversation that refines
rough ideas through questions, explores alternatives, and presents the
design in digestible sections for human validation. It saves a design
document at the end.

**Adapter note:** Livery requires the brainstorming output to be written
into SPEC.md format (see §4.1). The project CLAUDE.md should instruct
the agent to write the brainstorming output as a SPEC.md with Livery's
required sections: problem statement, user persona, commands/features,
non-feature list, success checklist, risk register. The brainstorming
skill is the engine; SPEC.md is the artifact.

### 2.2 Task-Level Planning → `writing-plans` skill

Superpowers' writing-plans skill breaks work into bite-sized tasks (2–5
minutes each) with exact file paths, complete context, and verification
steps. Plans are clear enough for a context-free subagent to follow.

**Adapter note:** When the plan includes tasks that involve data
transformation, parsing, or algorithmic work, the plan should note that
Livery's property-test requirement applies (see CLAUDE-base.md §Property-
Based Tests). When the plan includes refactoring, note that Livery's
three-pass refactoring applies. These are constitutional requirements
that the plan must account for in its time estimates and verification
steps.

### 2.3 TDD-Enforced Execution → `subagent-driven-development` skill

Superpowers' subagent-driven-development skill dispatches tasks to
subagents with two-stage review (spec compliance, then code quality).
The `test-driven-development` skill enforces red/green/refactor.

**Adapter note — critical:** Superpowers' TDD cycle ends at refactor.
Livery's TDD cycle extends the refactor phase with three mandatory
passes:

1. **Ousterhout pass** — deepen modules, hide information, simplify
   interfaces, define errors out of existence.
2. **ARC names pass** — misunderstanding test, banned words,
   scope-proportionality.
3. **ARC expression pass** — remove unnecessary indirection, flatten
   nesting, simplify conditionals.

The project CLAUDE.md must state that these three passes are mandatory
and override Superpowers' more generic refactoring guidance. Subagents
executing under the constitution must complete all three passes before
committing.

Where Superpowers' `test-driven-development` skill says to write
minimal code and refactor, Livery agrees — but defines what "refactor"
means more precisely. This is an extension, not a conflict.

### 2.4 Between-Task Code Review → `requesting-code-review` skill

Superpowers' requesting-code-review skill runs between tasks, reviewing
against the plan and reporting issues by severity. Critical issues block
progress.

**Adapter note:** This satisfies Livery's runtime interface requirement.
Livery's own review skills (red flags, naming, docs) run *additionally*
at session end (see §5.5). The two review passes serve different purposes:
Superpowers reviews for correctness and plan compliance; Livery reviews
for design quality and process compliance. Both are required.

### 2.5 Git Workflow Management → `using-git-worktrees` and `finishing-a-development-branch` skills

Superpowers' git skills handle worktree creation, branch management,
and merge/PR decisions.

**Adapter note:** No modification needed. Superpowers' git workflow
integrates cleanly. The only requirement is that the SESSIONS.md entry
(§5.6) must be committed *before* branch finishing begins.

---

## 3. Precedence Rule

Superpowers' `using-superpowers` skill states that project CLAUDE.md
overrides skill defaults. This is the mechanism by which Livery's
constitution takes precedence.

In concrete terms:
- If a Superpowers skill says "simplify" and CLAUDE-base.md says "apply
  Ousterhout's deep-module principle" — follow CLAUDE-base.md. It is
  more specific.
- If a Superpowers skill says "write tests" and CLAUDE-base.md says
  "write property tests for data-transforming functions, with reference
  models for complex algorithms" — follow CLAUDE-base.md. It is more
  demanding.
- If a Superpowers skill says "task complete, tests pass" and Livery's
  quality gate (Prism) says the session fails — the session is not done.
  Prism is authoritative.

**Superpowers' "done" is necessary but not sufficient.** All Superpowers
checks must pass. Then all Livery checks must pass. Only then is the
session done.

The project CLAUDE.md should include this in its header:

```markdown
> **Runtime:** This project uses Livery with Superpowers as the execution
> engine. See `livery/adapter-superpowers.md` for the integration.
> Superpowers skills handle workflow execution. CLAUDE-base.md is the
> design constitution and overrides any conflicting Superpowers guidance.
```

---

## 4. Project Lifecycle with Superpowers

Livery defines a five-phase lifecycle (see WORKFLOW.md). Superpowers is
involved in Phases 0 and 3. The other phases are pure Livery.

### 4.1 Phase 0 — Specification (Superpowers assists)

Superpowers' brainstorming skill runs the Socratic design conversation.
The output is written into Livery's SPEC.md format.

SPEC.md replaces what was previously called DESIGN.md. The rename
reflects the document's true nature: it is a product specification (what
the tool does, for whom), not a software design (how it is built). The
software design lives in ARCHITECTURE.md.

SPEC.md required sections:
- Problem statement and user persona
- Commands / features
- Non-feature list (explicit exclusions with rationale)
- Success checklist (written before code; v1.0 declared when every box
  is checked)
- Risk register

**Phase 0 gate:** SPEC.md exists, is complete, and has been reviewed by
the human developer. Superpowers' brainstorming may have produced it,
but the human signs it off.

### 4.2 Phase 1 — Architecture (Livery only)

ARCHITECTURE.md with API stubs, crate/module decomposition, dependency
graph, ADRs. Superpowers is not involved. See WORKFLOW.md.

### 4.3 Phase 2 — Constitution (Livery only, Superpowers verified)

Project CLAUDE.md, `prism.toml`, project-specific skills. Confirm
Superpowers is installed and responding. Confirm `livery/bin/prism` is
available.

### 4.4 Phase 3 — Implementation (Superpowers executes, Livery governs)

Each session follows the hybrid session flow in §5.

### 4.5 Phase 4 — Release (Livery only)

Prepare-release skill. Final Prism check, mutation testing, dogfood gate.
See WORKFLOW.md and the project's `prepare-release.md` skill.

---

## 5. Hybrid Session Flow

Every implementation session follows this flow. Each step is marked
with who drives it.

### 5.1 Preflight (Livery)

The agent reads documents in this order before writing any code:

1. `livery/CLAUDE-base.md` — the constitution
2. The appropriate mode document (`livery/conversion.md` or
   `livery/WORKFLOW.md`)
3. Project CLAUDE.md
4. ARCHITECTURE.md §[relevant crate/module]
5. SESSIONS.md (last 2–3 entries)
6. The session prompt or scope definition

The agent then captures the Prism baseline:

```bash
livery/bin/prism stats . --json > /tmp/prism-session-before.json
```

This is silent — the numbers are recorded for the delta at session end,
not reported to the human unless asked.

### 5.2 Planning (Superpowers)

Superpowers' writing-plans skill breaks the session scope into ordered
tasks. The plan accounts for Livery's requirements: property tests
noted where applicable, three-pass refactoring included in task
verification steps.

### 5.3 Execution (Superpowers, under Livery constitution)

Superpowers' subagent-driven development dispatches tasks. Each
subagent operates under the Livery constitution because CLAUDE-base.md
is loaded via the project's CLAUDE.md.

The TDD cycle within each task:

1. **Red** — write a failing test. Livery specifies what kinds: unit,
   property (mandatory for data-transforming functions), integration
   (for CLI behavior), reference model (for complex algorithms).
2. **Green** — write minimum implementation to pass.
3. **Refactor** — three passes (Ousterhout, ARC names, ARC expression).
   All three mandatory. All tests green throughout.

Superpowers' code review runs between tasks.

### 5.4 Quality Gate (Livery, automated)

When Superpowers considers execution complete, the agent runs Livery's
quality gate:

```bash
# Standard Rust gates
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings

# Prism quality gate
livery/bin/prism check . --strict

# Stats for delta calculation
livery/bin/prism stats . --json > /tmp/prism-session-after.json

# Baseline delta comparison
livery/bin/prism diff /tmp/prism-session-before.json .
```

If `prism check . --strict` exits non-zero, the session is not done.
The agent fixes violations before proceeding. Non-negotiable.

If `livery/bin/prism` is not executable, the agent reports the failure,
lists commands for manual execution, and leaves `[PRISM: manual]`
placeholders in the SESSIONS.md entry. The session is not complete
until Prism results are recorded.

### 5.5 Process Audit (Livery)

After the quality gate passes:

1. **Red Flag Audit** (`livery/skills/review-for-red-flags.md`) — scan
   all files touched this session. Record findings in SESSIONS.md.
   Critical findings block completion.

2. **Naming Review** (`livery/skills/naming-review.md`) — review all
   new names against ARC criteria. Count reviewed, list renames.

3. **Documentation Review** (`livery/skills/review-docs.md`) — verify
   contract-oriented doc comments on all new/modified public items.
   Run `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --workspace`.

4. **Prism Baseline Delta** — run
   `livery/bin/prism diff /tmp/prism-session-before.json .` and write
   the output into the SESSIONS.md entry. Prism diff compares all 13
   metrics and flags regressions automatically.

### 5.6 Session Entry (Livery)

The agent writes the SESSIONS.md entry:

```markdown
## Session N — [DATE]

**Scope:** [one-line summary]

**Out of scope:** [what was deliberately excluded]

**Entry state:**
- Baseline: [key Prism numbers from before.json]
- Open items from prior sessions: [list or "none"]

**Reference documents loaded:**
- [list of documents read during preflight]

**Skills invoked:**
- [list of runtime and Livery skills used, with findings]

**Decisions made:**
- [every design decision, with rationale]
- [every deviation from ARCHITECTURE.md, recorded as ADR]

**Files changed:**
- [list]

**Prism baseline delta:**
[paste prism diff summary table, or "No regressions. All 13 metrics stable or improved."]
Notes: [any accepted regression and why]

**Red Flag Audit:**
- Ran against: [files]
- Findings: [none / list]

**Naming Review:**
- Names reviewed: [count]
- Renames: [none / list old → new]

**Docs Review:**
- Coverage: [n/n pub items]
- Findings: [none / list]

**Open issues / deferred items:**
- [anything intentionally deferred]
```

### 5.7 Branch Finishing (Superpowers)

After the SESSIONS.md entry is committed, Superpowers handles git
mechanics: merge, PR, or keep worktree. This is the final step.

---

## 6. Future: Livery Skills in Superpowers Format

Four Livery concepts are candidates for extraction as Superpowers-format
SKILL.md files. Extraction gives them auto-activation and discoverability
within the Superpowers framework. This is a design intent, not a current
requirement.

| Skill | Activates when | Source material |
|---|---|---|
| Deep-module refactoring | Refactor phase of TDD cycle | CLAUDE-base.md §Design Philosophy |
| Property-test mandate | Implementing data-transforming functions | CLAUDE-base.md §Property-Based Tests |
| Naming review (ARC) | Session end, process audit | CLAUDE-base.md §Naming, `naming-review.md` |
| Session continuity | Session end | CLAUDE-base.md §Session Contract |

If extracted, these skills would live in the Livery repository and be
installed alongside Superpowers. They would carry Livery's intellectual
content while benefiting from Superpowers' discovery and testing
infrastructure.

---

## 7. Automated Prism Integration

### 7.1 The Problem

In pure Livery (as used in the mint project), the human developer ran
Prism locally and pasted results into SESSIONS.md entries. This was the
single largest per-session time sink.

### 7.2 The Solution

A pre-compiled, statically-linked Linux x86_64 Prism binary at
`livery/bin/prism`. The agent runs it directly — no Rust toolchain
required, no network access needed.

### 7.3 Binary Maintenance

Rebuild when Prism's source changes (infrequently):

```bash
# On Linux or via cross-compilation:
cd ~/code/prism
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/prism
cp target/x86_64-unknown-linux-gnu/release/prism livery/bin/prism
chmod +x livery/bin/prism
```

For cross-compilation from macOS:
```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/prism livery/bin/prism
```

The binary is committed directly to the Livery repo (typically 8–12MB
stripped).

### 7.4 Fallback

If `livery/bin/prism` is not executable:

1. Report failure to the human with the exact error.
2. List commands for manual execution.
3. Leave `[PRISM: manual]` placeholders in SESSIONS.md.
4. Do not declare the session complete until numbers are filled in.

---

## 8. Migration from Pure Livery

For projects built with Livery before this adapter existed:

1. **No retroactive changes.** Existing SESSIONS.md entries, ARCHITECTURE.md,
   and project CLAUDE.md remain valid.

2. **Rename DESIGN.md to SPEC.md.** Update references in CLAUDE.md. This
   is terminology, not content.

3. **Add the runtime declaration** to the project CLAUDE.md header (see §3).

4. **Place the Prism binary** at `livery/bin/prism`.

5. **Update the session contract** in CLAUDE.md to reference automated
   Prism execution instead of manual.

---

## 9. When to Update This Document

Update `adapter-superpowers.md` when:

- Superpowers releases a new major version that renames skills, changes
  the subagent protocol, or alters the plugin interface.
- Livery's runtime interface (CLAUDE-base.md §Runtime Interface) adds
  or removes a required capability.
- A new Superpowers skill becomes relevant to a Livery capability that
  was previously unmapped.

Do not update this document for minor Superpowers releases, bug fixes,
or changes to Superpowers skills that are not mapped to Livery's runtime
interface.

---

*Adapter version: 1.0. Written 2026-03-18.*
*Maps: Superpowers v5.0.2 → Livery runtime interface v1.*