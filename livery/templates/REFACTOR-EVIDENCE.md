# REFACTOR-EVIDENCE.md

Session: _[session name]_
Date: _[date]_

---

## Pass 1 — Ousterhout (structure)

**Standards loaded:** `livery/standards/ousterhout.md`

**Modules examined:**
- _[list each new or modified module]_

**Design Process Checklist (Part III) findings:**
- _[Can any module be deepened?]_
- _[Can any abstraction be removed?]_
- _[Does any function do more than one thing?]_

**Red Flags found and fixed:**
- _[list each red flag, or "None found" with brief justification]_

**Changes made:**
- _[describe structural changes, or "No changes — [reason]"]_

---

## Pass 2 — ARC names (surface)

**Standards loaded:** `livery/standards/readable-code.md` Part I

**Names reviewed:**
- _[list each name written or modified during Green phase]_

**Checks applied:**
- [ ] Misunderstanding test on every name
- [ ] Placeholder names replaced
- [ ] Banned-words check passed
- [ ] Scope-proportionality applied
- [ ] Boolean-naming rule applied

**Renames performed:**
- _[old_name -> new_name: reason, or "None needed"]_

**Design signals from naming difficulty:**
- _[any functions that were hard to name without "and" — returned to Pass 1? or "None"]_

---

## Pass 3 — ARC expression (surface)

**Standards loaded:** `livery/standards/readable-code.md` Parts II–IV

**Comments reviewed:**
- [ ] Stripped comments that restate the code
- [ ] Added comments for non-obvious decisions made during Green

**Control flow improvements:**
- [ ] Guard clauses applied (nested conditions inverted, happy path at lowest indentation)
- [ ] Complex boolean conditions extracted into named explaining variables

**Single-responsibility check:**
- _[every function does one thing at one level of abstraction? list any fixes]_

**Changes made:**
- _[describe expression-level changes, or "No changes — [reason]"]_
