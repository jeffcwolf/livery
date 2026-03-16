# Skill: review-for-red-flags

> **When to invoke:** At the end of every session, as part of the session contract.
> Also invoke before declaring any new module design complete.
> **Load:** `livery/livery/standards/ousterhout.md`

---

## Procedure

Run this procedure against every module, struct, trait, and public function created or
modified in the current session. Work through each step in order. Do not skip steps.

### Step 1 — Identify scope

List every file touched this session. For each file, identify:
- New public functions or types added
- Existing public functions or types modified
- New internal modules created

### Step 2 — Run the Design Process Checklist

From `livery/livery/standards/ousterhout.md` Part III, apply every question to each item in scope.

**Before-implementation questions** (apply to new modules/types designed this session):
- Can you state in one sentence what complexity this module hides?
- Were at least two alternative designs considered? (Design it twice)
- Is every public item truly public — does it have at least two callers?
- Are there error conditions that could be eliminated by type constraints?

**After-implementation questions** (apply to all code written this session):
- Does any public function leak internal types or structures?
- Does every function do one thing, describable without "and"?
- Is there any repeated logic that belongs in a new method?
- Does any module have temporal decomposition — sequential stages that share data and
  should be a single module?
- Are there any pass-through methods?
- Are there any conjoined methods that require a call sequence?

**Documentation questions:**
- Do all doc comments describe contracts, not implementations?
- Does every public item have a doc comment that adds information beyond the name?
- Are non-obvious decisions explained with comments that say *why*, not *what*?

### Step 3 — Check every Red Flag explicitly

For each Red Flag in `livery/livery/standards/ousterhout.md` Part II, scan the session's code and
confirm it is absent. State the finding for each:

- Shallow Module — present / absent
- Information Leakage — present / absent
- Temporal Decomposition — present / absent
- Overexposure — present / absent
- Pass-Through Methods — present / absent
- Repetition — present / absent
- Special-General Mixture — present / absent
- Conjoined Methods — present / absent
- Comment Repeats Code — present / absent
- Implementation Documentation Contaminates Interface — present / absent
- Vague Name — present / absent
- Hard to Describe — present / absent
- Nonobvious Code — present / absent

### Step 4 — Resolve findings

For each Red Flag found to be present:
1. State which file and function contains it
2. State which Red Flag it triggers and why
3. Fix it before the session ends
4. If the fix requires a structural change large enough to be its own session, record it
   as a design debt item in `<project>/SESSIONS.md` with a clear description of what needs to change

### Step 5 — Record in session log

In the `<project>/SESSIONS.md` entry for this session, add a "Red Flag Audit" section:
```
**Red Flag Audit:**
- Ran review-for-red-flags against: [list files]
- Findings: [none / list of findings and resolutions]
- Design debt recorded: [none / list of deferred items]
```
