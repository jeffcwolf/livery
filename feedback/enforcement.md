# enforcement.md — Rule Enforcement Register

> **What this document is:** A register tracking which Livery rules have
> been escalated from their default enforcement level based on observed
> non-compliance. The register is maintained by the human based on
> evidence from the feedback loop.
>
> **How it works:** Every rule starts at Level 1 (constitutional prose).
> When a rule is observed to be inconsistently followed, the response is
> not to make the prose stronger — it is to escalate the rule to a higher
> enforcement level. The register records what was escalated, why, and
> what the escalation looks like in practice.

---

## Enforcement Levels

### Level 1: Constitutional Prose

The rule is written in CLAUDE-base.md or a standards file. Read at
session start. Relies on the agent remembering and choosing to comply.
This is the default for all rules. Most rules stay here permanently.

**Strength:** zero overhead.
**Weakness:** the rule competes with everything else in context. As
the session progresses, early instructions lose salience.

### Level 2: Skill with Trigger

The rule is extracted into a SKILL.md that activates at a specific
moment — during refactoring, when writing a test, when adding a public
function. Stronger than prose because the rule is re-encountered at
the point of action.

**Strength:** the rule appears when it matters, not just at session start.
**Weakness:** requires writing and maintaining a skill. Overhead is
justified only for rules that are important and frequently missed.

### Level 3: Verification Checkpoint

The rule is added to an explicit checklist that the agent must confirm
at a gate point (session end, before committing, before branch finishing).
The agent doesn't just follow the rule — it must assert that it followed
the rule and provide evidence.

**Strength:** the agent cannot passively skip the rule. It must
actively confirm compliance.
**Weakness:** adds time to every checkpoint. Too many Level 3 rules
make checkpoints tedious and invite rubber-stamping.

### Level 4: Automated Gate

The rule is checked mechanically by a tool, and failure blocks session
completion. Examples: Prism thresholds, `cargo clippy`, `cargo fmt`,
`RUSTDOCFLAGS="-D missing_docs"`.

**Strength:** unforgeable. The agent cannot game it or forget it.
**Weakness:** not every rule can be automated. "Is this module deep?"
cannot be checked by a tool. Only rules with clear mechanical criteria
reach Level 4.

---

## Escalation Protocol

1. A pattern of non-compliance is recorded in `known-patterns.md`.
2. A proposal is written (see `feedback-loop.md`) that recommends
   escalation to a specific level and describes what the escalation
   looks like in practice.
3. The human approves, rejects, or modifies the proposal.
4. If approved, the escalation is implemented (skill written, checkpoint
   added, or tool configured) and the register below is updated.
5. After 5+ sessions at the new level, review whether the escalation
   is working. If the pattern has stopped, the escalation stays. If
   not, consider escalating further or revisiting whether the rule
   is well-defined.

De-escalation is also possible. If a rule was escalated due to a
specific context (unfamiliar domain, early project phase) and the
pattern has not recurred for 10+ sessions, the human may de-escalate
it. Record the de-escalation in the register with a rationale.

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