# enforcement.md — Rule Enforcement Register (updated)
#
# REPLACEMENT for the Register section and Rules That Start Above Level 1
# section in feedback/enforcement.md. The preamble, levels, and escalation
# protocol sections remain unchanged.

---

## Register

| Rule | Default | Current | Escalated because | Date |
|---|---|---|---|---|
| TDD: test before code | 1 | 4 | Agent observed writing implementation before tests across multiple sessions and projects. Escalated to `tdd-audit` script that checks git history for test-before-impl evidence. | 2026-03-28 |
| Three-pass refactoring | 1 | 3 | ARC passes skipped under time pressure — agent proceeds directly from green to commit without running Ousterhout, naming, or expression passes. Escalated to `refactor-check` script requiring REFACTOR-EVIDENCE.md. | 2026-03-28 |
| No .unwrap() in lib code | 1 | 4 | Prose rule in rust-specifics.md consistently ignored. Escalated to grep-based check in `lint-rules` script. | 2026-03-28 |
| No assert!(x.is_ok()) | 1 | 4 | Weak assertions hide actual error values and make test failures uninformative. Rule stated in CLAUDE-base.md Testing Standards but not mechanically enforced. Escalated to `lint-rules`. | 2026-03-28 |
| No #[allow(dead_code)] | 1 | 4 | Observed pattern in Scribe conversion (known-patterns.md): `#[allow(dead_code)]` added to mask wiring errors during structural changes. Escalated to `lint-rules`. | 2026-03-28 |
| Doc comments on pub items | 1 | 4 | Repeatedly missing despite prose rule. Escalated to RUSTDOCFLAGS gate in `lint-rules` and `validate.sh`. | 2026-03-28 |
| Scope frozen at session start | 1 | 2 | Agent observed making "adjacent improvements" outside session scope, introducing untested cross-crate changes. Escalated to explicit HARD RULES block in CLAUDE.md (re-encountered at session start). | 2026-03-28 |
| Commit message format | 1 | 4 | Single-line commit messages observed despite format being specified in every CLAUDE.md. Escalated to `commit-check` script. | 2026-03-28 |
| HARD RULES inlined in CLAUDE.md | — | 3 | CLAUDE-base.md rules lose salience as context fills. Critical rules now duplicated in project CLAUDE.md (which is auto-loaded) to survive context decay. | 2026-03-28 |

---

## Rules That Start Above Level 1

Some rules are already enforced above Level 1 in the base system.
These are not escalations — they were designed at a higher level from
the start.

| Rule | Level | Mechanism |
|---|---|---|
| Code formatting | 4 | `cargo fmt --check` |
| Lint compliance | 4 | `cargo clippy -- -D warnings` |
| Doc coverage threshold | 4 | `prism check . --strict` (prism.toml `min_doc_coverage`) |
| Cyclomatic complexity ceiling | 4 | `prism check . --strict` (prism.toml `max_cyclomatic`) |
| Cognitive complexity ceiling | 4 | `prism check . --strict` (prism.toml `max_cognitive`) |
| Test ratio minimum | 4 | `prism check . --strict` (prism.toml `min_test_ratio`) |
| No unsafe blocks | 4 | `prism check . --strict` (prism.toml `max_unsafe_blocks`) |
| Red Flag Audit | 3 | Session-end checkpoint in session contract |
| Naming Review | 3 | Session-end checkpoint in session contract |
| Documentation Review | 3 | Session-end checkpoint in session contract |

These form the baseline. Escalations in the register above are for
rules that started at Level 1 and were promoted based on evidence.

---

*Register created 2026-03-18. First escalation batch: 2026-03-28.*
