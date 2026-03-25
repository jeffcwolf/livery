# CLAUDE-base.md — Livery Base Constitution

> **What this file is:** The generic base constitution for all projects built with the
> Livery agentic engineering system. It encodes the design philosophy, TDD workflow,
> testing standards, Rust coding rules, and session discipline that apply to every
> project regardless of its specific domain.
>
> **What this file is not:** A complete CLAUDE.md. Every project extends this file
> with a project-specific `CLAUDE.md` that adds crate boundaries, project-specific
> session contract commands, and project-specific skill references.
>
> **How to use:** At the start of every session, read this file first, then read the
> project's `CLAUDE.md`. Where the two files conflict, the project `CLAUDE.md` wins.
>
> **Livery system location:** `livery/` directory. For rationale behind every rule
> here, see the corresponding file in `livery/docs/`.

---

## Reference Documents

Load the relevant file when the corresponding task arises. Do not rely on memory —
load the file. The project `CLAUDE.md` specifies the exact paths for this project.

| Document | Load when |
|---|---|
| `livery/docs/INDEX.md` | Orienting to the system; understanding how files connect |
| `livery/docs/design-philosophy.md` | Understanding why Ousterhout's principles were chosen |
| `livery/docs/testing-approach.md` | Understanding the testing rationale and the shared-assumption problem |
| `livery/docs/documentation-approach.md` | Understanding the documentation rationale |
| `livery/docs/naming-approach.md` | Understanding the naming rationale and design-probe connection |
| `livery/docs/session-discipline.md` | Understanding why session structure matters |
| `livery/docs/validation-approach.md` | Understanding the three-granularity validation approach |
| `livery/standards/ousterhout.md` | Designing or reviewing any module; running the design audit |
| `livery/standards/readable-code.md` | Writing or reviewing names, comments, or control flow |
| `livery/standards/rust-specifics.md` | Writing any Rust type, trait, error type, or test |
| Project `SPEC.md` | Scope questions; checking whether a feature is in scope |
| Project `ARCHITECTURE.md` | Designing or modifying any module, crate, or public API |
| Project `SESSIONS.md` | Starting a session (read last 2–3 entries); ending a session |
| `livery/standards/user.md` | User-specific conventions apply to this session |
| `livery/feedback/feedback-loop.md` | Milestone retrospective; writing proposals |
| `livery/feedback/enforcement.md` | Checking whether a rule needs escalation |
| Project `standards/project.md` | Project-specific conventions apply to this session |

---

## Design Philosophy

This system follows John Ousterhout's *A Philosophy of Software Design*. The full
rules and all Red Flags are in `livery/standards/ousterhout.md`. Three principles
override all other instincts:

**1. Deep modules.** Every module must justify its existence by hiding significant
complexity. Before creating any module, answer in one sentence: what complexity does
this hide? If you cannot answer, the module should not exist. If the answer is "not
much", collapse it into its parent.

**2. Information hiding.** Internal representations never appear in public interfaces.
A caller should need to know only what the interface promises — never how it delivers.
The moment an internal type appears in a public function signature, the implementation
is frozen.

**3. Strategic programming.** There is no "we'll clean this up later." Every change
should improve the overall design or at minimum not degrade it. If a task requires a
hack, stop and redesign. If the redesign is too large for this session, record it as
design debt in `<project>/SESSIONS.md` and find a clean approach for the current task.

---

## Runtime Interface

Livery is a design constitution. It defines *what quality the output must meet* but
does not prescribe *how the session is executed*. Execution may be delegated to a
runtime — an external tool, plugin, or workflow system that provides structured
brainstorming, task-level planning, TDD-enforced execution with subagent dispatch,
between-task code review, and git workflow management.

