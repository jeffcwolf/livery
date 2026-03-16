# Documentation Approach — Rationale and Enforcement

> **The short version:** Documentation in this system serves two purposes: it
> communicates contracts to callers, and it probes the design. If you cannot document
> a function's contract without describing its implementation, the function's design
> is leaking. Documentation difficulty is a design signal.

---

## The Core Principle

There are two fundamentally different things a doc comment can describe: what a
function *guarantees* (the contract), and how a function *works* (the implementation).
Only the contract belongs in the public documentation. The implementation is a detail —
it can change at any time, and callers should neither know nor care about it.

This distinction matters more in an agentic codebase than in a human-authored one.
An agent reading a doc comment that says "calls `serde_yaml::from_str` to parse the
input" now knows the implementation detail. In a future session, it will write code
that depends on that detail — subtly coupling the caller to the implementation. Over
twenty sessions, these subtle couplings accumulate into a codebase where nothing can
be changed without breaking something unexpected.

The rule: **doc comments describe contracts, not implementations.** The rule has a
corollary: **if you cannot describe the contract without mentioning the implementation,
the module is not hiding its implementation.** That is a design problem, not a
documentation problem.

---

## Why This Approach

### Why "why not what"

Boswell & Foucher's rule from *The Art of Readable Code* — comments explain *why*,
not *what* — is the most important single rule in the documentation approach. Its
application to doc comments is specific: the *what* is the contract (return value,
error conditions, preconditions); the *why* is the reasoning behind non-obvious
decisions. The *how* (implementation details) is almost never appropriate in a doc
comment.

Without this rule, agent-generated doc comments default to summarising the
implementation — the code's structure is visible to the agent, so the agent naturally
describes it. The result is documentation that is always in danger of going stale
(the implementation changes, the comment does not) and that contaminates the reader's
understanding of the interface with implementation knowledge they should not have.

### Why documentation coverage is a hard requirement

A public function without a doc comment has made the contract invisible. Callers must
infer the contract from the implementation — which means callers are depending on the
implementation. Invisible contracts are the precondition for the information leakage
Red Flag.

The coverage requirement (`RUSTDOCFLAGS="-D missing_docs" cargo doc`) is not a style
preference. It is the enforcement mechanism that keeps contracts visible. Every `pub`
item that lacks a doc comment is a contract that has been hidden from callers. In the
context of agentic sessions, a hidden contract is a contract that will be guessed at
— and guessed at wrongly — in future sessions.

### Why doctests for non-trivial functions

A doctest is an example that is also a test. It serves two purposes simultaneously:
it shows callers how to use the function, and it verifies that the example is correct
(and stays correct as the implementation changes). Without a doctest requirement,
agents write doc comment examples that are never compiled — examples that may be wrong
or stale without anyone noticing.

The doctest requirement is bounded: it applies to non-trivial functions, not to
obvious ones. `as_str()` on a newtype does not need a doctest. A function that accepts
optional parameters and has conditional error behaviour does.

### Why documentation as design probe

This is the connection between documentation and design that is easy to miss.
Writing a clear, concise doc comment for a function is a design test as much as
a documentation task. The question "what does this function guarantee to its callers?"
is the same question as "what is this function's contract?" — and the inability to
answer it clearly is the same signal as the Ousterhout "hard to describe" Red Flag.

If you cannot write a one-sentence doc comment for a function without:
- Using "and" (two responsibilities)
- Mentioning the implementation (contract not separated from implementation)
- Writing more than two sentences for the error conditions (too many error paths)
- Using vague words like "process" or "handle" (unclear purpose)

...then the function has a design problem. The documentation difficulty is the symptom;
the design problem is the cause. The `review-docs` skill captures this in its "design
signal" pass — documentation review is also design review.

---

## How It's Encoded

| Concern | Where encoded | How enforced |
|---|---|---|
| Contract vs implementation distinction | `livery/standards/readable-code.md` Part II; `livery/standards/ousterhout.md` Red Flag: Implementation Documentation | `review-docs` skill Step 3 |
| Why not what rule | `livery/standards/readable-code.md` Rule: Comments Explain Why | Refactor Pass 3; `review-docs` Step 3 |
| Coverage requirement | `livery/CLAUDE-base.md` Rust Standards visibility section | `review-docs` Step 1 (`-D missing_docs`) |
| Doctest requirement | `livery/CLAUDE-base.md` Testing Standards; `livery/standards/rust-specifics.md` | `review-docs` Step 5 |
| Module-level `//!` comment requirement | `livery/standards/rust-specifics.md` Part V | `review-docs` Step 2 |
| Documentation as design probe | `livery/standards/readable-code.md` Part V; `livery/skills/review-docs.md` Step 6 | `review-docs` Step 6 "design signal pass" |
| Surprise and limitation annotation | `livery/standards/readable-code.md` Rules: Record Surprises, Announce Flaws | Refactor Pass 3 |
| Director comments for non-obvious structure | `livery/standards/readable-code.md` Rule: Director Comments | Module creation; `review-docs` Step 2 |

