# Skill: session-open

> **When to invoke:** At the start of every coding session, before writing any code.
> This skill is the entry ritual that prevents the most common session failure modes:
> starting without context, drifting from scope, and building on a broken baseline.

---

## Why This Skill Exists

The most expensive session failures happen in the first five minutes:
- Starting work before reading what the last session decided
- Implementing something that was already done, or already decided against
- Building on a codebase that has a pre-existing failing test or lint error

This skill costs two minutes at the start of a session. It prevents hours of wasted
work and silent regressions.

---

## Procedure

### Step 1 — Verify the baseline is clean

Before reading any context, confirm the codebase is in a clean state:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

If any of these fail: **stop**. Do not proceed into the session's intended work. The
session's first task is to fix the pre-existing failure, recorded as such in
`<project>/SESSIONS.md`. A session built on a broken baseline compounds problems.

If `cargo check` fails with compile errors, read the errors before anything else.
They may indicate that the last session ended in an incomplete state.

### Step 2 — Clean up prior session artifacts

If `REFACTOR-EVIDENCE.md` exists from a previous session, delete it.
It will be recreated during this session's refactoring.

### Step 3 — Capture the Prism baseline

After the baseline is confirmed clean and prior artifacts cleaned up, capture the Prism metrics snapshot:

```bash
livery/bin/prism stats . --json > /tmp/prism-session-before.json
```

This is a silent step — record the numbers for the delta at session end but do not
report them to the human unless asked. If `livery/bin/prism` is not executable, note
`[PRISM: manual — baseline not captured]` and continue. The session-end validation
(see `livery/skills/run-validation.md`) will need the human to provide Prism data
manually.

### Step 4 — Read the session context

In order:

1. **Read the last 2–3 entries in `<project>/SESSIONS.md`.** Understand:
   - What was built or changed
   - What decisions were made and why
   - What was explicitly deferred (do not implement deferred items without a scope change)
   - What the Prism baseline was at the end of the last session

2. **Read the session scope statement** (provided in the session prompt). Confirm you
   understand exactly what this session will and will not do. If the scope is ambiguous,
   resolve the ambiguity before writing code — not after.

3. **Read the relevant section of `<project>/ARCHITECTURE.md`** for the crate(s) being worked on.
   Confirm your understanding of the public API contract and the dependency boundaries.

### Step 5 — Confirm scope boundaries explicitly

State the scope of this session in one sentence. Then state what is explicitly out of
scope. If you cannot do this, the session prompt is ambiguous — seek clarification.

Example:
> **In scope:** Implement `mint-meta::parse_citation` and `mint-meta::serialise_citation`
> with unit tests and roundtrip property tests.
>
> **Out of scope:** The CodeMeta and BibTeX transformation functions (next session).
> Any changes to `mint-config` or `mint-cli`.

### Step 6 — Load the relevant reference documents

Based on the session scope, identify which files to have ready:

- Working on a library crate? Load `<project>/ARCHITECTURE.md` §Public API Stubs for that crate.
- Designing a new type or module? Load `livery/standards/ousterhout.md`.
- Writing any Rust code? Load `livery/standards/rust-specifics.md`.
- Writing names or comments? Load `livery/standards/readable-code.md`.
- Following a procedural task? Load the relevant skill file.

Do not load everything — load what is relevant to this session's scope. Prompt dilution
is real: loading ten files when two are relevant reduces the effective weight of all of
them.

**The naming-as-design-signal rule — active throughout every session:**
At any point during a session, if you find yourself unable to name a function, module,
or type clearly without using "and" or a long qualifying clause, treat this as a design
signal, not a naming problem. Stop. Ask: does this thing have more than one
responsibility? Consult `livery/standards/ousterhout.md` — the "hard to describe" Red Flag
and the single-responsibility principle. ARC and Ousterhout converge here: if you
cannot name it, it probably needs to be split. Resolve the design question before
choosing a name.

### Step 7 — Constitution check

Before writing any code, state back the three rules from the loaded standards files
that are most relevant to this session's scope. For example, if the session involves
designing a new module, cite the specific Ousterhout principle that applies. If the
session involves writing a parser, cite the property test mandate for transformation
functions.

If you cannot state three specific, relevant rules, you did not load the constitution
properly. Go back and re-read `CLAUDE-base.md` and the relevant standards files.

This step produces no artifact — it is a self-check. Record the three rules in the
session process log as a `CONSTITUTION_CHECK` entry.

### Step 8 — Create the process log

Create the session process log file before any other action:

```
process/SESSION-NNN.md
```

See CLAUDE-base.md §Session Process Log for the format. The first entries will be
`READ` entries for each file loaded in Steps 4 and 6.

### Step 9 — Record the session opening in `<project>/SESSIONS.md`

Add the session header before writing any code:

```markdown
## Session [N] — [YYYY-MM-DD]

**Scope:** [One sentence — what this session builds or fixes]

**Out of scope:** [What is explicitly deferred or excluded]

**Entry state:**
- Baseline: [cargo test --workspace result: X tests passing]
- Prism: [key metrics from /tmp/prism-session-before.json, or "same as Session N-1"]
- Open items from last session: [none / list]

**Reference documents loaded:** [list]
```

The session is now open. Begin the TDD cycle.

---

## Session Prompt Template

When opening a session with an agent, use this template in the prompt. Do not deviate.

```
## Session [N] — [Date]

**Scope:** [One sentence. What is being built or fixed.]

**Reference:** SPEC.md §[relevant section] / ARCHITECTURE.md §[relevant crate]

**Task:**
[Specific implementation task. Reference the public API stub from ARCHITECTURE.md.
 Reference a skill if applicable: "Follow the `add-subcommand` skill."]

**Stopping condition:** This session is complete when:
- [ ] [Specific deliverable 1]
- [ ] [Specific deliverable 2]
- [ ] `cargo test --workspace` passes
- [ ] `<project>/scripts/validate.sh` exits 0
- [ ] `review-for-red-flags` skill completed
- [ ] `review-docs` skill completed
- [ ] `<project>/SESSIONS.md` entry written

**Hard constraints:**
- Follow `livery/CLAUDE-base.md` and `<project>/CLAUDE.md` in full.
- Do not implement anything outside this session's scope.
- Do not modify public APIs without updating ARCHITECTURE.md.
- Do not refactor code outside the files this task touches unless required
  to enable the task (document why in `<project>/SESSIONS.md` if you do).
```