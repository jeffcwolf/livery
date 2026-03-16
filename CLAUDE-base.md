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
| Project `DESIGN.md` | Scope questions; checking whether a feature is in scope |
| Project `ARCHITECTURE.md` | Designing or modifying any module, crate, or public API |
| Project `SESSIONS.md` | Starting a session (read last 2–3 entries); ending a session |

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
must stay green throughout.

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
| **Scope creep** | Implementing anything not in `<project>/DESIGN.md`. Record ideas in `<project>/SESSIONS.md` under deferred items. Do not implement them. |

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

## Session Contract

The project `CLAUDE.md` specifies the exact commands for this project's session
contract. These are the universal gates that apply to every project:

- [ ] All tests pass (`cargo test --workspace` or project equivalent)
- [ ] No formatting violations (`cargo fmt --check`)
- [ ] No lint warnings (`cargo clippy -- -D warnings` or equivalent)
- [ ] Quality gate passes (Prism or project equivalent tool)
- [ ] `livery/skills/review-for-red-flags.md` completed on all modules touched
- [ ] `livery/skills/review-docs.md` completed if public items were added or modified
- [ ] `livery/skills/naming-review.md` completed if new names were introduced
- [ ] `<project>/SESSIONS.md` entry written with scope, decisions, Prism delta, audit results

---

## What to Do When Uncertain

1. **Scope question** ("should I implement X?") → Read `<project>/DESIGN.md`.
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
