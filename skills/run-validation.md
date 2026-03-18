# Skill: run-validation

> **When to invoke:** At the end of every session, as the final step before writing
> the `<project>/SESSIONS.md` entry. Also run after any significant change mid-session to catch
> regressions early.

---

## The Validation Pipeline

Run every command in this sequence. Each must pass before running the next. Do not
skip steps. Do not declare a session complete if any step fails.

### Step 1 — Formatting

```bash
cargo fmt --check
```

If this fails: run `cargo fmt` to auto-fix, then re-run `cargo fmt --check` to confirm.
Review the diff — formatting changes should not obscure logic changes.

### Step 2 — Linting

```bash
cargo clippy --workspace -- -D warnings
```

If this fails: fix every warning. Do not use `#[allow(...)]` to suppress a warning
unless you can state in an inline comment exactly why it is a false positive in this
specific location. Suppressing a warning to make this step pass is a red flag — stop
and understand the lint.

### Step 3 — Tests

```bash
cargo test --workspace
```

If this fails: do not proceed. A failing test means the session's work is not complete.
Fix the failure, which may require revisiting implementation from this session.

Run with `-- --nocapture` if a test failure message is insufficient:
```bash
cargo test --workspace -- --nocapture
```

### Step 4 — Prism quality gate

```bash
livery/bin/prism check . --strict --fix-suggestions
```

If this fails: read the fix suggestions. Address each failure. Do not disable thresholds
in `<project>/prism.toml` to make this pass — raise the quality of the code instead.

If `livery/bin/prism` is not executable (missing binary, wrong architecture, permissions),
report the failure to the human with the exact error and list the command for manual
execution. Leave `[PRISM: manual]` in the SESSIONS.md entry. Do not skip the gate.

### Step 5 — Prism baseline delta

```bash
livery/bin/prism stats . --json > /tmp/prism-session-after.json
```

Compare against the session-start baseline (captured at the beginning of the session
per CLAUDE-base.md §Automated Quality Gate Protocol, stored at
`/tmp/prism-session-before.json`). Any metric that worsened requires a note in the
`<project>/SESSIONS.md` entry explaining why — or it must be fixed.

Key metrics to track:
- Test count and coverage (should increase or hold)
- Module depth ratios (should not decrease)
- Function complexity (should not increase)
- Doc coverage (should not decrease)
- Functions over 50 lines (should not increase)

### Step 6 — Architecture consistency check

If any public API, crate structure, or inter-crate dependency changed this session:

```bash
livery/bin/prism map . --mermaid
```

Compare the output against the Dependency Graph section of `<project>/ARCHITECTURE.md`. If they
diverge, update `<project>/ARCHITECTURE.md` to reflect the current state and record the change
as a session decision.

### Step 7 — Session-end pattern check

Scan the last 3–5 SESSIONS.md entries (including the one about to be written). Look
for recurring patterns: same metric drifting the same direction, same red flag finding,
same kind of naming issue, same deferred item accumulating. If a recurrence is found,
append or update an entry in `livery/feedback/known-patterns.md`. See
`livery/feedback/feedback-loop.md` §3.1 for the full protocol.

---

## Quick Reference (All Steps)

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
livery/bin/prism check . --strict --fix-suggestions
livery/bin/prism stats . --json > /tmp/prism-session-after.json
```

All five must exit 0 before the session is complete.

---

## When a Step Cannot Pass This Session

Occasionally a validation step will fail due to an issue that genuinely requires more
than the current session to fix (e.g., a Prism threshold that would require a large
refactor). In this case:

1. Do not suppress the failure
2. Record it explicitly in `<project>/SESSIONS.md` as a known failing gate with a description
   of what is needed to fix it
3. Create a follow-up session scope to address it
4. The next session opens with this as its primary task

Carrying a known failing gate forward is acceptable for one session. Carrying it
across two sessions is a design debt emergency.