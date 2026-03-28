# hard-rules-block.md — Insert into CLAUDE.md template after MANDATORY PRE-FLIGHT
#
# This block is the Level 3/4 enforcement of rules that were previously
# Level 1 prose in CLAUDE-base.md. These rules are inlined in every
# project's CLAUDE.md so they survive context decay.

## HARD RULES (these override all other instincts — violation = session invalid)

**1. TDD is literal, not conceptual.**
Write a failing test. Run it. Watch it fail. Confirm it fails for the
right reason (missing functionality, not a compile error). Only then
write the minimum implementation to pass. Run the test. Watch it pass.
Never write implementation before a failing test exists. Never.

**2. Three-pass refactor is mandatory after every green.**
After tests pass, complete all three passes before committing:
- *Pass 1 — Ousterhout:* Load `livery/standards/ousterhout.md`. Run the
  Design Process Checklist. Fix every Red Flag.
- *Pass 2 — ARC names:* Load `livery/standards/readable-code.md` Part I.
  Misunderstanding test on every new name. Replace every placeholder.
- *Pass 3 — ARC expression:* Load `livery/standards/readable-code.md`
  Parts II–IV. Strip restating comments. Add decision comments.
  Guard clauses. One function, one thing.
Write findings to `REFACTOR-EVIDENCE.md` (checked by `livery/bin/refactor-check`).

**3. No `.unwrap()` in library code without `// SAFETY:` comment.**
Use `.expect("reason")` or return `Result`. In tests, `.unwrap()` is fine.
Checked by `livery/bin/lint-rules`.

**4. No `assert!(result.is_ok())` or `assert!(result.is_err())`.**
Unwrap or pattern-match so failures are informative. Weak assertions hide
the actual error. Checked by `livery/bin/lint-rules`.

**5. No `#[allow(dead_code)]` in non-test code.**
Investigate the caller chain. If genuinely unused, remove the item.
If used, fix the visibility. Suppressing the warning masks wiring errors.
Checked by `livery/bin/lint-rules`.

**6. Every `pub` item has a doc comment. No exceptions.**
Checked by `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` in
`livery/bin/lint-rules`.

**7. Scope is frozen at session start.**
If it is not listed in the session scope, do not touch it. Adjacent
improvements are scope creep. Record them in SESSIONS.md for a future
session.

**8. Commit messages follow the structured format.**
Type prefix, SESSION, WHY, WHAT, TRAPS fields. Single-line messages
are not acceptable. Checked by `livery/bin/commit-check`.

**9. Session does not close until all gates pass.**
Run `scripts/validate.sh` and confirm exit 0. No "mostly passing."
No "I'll fix this next session." Green or not done.
