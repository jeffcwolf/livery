# livery-update-plan.md — Complete update instructions
#
# This document describes all changes needed to implement the
# enforcement escalation. Use Claude Code to apply these changes
# to the livery repository.

---

## 1. New files to add

### livery/bin/tdd-audit
Copy from: livery-updates/bin/tdd-audit
Make executable: chmod +x livery/bin/tdd-audit

### livery/bin/refactor-check
Copy from: livery-updates/bin/refactor-check
Make executable: chmod +x livery/bin/refactor-check

### livery/bin/lint-rules
Copy from: livery-updates/bin/lint-rules
Make executable: chmod +x livery/bin/lint-rules

### livery/bin/commit-check
Copy from: livery-updates/bin/commit-check
Make executable: chmod +x livery/bin/commit-check

### livery/templates/REFACTOR-EVIDENCE.md
Copy from: livery-updates/REFACTOR-EVIDENCE-template.md
(Agent copies this to project root at session start if it doesn't exist)

---

## 2. Update: livery/bin/scaffold

### Change A: Insert HARD RULES block into CLAUDE.md heredoc

In the scaffold script, find the CLAUDE_MD heredoc. After the line:

    > **Extends:** \`livery/CLAUDE-base.md\` ...

And before the `### Commit Message Format` section, insert the full
HARD RULES block from livery-updates/hard-rules-block.md.

The HARD RULES must appear BEFORE commit message format, BEFORE crate
responsibilities, BEFORE reference documents. This ensures they are
the first substantive content the agent reads after pre-flight.

### Change B: Replace validate.sh heredoc

Replace the entire VALIDATE_SH heredoc in the scaffold script with
the content from livery-updates/validate-template.sh.

The new validate.sh adds gates 5-9:
  5. livery/bin/lint-rules
  6. Prism (unchanged)
  7. livery/bin/tdd-audit
  8. livery/bin/refactor-check
  9. livery/bin/commit-check

### Change C: Update scaffold summary output

In the "Phase 2 complete!" summary section, add a line after the
existing "Next steps" block:

    echo "  New enforcement gates:"
    echo "    livery/bin/tdd-audit       Verify test-before-impl in git history"
    echo "    livery/bin/lint-rules      Livery-specific anti-pattern checks"
    echo "    livery/bin/refactor-check  Verify three-pass refactoring evidence"
    echo "    livery/bin/commit-check    Verify structured commit messages"

---

## 3. Update: livery/feedback/enforcement.md

Replace the Register section (from `## Register` to the end of
`## Rules That Start Above Level 1`) with the content from
livery-updates/enforcement-register-update.md.

Keep the preamble, enforcement levels, and escalation protocol
sections unchanged.

---

## 4. Update: livery/CLAUDE-base.md

### Change A: Add gate scripts to the session contract

Find the section that defines the session contract commands. It
currently says:

    cargo test --workspace
    cargo fmt --check
    cargo clippy --workspace -- -D warnings
    scripts/validate.sh

The validate.sh now includes the new gates, so no change is needed
to the command list itself. But add a paragraph after the session
contract block:

    **Gate scripts.** The validation pipeline includes automated checks
    beyond cargo tooling. These are Level 4 (automated gate) enforcement
    of rules that were previously Level 1 (prose). The gate scripts are:

    | Script | What it checks | Blocking? |
    |---|---|---|
    | `livery/bin/lint-rules` | .unwrap() without SAFETY, weak assertions, #[allow(dead_code)], missing docs, vague test names | Yes (exit 2 = fail) |
    | `livery/bin/tdd-audit` | Git history shows test files modified before/with impl files | Yes if >50% violations |
    | `livery/bin/refactor-check` | REFACTOR-EVIDENCE.md exists with three passes documented | Yes if file missing |
    | `livery/bin/commit-check` | Commit messages follow structured format | Yes if >50% violations |

### Change B: Add REFACTOR-EVIDENCE.md to the TDD workflow

In the TDD section, after the three-pass refactor description and
before "Never skip red", add:

    **Document the refactoring.** After completing all three passes, write
    (or update) `REFACTOR-EVIDENCE.md` in the project root. Document what
    each pass examined and what changed. This file is checked by
    `livery/bin/refactor-check` at session end. A template is available at
    `livery/templates/REFACTOR-EVIDENCE.md`.

### Change C: Reference HARD RULES in the session opening

In any section that discusses session structure or the session-open
skill, add a note:

    The project CLAUDE.md contains a HARD RULES section that inlines the
    most critical rules from this constitution. These rules are duplicated
    deliberately — they survive context decay because the project CLAUDE.md
    is auto-loaded at session start, while CLAUDE-base.md is only loaded
    if the agent follows the pre-flight instructions.

---

## 5. Update: livery/skills/session-open.md

Add a step after baseline verification:

    5. If REFACTOR-EVIDENCE.md exists from a previous session, delete it.
       It will be recreated during this session's refactoring.

---

## 6. Update: livery/README.md

Add the new scripts to the file listing:

    bin/                  Pre-compiled tooling and gate scripts.
      prism               Automated quality gate binary.
      scaffold            Project scaffold generator.
      tdd-audit           Verify TDD discipline from git history.
      lint-rules          Livery-specific anti-pattern checks.
      refactor-check      Verify three-pass refactoring evidence.
      commit-check        Verify structured commit messages.
      README.md           Build and rebuild instructions.

---

## 7. Update: livery/docs/INDEX.md

Add to the Tooling section:

    | `livery/bin/tdd-audit` | Checks git history for test-before-implementation evidence. Level 4 gate. | Session-end validation |
    | `livery/bin/lint-rules` | Grep-based checks for .unwrap(), weak assertions, #[allow(dead_code)], missing docs, vague test names. Level 4 gate. | Session-end validation |
    | `livery/bin/refactor-check` | Checks that REFACTOR-EVIDENCE.md exists with three passes documented. Level 3 gate. | Session-end validation |
    | `livery/bin/commit-check` | Checks commit messages follow structured format. Level 4 gate. | Session-end validation |

---

## 8. Update existing projects

For each existing project (orb, prism, quire):

1. Add the HARD RULES block to the project's CLAUDE.md
   (after pre-flight, before commit format)

2. Replace scripts/validate.sh with the new template
   (adjust project-specific extensions if any)

3. Commit: "chore: escalate enforcement — add HARD RULES and gate scripts"

---

## 9. Update: livery/QUICKSTART.md

In the session prompt template, add to the Hard constraints section:

    - Create REFACTOR-EVIDENCE.md during refactoring (three passes).
    - Run `scripts/validate.sh` at session end — all 9 gates must pass.

---

## Summary of enforcement changes

| Rule | Was | Now | Mechanism |
|---|---|---|---|
| TDD: test before code | Level 1 (prose in CLAUDE-base.md) | Level 4 (script) | `livery/bin/tdd-audit` in validate.sh |
| Three-pass refactoring | Level 1 (prose in CLAUDE-base.md) | Level 3 (evidence) | `livery/bin/refactor-check` + REFACTOR-EVIDENCE.md |
| No .unwrap() in lib code | Level 1 (prose in rust-specifics.md) | Level 4 (script) | `livery/bin/lint-rules` check 1 |
| No weak assertions | Level 1 (prose in CLAUDE-base.md) | Level 4 (script) | `livery/bin/lint-rules` check 2 |
| No #[allow(dead_code)] | Observation in known-patterns.md | Level 4 (script) | `livery/bin/lint-rules` check 3 |
| Doc comments on pub items | Level 4 (Prism) but inconsistent | Level 4 (script + Prism) | `livery/bin/lint-rules` check 4 + RUSTDOCFLAGS |
| Scope frozen | Level 1 (prose) | Level 2 (HARD RULES block) | Inlined in CLAUDE.md, re-encountered at session start |
| Commit message format | Level 1 (prose in CLAUDE.md) | Level 4 (script) | `livery/bin/commit-check` in validate.sh |
| Critical rules survive context decay | Level 1 (reference to CLAUDE-base.md) | Level 3 (inlined) | HARD RULES block in every project CLAUDE.md |
