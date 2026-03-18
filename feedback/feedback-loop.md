# feedback-loop.md — How Livery Improves Over Time

> **What this document is:** The protocol for Livery's self-correction
> mechanism. It defines how data from sessions is analyzed, how patterns
> are recorded, how proposals for change are generated, and how the human
> decides what gets promoted into the constitution.
>
> **What this document is not:** A machine learning system, a training
> loop, or an autonomous self-modification protocol. The agent observes
> and proposes. The human decides.

---

## 1. Principle

Livery's design philosophy (Ousterhout's principles, TDD, the quality
gate concept) is stable. It changes only when the human deliberately
evolves it. What improves over time is *the system's skill at following
its own principles* — sharper review skills, more precise anti-patterns,
better-calibrated thresholds, accumulated knowledge from real projects.

Improvement moves knowledge upward through four layers:

```
Axioms            Ousterhout's principles, TDD, the runtime interface.
                  Change requires deliberate human decision with
                  strong justification. Never changed by proposal.

Constitution      CLAUDE-base.md specifics, prism.toml thresholds,
                  review skill criteria, documentation and naming
                  standards. Changes via human-approved proposals
                  after project milestones.

Project rules     Project CLAUDE.md anti-patterns, project-specific
                  skills. Change within a project based on
                  session-level feedback. No approval needed beyond
                  the session log.

Observations      Session findings, Prism deltas, mutation survivors,
                  known patterns. Raw signal. Append-only during a
                  project. Feeds into the layers above.
```

The agent may propose changes to the Constitution and Project rules
layers. The agent never proposes changes to the Axioms layer. The
human approves or rejects all Constitution-level proposals.

---

## 2. File Structure

```
livery/
  known-patterns.md          — recurring patterns observed across sessions
  proposals/                 — proposed changes awaiting human review
    001-description.md
    002-description.md
    ...
```

### known-patterns.md

A neutral, append-only log of patterns detected across sessions. Patterns
are neither good nor bad — they are observations. A pattern that property
tests consistently catch real bugs is recorded alongside a pattern that
functions keep exceeding 50 lines. The judgment belongs in the proposal
that acts on the pattern, not in the pattern entry itself.

Entry format:

```markdown
## Pattern — [short description]

**First observed:** Session [N], [project name]
**Recurrences:** Sessions [M, P, ...], [project(s)]
**Category:** [metric drift | review finding | mutation gap | positive signal | other]
**Data:**
- [specific numbers, findings, or observations from each occurrence]

**Status:** [observing | proposal written (NNN) | resolved]
```

The agent appends new patterns and updates recurrences. The agent never
removes or edits past entries. If a pattern is resolved (a proposal was
accepted and the pattern stops recurring), the status is updated but
the entry remains.

### proposals/

Each file is a discrete, numbered proposal for a change that crosses
the Constitution boundary. Proposals are the mechanism by which the
system improves — they carry specific evidence and a specific change.

---

## 3. Triggers

Three events trigger analysis. Each is lightweight and uses data that
already exists.

### 3.1 Session-End Pattern Check

**When:** After writing the SESSIONS.md entry, before branch finishing.

**What the agent does:**

1. Read the last 3–5 SESSIONS.md entries (including the one just written).
2. Look for recurrences:
   - Same Prism metric moving in the same direction across 2+ sessions
     (e.g., `fns_over_50_lines` increasing, `test_ratio` decreasing)
   - Same red flag finding appearing in 2+ sessions
   - Same naming issue or doc quality finding recurring
   - Same kind of deferred item accumulating without resolution
   - Same positive outcome reliably appearing (e.g., property tests
     catching real bugs, a particular module design consistently
     scoring well)
3. If a recurrence is found, append or update an entry in
   `known-patterns.md`.
4. No proposal is written yet. This is observation, not action.

**Time cost:** 1–2 minutes. Happens every session.

### 3.2 Milestone Retrospective

**When:** Every 5 sessions, or at project milestones (v1.0, major
feature complete), or when the human explicitly requests one.

**What the agent does:**

1. Read the full SESSIONS.md for the project.
2. Read `known-patterns.md`.
3. Read the Prism delta history across all sessions — identify trends,
   not just individual data points.
4. Read any surviving mutation reports from the most recent cargo-mutants
   run.
5. Produce a retrospective analysis answering:
   - Which Prism metrics improved consistently? Which degraded?
   - Which known patterns are resolved? Which are worsening?
   - Were there sessions where quality regressed — and what was
     different about those sessions (scope too large? unfamiliar
     domain? missing skill?)
   - Are the review skills catching what Prism later flags, or are
     there blind spots?
   - Are the prism.toml thresholds well-calibrated — too lenient
     (everything passes easily) or too strict (constant friction
     without quality gain)?
6. For each actionable finding, write a proposal in `proposals/`.

**Time cost:** 15–30 minutes. Happens infrequently.

### 3.3 Mutation Testing Analysis

**When:** After a `cargo mutants` run, which happens at milestones or
when the human triggers it (not every session — it is expensive).

**What the agent does:**

1. Read the mutation report.
2. Categorize surviving mutants:
   - By module: are survivors concentrated in one area?
   - By mutation type: boundary conditions? Error paths? Format
     strings? Default returns?
   - By test type: are unit tests catching different mutants than
     property tests?
3. Check whether the survivors reveal a *systematic* gap — a class
   of test that is consistently missing, not just individual
   missed cases.
4. If a systematic gap is found, write a proposal targeting the
   testing standards or a specific review skill.