If a runtime is present (declared in the project's CLAUDE.md), Livery defers to it
for execution and focuses on quality gates, design philosophy, process auditing, and
session continuity. If no runtime is present, the agent executes sessions directly
using the workflow described in WORKFLOW.md.

An adapter document (e.g., `livery/adapter-superpowers.md`) maps a specific runtime
to this interface. The adapter is the only Livery document that references the runtime
by name.

**What the runtime does not replace.** Even with a runtime present, the following
remain Livery's responsibility and are never delegated: the design philosophy
(Ousterhout's principles), the three-pass refactoring protocol, property-based tests
and reference models, quality gates (Prism with numeric thresholds), process auditing
(Red Flag, Naming, Documentation reviews), session continuity (SESSIONS.md entries),
documentation standards, naming standards, and language-specific code standards.

**Completion definition.** When a runtime is present, session completion requires both
systems to agree. The runtime considers execution complete (all tasks done, tests
passing, code review clean) — this is necessary but not sufficient. Livery's quality
gate passes, all review skills have run, and the SESSIONS.md entry is written — this
is the actual completion criterion.

---

## Development Workflow: TDD — Red / Green / Refactor

The sequence is non-negotiable in every session, for every piece of behaviour.

**Red:** Write a failing test that captures the contract of the behaviour being
implemented. Confirm it fails for the right reason — missing functionality, not a
compilation error in unrelated code. Do not write implementation before this step.
The human reviews the failing test before implementation begins.

**Green:** Write the minimum implementation to pass the test. Not the cleanest
implementation — the minimum. Do not add functionality the test does not demand.
Green phase names are often temporary (`result`, `val`, `data`) — that is acceptable.
Green phase control flow is often inside-out — that is acceptable. Both are fixed in
Refactor.

**Refactor:** With tests green, work through these three passes in order. All tests
must stay green throughout. All three passes are mandatory in every refactor phase. If
a runtime dispatches tasks to subagents, those subagents must complete all three passes
before committing. This overrides any runtime guidance that defines refactoring more
loosely.

*Pass 1 — Ousterhout (structure):* Load `livery/standards/ousterhout.md`. Can any
module be deepened? Can any abstraction be removed? Does any function do more than
one thing? Run the Design Process Checklist (Part III) on every new or modified
module. Fix every Red Flag before moving to Pass 2.

*Pass 2 — ARC names (surface):* Load `livery/standards/readable-code.md` Part I.
Apply the misunderstanding test to every name written during Green. Replace every
placeholder name. Apply the banned-words check. Apply scope-proportionality. Apply
the boolean-naming rule. If you cannot name a function clearly without "and", that
is a design signal — return to Pass 1 and split the function. Naming difficulty is
a design problem, not a naming problem.

*Pass 3 — ARC expression (surface):* Load `livery/standards/readable-code.md`
Parts II–IV. Strip every comment that restates the code. Add comments for every
non-obvious decision made during Green. Apply guard clauses: invert nested conditions
so the happy path runs at the lowest indentation level. Extract complex boolean
conditions into named explaining variables. Verify every function does one thing at
one level of abstraction.

**Never skip red.** The moment you want to "write the implementation and test it
after" is the moment you produce tests that verify what the code does rather than
what it should do. That is the shared-assumption trap.

---

## Testing Standards

### Unit tests
- Live in `#[cfg(test)] mod tests` at the bottom of the same file as the code.
- Named for behaviour: `parse_citation_rejects_missing_title`, not `test_parse`.
- Use `assert_eq!` with descriptive messages. Never `assert!(result.is_ok())` —
  unwrap or match so failures are informative.

### Property-based tests
- Any function that parses, transforms, or computes must have proptest strategies.
- Live in `#[cfg(test)] mod proptests` in the same file.
- Focus on: roundtrip identity, invariant preservation, equivalence with a reference
  model, idempotence where applicable.
- See `livery/skills/write-proptest.md` for strategy patterns.

### Reference models
- For any complex algorithm, write a naive obviously-correct `_reference` function.
- Use property tests to verify the real implementation matches the reference on
  arbitrary inputs. The reference is the specification.

### Integration tests
- Live in `tests/` at the crate root.
- For CLI crates: use `assert_cmd` to invoke the binary as a subprocess. Test stdout,
  stderr, and exit codes.

### Test quality gate
- A test is only good if mutating the implementation causes it to fail. If you can
  change the implementation without breaking the test, the test is weak. Rewrite it.

### When to run mutation testing
The human runs `cargo mutants --file <file> -- --workspace` locally when any of
these apply:

(a) A session touched complex logic and you want to verify the tests are strong.
(b) 5+ sessions have passed since the last mutation run.
(c) prism diff shows test_ratio declining.
(d) It just feels like the tests might be skating by.

This is a human judgment call, not an automated gate. Record results in the
SESSIONS.md entry.

---

## Rust Standards

Full rules are in `livery/standards/rust-specifics.md`. The hard constraints:

**Errors:**
- Library crates use `thiserror` exclusively. Binary crates use `anyhow`.
- Every error variant carries enough context to diagnose without reading source code.
- No `.unwrap()` in library code without a `// SAFETY:` comment explaining why the
  panic is logically impossible.

**Visibility:**
- `pub` only for items that are part of a crate's external contract with at least
  two real callers (not tests).
- `pub(crate)` for all intra-crate items. Private for everything else.
- Every `pub` item has a doc comment. No exceptions.

**Types:**
- Newtypes only when they enforce an invariant or prevent type confusion.
  A newtype that wraps a field and forwards all methods is a Shallow Module — delete it.
- Any function with two or more boolean parameters must use an options struct or enum.
- `new()` is always infallible. Fallible construction uses `parse()` or `try_new()`.

**Traits:**
- Do not define a trait unless there are at least two implementors (including mocks).
- A trait with one implementor is indirection without benefit — use a concrete type.

**Formatting:**
- All code passes `cargo fmt` and `cargo clippy -- -D warnings` with no exceptions.
- No `#[allow(...)]` without an inline comment explaining why the lint is a false
  positive in this specific location.

---

## Anti-Patterns

Refuse to produce these. If you find yourself about to produce one, stop and redesign.

| Anti-pattern | Rule |
|---|---|
| **Shallow wrapper** | A struct wrapping one field that forwards all methods. Delete it; use the inner type directly, or add invariants that justify the wrapper. |
| **Unjustified trait** | A trait with one implementor and no mock need. Use a concrete type. |
| **Premature generic** | A generic type parameter with one concrete instantiation. Start concrete. |
| **Boilerplate builder** | A builder for a struct with fewer than three fields or where all fields are required. Use a plain constructor. |
| **Comment restates code** | A comment describing what the code does in the same terms. Delete it. Comments explain *why*. |
| **Pass-through method** | A method whose body is a single call to another method with the same signature. Delete it. |
| **Tactical hack** | Any shortcut justified by "we'll fix it later." Record it as design debt in `<project>/SESSIONS.md` and find a clean approach. |
| **Scope creep** | Implementing anything not in `<project>/SPEC.md`. Record ideas in `<project>/SESSIONS.md` under deferred items. Do not implement them. |

---

## The Naming-as-Design-Signal Rule

Active throughout every session, not just during review:

At any point during a session, if you cannot name a function, module, or type clearly
without using "and" or a vague word, treat this as a design signal — not a naming
problem. Stop. Ask: does this thing have more than one responsibility? Load
`livery/standards/ousterhout.md` and apply the "hard to describe" Red Flag. ARC and
Ousterhout converge here: if you cannot name it, it probably needs to be split.
Resolve the design question before choosing a name.

---

## Session Process Log

Every session produces a process log: a sequential, append-only record of
what was done, in the order it was done. The log is the audit trail for the
session. It makes TDD rhythm visible, records in-session decisions that
would otherwise be lost, and allows a reviewer to reconstruct exactly what
happened without reading the full session transcript.

### Location

```
process/SESSION-NNN.md
```

at the repository root. `NNN` is the zero-padded session number matching
the session prompt (e.g. `SESSION-006.md`). Create the file at the start
of the session, before any other action.

### Format

Each entry is a single line:

```
[ACTION] [subject] — [detail]
```

**Action types:**

| Tag | When to write it |
|---|---|
| `READ` | Before reading any file (skill, spec, oracle, prior session) |
| `DECISION` | When making any design choice not fully specified in the prompt |
| `RED` | After writing a failing test — before writing any implementation |
| `GREEN` | After the test passes — before the refactor pass |
| `REFACTOR` | After each refactor pass — name the specific change made |
| `GATE` | After running each session gate (fmt, clippy, tests, prism) |
| `DEFER` | When explicitly deciding not to implement something this session |
| `ADR` | When recording an architectural deviation from the spec |

### Timing rule

**Write the entry before taking the action, not after.**

For `READ`: write the entry, then open the file.
For `RED`: write the entry including the expected failure reason, then run
the test to confirm it fails.
For `GREEN`: write the entry, then confirm by running the test suite.

This is the discipline that makes the log a genuine audit trail rather than
a reconstruction. A log written after the fact is indistinguishable from
fabrication.

### Example

```
READ  livery/CLAUDE-base.md
READ  livery/conversion.md
READ  mint/ARCHITECTURE.md §mint-archive
READ  mint/SESSIONS.md (last 2 entries)
READ  mint/skills/write-proptest.md
READ  /tmp/release-scholar/src/archive/tarball.rs (oracle)
DECISION  ArchiveError variants: ProjectDirNotFound, GitNotInstalled,
          NotAGitRepository, GitCommandFailed, OutputDirCreationFailed,
          IoError — chosen for caller informativeness
ADR  Subprocess (git ls-files) instead of git2 crate — git2 not in
     workspace Cargo.toml; adds C dependency; equivalent output
RED   list_tracked_files_returns_error_when_project_dir_missing
      — expected: ArchiveError::ProjectDirNotFound
GREEN list_tracked_files_returns_error_when_project_dir_missing
REFACTOR  renamed `dir` → `project_dir` in test setup (ARC: specific noun)
RED   list_tracked_files_returns_error_when_not_a_git_repo
      — expected: ArchiveError::NotAGitRepository
GREEN list_tracked_files_returns_error_when_not_a_git_repo
RED   list_tracked_files_returns_sorted_paths
      — expected: vec in ascending order
GREEN list_tracked_files_returns_sorted_paths
REFACTOR  extracted sort to end of list_tracked_files; removed sort from
          callers (deep module: callers receive sorted paths as contract)
RED   archive_determinism_same_inputs_same_sha256 (proptest)
      — expected: sha256 equality across two build_archive calls
GREEN archive_determinism_same_inputs_same_sha256
GATE  cargo fmt --check — PASS
GATE  cargo clippy --workspace -- -D warnings — PASS
GATE  cargo test --workspace — PASS (149 tests)
GATE  prism check . --strict — PASS (integration-test gate still failing,
      known deferred)
DEFER integration tests against real git fixture repo — Session 7
```

### What the log is not

The process log is not the SESSIONS.md entry. SESSIONS.md is the permanent
record written after all gates pass. The process log is the working record
written during the session. Both are required; they serve different purposes.

The process log does not need to be prose. Single-line entries only. It is
designed to be scanned, not read.

### Hard constraints

- Create the file before any other action in the session.
- Never batch-write entries. Each entry is written before its action.
- `RED` entries must appear before `GREEN` entries for every test. A
  `GREEN` entry with no preceding `RED` entry for the same test name is
  a process violation.
- The log is committed as part of the session's final commit alongside
  the SESSIONS.md entry. It is a permanent repository artifact.
- Do not delete or rewrite entries. If a decision is reversed, add a new
  `DECISION` entry explaining the reversal — do not edit the earlier one.

---

## Automated Quality Gate Protocol

At session start, capture the Prism baseline:

```bash
livery/bin/prism stats . --json > /tmp/prism-session-before.json
```

At session end, after all implementation is complete, run in this order:

```bash
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings
livery/bin/prism check . --strict
livery/bin/prism stats . --json > /tmp/prism-session-after.json
livery/bin/prism diff /tmp/prism-session-before.json .
```

Record any regressions flagged by `prism diff`. Non-regression deltas are recorded
as a summary table.

> **Exit code semantics:** 0 = pass, 1 = warn, 2 = fail. `prism diff` exits non-zero
> when regressions are detected — this blocks the session or requires justification.

If `prism check . --strict` exits non-zero, the session is not done. Fix the
violations before proceeding. This is non-negotiable.

If `livery/bin/prism` is not executable (missing binary, wrong architecture,
permissions), report the failure to the human with the exact error, list the commands
that need to be run manually, and leave `[PRISM: manual]` placeholders in the
SESSIONS.md entry. Do not skip the gate — the session is not complete until Prism
data is recorded.

---

## Session Contract

The project `CLAUDE.md` specifies the exact commands for this project's session
contract. These are the universal gates that apply to every project:

- [ ] All tests pass (`cargo test --workspace` or project equivalent)
- [ ] No formatting violations (`cargo fmt --check`)
- [ ] No lint warnings (`cargo clippy -- -D warnings` or equivalent)
- [ ] Quality gate passes (`livery/bin/prism check . --strict` or project equivalent)
- [ ] Prism baseline delta computed and recorded
- [ ] Mutation check recorded: `**Mutation check:** [not run / run on <files> — N mutants tested, M survived (details below)]`
- [ ] `livery/skills/review-for-red-flags.md` completed on all modules touched
- [ ] `livery/skills/review-docs.md` completed if public items were added or modified
- [ ] `livery/skills/naming-review.md` completed if new names were introduced
- [ ] `<project>/SESSIONS.md` entry written with scope, decisions, Prism delta, audit results
- [ ] `process/SESSION-NNN.md` committed alongside SESSIONS.md entry
- [ ] Session-end pattern check completed (see `livery/feedback/feedback-loop.md` §3.1)

---

## What to Do When Uncertain

1. **Scope question** ("should I implement X?") → Read `<project>/SPEC.md`.
   If X is not there, the answer is no. Record the idea as a deferred item.

2. **Structure question** ("where does this code belong?") → Read the project
   `<project>/ARCHITECTURE.md`. If the answer is not there, do not improvise — raise it as
   a session decision, resolve it, record it as an ADR.

3. **Design quality question** ("is this module deep enough?") → Load
   `livery/standards/ousterhout.md` and run the Part III checklist.

4. **Naming question** ("is this name good?") → Load
   `livery/standards/readable-code.md` Part I and apply the misunderstanding test.
   If naming is difficult, treat it as a design signal and go to step 3.

5. **Rust idiom question** ("is this the right pattern?") → Load
   `livery/standards/rust-specifics.md`. If no rule covers it, apply the principle
   it would follow from and record the decision in `<project>/SESSIONS.md`.