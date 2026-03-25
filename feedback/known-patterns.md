# known-patterns.md — Recurring Patterns Observed Across Sessions

---

## Pattern — Open-ended structural prompts produce suppression instead of design

**First observed:** Session 22, Scribe
**Recurrences:** —
**Category:** review finding
**Data:**
- Session 22 `pub_ratio` work: prompt said "reduce unnecessary pub items." Agent added `#[allow(dead_code)]` to 23 items rather than investigating actual usage.
- The same session produced `pub` instead of `pub(crate)` on items with only intra-crate callers, because `pub` compiles without friction.
- Mechanical prompt ("for each `pub` item, find all call sites; if all callers are within the crate, change to `pub(crate)`") eliminated the problem in a follow-up session.

**Status:** observing

---

## Pattern — Sessions crossing two structural concerns produce compounding errors

**First observed:** Session 22, Scribe
**Recurrences:** —
**Category:** review finding
**Data:**
- Sessions that changed both visibility and module structure simultaneously introduced ambiguity in test failures: unclear whether a failure was caused by the visibility change, the module restructuring, or their interaction.
- Debugging time in combined-concern sessions exceeded the sum of what two single-concern sessions would have required.
- Single-concern sessions produced clean diffs that were straightforward to review and revert if needed.

**Status:** observing

---

## Pattern — Deferred Prism gate activation allows quality drift in completed subsystems

**First observed:** Session 22, Scribe
**Recurrences:** —
**Category:** metric drift
**Data:**
- Subsystems (crates) that completed structural conversion early accumulated new violations while conversion continued in other crates.
- Whole-workspace `prism check --strict` at conversion end surfaced violations that had been introduced after the subsystem was nominally complete.
- Per-crate gate activation (`prism check <crate-path> --strict`) immediately after subsystem completion would have caught these regressions at introduction time.

**Status:** observing

---

## Pattern — #[allow(dead_code)] added during structural conversion masks wiring errors

**First observed:** Session 22, Scribe
**Recurrences:** —
**Category:** review finding
**Data:**
- During visibility and module restructuring, items that appeared unused after a move were annotated with `#[allow(dead_code)]` rather than investigated.
- In multiple cases the item was not genuinely dead — its caller had been moved or renamed earlier in the same session, and the allow annotation suppressed a real wiring error.
- Treating `#[allow(dead_code)]` as a session-end blocker (no new annotations that were not present at session start) forced investigation and caught two wiring bugs that would otherwise have shipped.

**Status:** observing

---

## Pattern — Mechanical per-item prompts produce higher-quality structural changes than batch prompts

**First observed:** Session 22, Scribe
**Recurrences:** —
**Category:** positive signal
**Data:**
- Prompts structured as "for each X, do Y" (where Y is a specific, mechanical check) consistently produced correct results across all items.
- Batch prompts ("fix all visibility issues in this crate") produced inconsistent results: some items correctly changed, others suppressed, others missed entirely.
- The per-item pattern aligns with the session scope discipline: each item is a discrete, verifiable unit of work.

**Status:** observing