5. Update `known-patterns.md` with the mutation pattern if it
   recurs across runs.

**Time cost:** 10–15 minutes. Happens at milestones.

---

## 4. Proposals

### Format

```markdown
# Proposal NNN — [title]

**Triggered by:** [session-end pattern | milestone retrospective | mutation analysis]
**Date:** [YYYY-MM-DD]
**Project:** [project name, or "cross-project" for constitutional changes]
**Evidence:** Sessions [N, M, P]; known-patterns.md entry [description]
**Target:** [exact file and section to change]
**Layer:** [constitution | project rules]

## Observation

[What was observed, with specific data points. No interpretation yet.]

## Analysis

[Why this pattern matters. What it costs if left unaddressed. Whether
it is a gap in the standards, a gap in the skills, a miscalibrated
threshold, or something else.]

## Proposed Change

[Exact text to add, modify, or remove. Diffs are preferred over
descriptions. If the proposal modifies prism.toml, show the before
and after values.]

## Expected Effect

[What metric or behavior should improve. How will you know if the
change worked? What would you look for in the next 3–5 sessions?]

## Risks

[Could this change cause problems elsewhere? Could it make sessions
slower without quality gain? Could it create a new class of false
positives in review skills?]

## Status

- [ ] Pending human review
- [ ] Approved — merged on [date]
- [ ] Rejected — rationale: [reason]
- [ ] Deferred — revisit after [condition]
```

### Scope Rules

Proposals may target:

- **CLAUDE-base.md** — anti-patterns list, review skill criteria,
  documentation specifics, naming conventions, testing standards
  (but not the design philosophy or TDD requirement)
- **prism.toml** — threshold adjustments with strong data justification
- **Review skills** — adding checks, sharpening criteria, removing
  checks that produce only false positives
- **Project CLAUDE.md** — project-specific anti-patterns, boundary
  rules, skill references
- **New skills** — proposing a new review skill or project skill
  based on a recurring gap

Proposals may **not** target:

- Ousterhout's design principles (deep modules, information hiding,
  strategic programming, defining errors out of existence)
- The TDD requirement (red/green/refactor)
- The three-pass refactoring protocol (Ousterhout, ARC names,
  ARC expression)
- The requirement for property-based tests on data-transforming
  functions
- The requirement for session continuity (SESSIONS.md)
- The runtime interface definition

These are axioms. If the human wants to change them, that is a
deliberate philosophical decision made outside the proposal system.

### Human Review Protocol

The human reviews proposals when they appear — typically at milestone
retrospectives, not after every session. The review is a decision
with four outcomes:

1. **Approved.** The change is made to the target file. The proposal
   status is updated with the merge date.
2. **Rejected.** The proposal status is updated with a rationale.
   The rationale is important — it tells future agents why this
   change was considered and declined, preventing the same proposal
   from being regenerated.
3. **Deferred.** The proposal is kept open with a condition for
   revisiting (e.g., "revisit after 5 more sessions of data" or
   "revisit when project X reaches v1.0").
4. **Partially approved.** The human modifies the proposed change
   before merging. The proposal is updated to reflect what was
   actually merged.

---

## 5. Cross-Project Learning

When a pattern or proposal originates in one project but applies
universally, it targets CLAUDE-base.md (constitution layer). When it
is project-specific, it targets the project's CLAUDE.md.

The signal that a pattern is universal: it appears in 2+ projects, or
it addresses a gap that is not specific to any project's domain (e.g.,
"functions over 50 lines tend to have surviving mutants" is universal;
"CFF parsing edge cases need more property test strategies" is
project-specific).

After a project completes, the agent reviews the project's
`known-patterns.md` and proposes which project-level anti-patterns
should be promoted to CLAUDE-base.md. This is the mechanism by which
the constitution accumulates hard-won knowledge over time.

---

## 6. What Success Looks Like

The feedback loop is working if, over time:

- The same red flag findings stop appearing after a proposal addresses
  them.
- Prism metrics trend toward the thresholds without constant manual
  intervention.
- Mutation catch rates improve across projects (more mutants caught
  by the same test-writing standards).
- The review skills become more precise — fewer false positives,
  fewer missed findings that Prism later catches.
- The proposals/ directory shows a mix of approved, rejected, and
  deferred entries — evidence that the system is generating useful
  observations and the human is exercising judgment.
- New projects start faster because the constitution has absorbed
  lessons from prior projects.

The feedback loop is **not** working if:

- The same patterns recur across many sessions without proposals
  being written.
- Proposals are generated but never reviewed.
- The constitution grows so large that agents cannot hold it in
  context.
- Prism thresholds are relaxed to make gates easier to pass rather
  than to reflect genuine calibration.
- The axioms layer starts receiving proposals (the boundary is
  being ignored).

---

## 7. Bootstrapping

For existing Livery projects (like mint) that predate this protocol:

1. Create `livery/known-patterns.md` with an empty header.
2. Create `livery/proposals/` as an empty directory (add a `.gitkeep`).
3. Optionally, review the existing SESSIONS.md and seed
   `known-patterns.md` with any patterns visible in hindsight
   (e.g., the three functions over 50 lines that persisted across
   all 12 mint sessions).
4. Begin applying the session-end pattern check at the next session.

No retroactive changes to existing session entries or documents.

---

*This protocol was designed on 2026-03-18. It is itself subject to
the feedback loop — if the protocol does not produce useful
improvements after sustained use, it should be revised via proposal.*