---

## How It's Enforced in Practice

### During Refactor Pass 3

The third pass of the TDD Refactor phase is ARC expression, which includes
documentation. Every comment written during Green is reviewed:
- Strip comments that restate the code (what)
- Add comments for non-obvious decisions (why)
- Write `// SAFETY:` comments for every `.expect()` that was added

Every public item added during Green gets a doc comment during Refactor if it does
not already have one.

### During session-end `review-docs` skill

Five steps, in order:
1. **Coverage** — `RUSTDOCFLAGS="-D missing_docs" cargo doc` — mechanical
2. **Module comments** — quality review of `//!` comments
3. **Function comments** — contract vs implementation check for every new `pub fn`
4. **Type comments** — invariants and representation check for every new `pub` type
5. **Doctests** — presence check for non-trivial functions
6. **Design signal pass** — documentation difficulty flagged as design issues

Step 6 is the unique contribution of this skill: it feeds findings back into the
design process. A function whose doc comment required more than two sentences for its
error conditions is a candidate for error type simplification. A module whose `//!`
comment required "and" is a candidate for splitting.

### At the v1.0 gate

`prepare-release` skill Step 4 runs the full documentation checklist across the entire
workspace, not just the most recent session's files. This is the milestone-level
review: every public item in every crate must have a doc comment, and a sample of
those comments must be read and verified as contract descriptions, not implementation
summaries.

---

## What Failure Looks Like

**The implementation-summary comment.** The most common failure mode:

```rust
// BAD: summarises the implementation
/// Calls `git ls-files` to enumerate tracked files, then filters for files
/// that exist on the filesystem and are not in the exclude list.
pub fn list_tracked_files(project_dir: &Path) -> Result<Vec<PathBuf>, ArchiveError>;

// GOOD: describes the contract
/// Return all files tracked by git in the given project directory.
/// Returns Err if the directory does not contain a git repository, or if
/// git is not installed.
pub fn list_tracked_files(project_dir: &Path) -> Result<Vec<PathBuf>, ArchiveError>;
```

The bad comment tells you *how* it works (calls `git ls-files`, filters the result).
The good comment tells you *what* it guarantees (all git-tracked files) and *when*
it fails (no git repo, git not installed). A caller should care about the contract,
not the implementation.

**The stale doctest.** A `# Examples` section added when the function was written,
never updated when the function's signature changed. Without `cargo test --doc`, this
goes unnoticed. The doctest fails in production; no one knows why because the failure
is invisible in normal CI. This is why `cargo test --doc --workspace` is part of the
`prepare-release` checklist.

**Missing `//!` module comments.** An agent starting a session on `mint-check` reads
`src/lib.rs`. If there is no module-level comment, the agent must infer the module's
purpose from the code. It will infer correctly most of the time, and incorrectly
enough times to cause subtle bugs. A two-sentence `//!` comment costs nothing and
prevents this entirely.

**Documentation that passes coverage but fails quality.** The coverage check
(`-D missing_docs`) catches absence. It cannot catch the comment that says:
```rust
/// Gets the title.
pub fn title(&self) -> &str;
```
This passes coverage. It adds no information beyond the function name. The design
signal pass in `review-docs` is what catches this — "does this comment add information
beyond the name?" is a human judgment call that no linter can make.

---

## Connection to Other Concerns

**Design.** Documentation difficulty is a design signal. The inability to write a
clean contract doc comment for a function is the same signal as Ousterhout's "hard
to describe" Red Flag. See `docs/design-philosophy.md`. The `review-docs` skill feeds
design findings back to the design audit.

**Naming.** The ARC rule "do not comment bad names — fix the names" creates a direct
connection between documentation and naming. A comment that exists to explain what a
name means is evidence that the name is wrong. Both concerns are audited in the same
phase (Refactor Pass 2 and 3). See `docs/naming-approach.md`.

**Testing.** Behaviour-named tests are a form of documentation — a machine-executable
specification of the module's behaviour. Well-named tests reduce the documentation
burden on doc comments: the test `parse_citation_rejects_missing_title` documents
that behaviour more precisely than any prose doc comment could. Tests and docs are
complementary, not redundant.
