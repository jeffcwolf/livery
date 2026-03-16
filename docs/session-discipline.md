# Session Discipline — Rationale and Enforcement

> **The short version:** Agentic coding tools are powerful but amnesiac. Each session
> starts with near-zero context. Structured sessions with defined scope, documented
> decisions, and verified stopping conditions are the only reliable defence against
> the context loss that compounds into unmaintainable codebases.

---

## The Core Principle

An agent that starts a session without clear scope will invent scope. An agent that
ends a session without verifying its work will leave a codebase in an unknown state.
An agent that makes structural decisions without recording them leaves no trail for
future sessions to follow. Multiplied across twenty or fifty sessions, these failures
produce a codebase that is technically functional but effectively unmaintainable —
no one, human or agent, can reconstruct why it is structured the way it is.

Session discipline is the practice of treating each session as a complete, bounded,
verifiable unit of work. It starts from a clean baseline, executes a defined scope,
ends with all gates passing, and records every decision made. The next session
inherits a clean baseline and a complete decision trail.

---

## Why This Approach

### Why sessions must have defined scope

An agent without a scope boundary will implement what seems useful. This is not a
failure of intelligence — it is a natural response to ambiguity. The agent will
implement features adjacent to the task, refactor code that was not part of the
task, add abstractions that seem helpful, and optimise things that did not need
optimising. Each of these additions may be individually reasonable. Collectively
they violate the scope boundary and introduce untested, unreviewed code into the
codebase.

The scope boundary is stated explicitly at the start of every session: what is in
scope, what is explicitly out of scope. The out-of-scope list is as important as
the in-scope list. An agent that knows that "changes to `mint-cli`" are out of
scope for this session will not make them. An agent that only knows the in-scope
task may cross into `mint-cli` "just to fix something while I'm nearby."

### Why sessions must start from a clean baseline

Building on a broken baseline compounds problems. If the previous session ended with
a failing test or a lint error that was not caught, the current session will either
inherit the failure silently or spend its first hour diagnosing a problem it did not
create. The `session-open` skill's first step — verify the baseline is clean — is
not bureaucracy. It is the precondition for meaningful work.

An agent that starts a session on top of a failing test has a contaminated frame of
reference. Its changes will be evaluated against the wrong baseline. The session's
stopping condition ("all tests pass") will be unachievable unless the pre-existing
failure is fixed first — which means the session's scope has already changed.

### Why sessions must end with all gates passing

"Mostly done" is not a stopping condition. A session that ends with "tests passing
except for one that I'll fix next time" has left a known failure in the codebase,
has not verified its work, and has created a dependency between sessions that will
compound if the next session also ends "mostly done."

The session contract in `livery/CLAUDE-base.md` is a binary gate, not a checklist to be
satisfied partially. All tests pass. Formatting is clean. Clippy is clean. Prism
passes. Documentation was reviewed. The session log was written. Until all gates
pass, the session is not complete.

### Why SESSIONS.md is the backbone of institutional memory

In a human team, decisions are remembered by the people who made them. In an
agentic workflow, the agent has no memory between sessions. The only memory is
what is written down. SESSIONS.md is the written memory of the project.

Without SESSIONS.md, every session starts without context:
- Why is this module structured this way?
- Why was this alternative rejected?
- What was the Prism baseline before this feature was added?
- What design debt was deliberately incurred in session 7?

With a complete SESSIONS.md, every session can recover this context in two minutes
by reading the last few entries. Architectural decisions are traceable. Design debt
is visible. Baseline deltas are measurable.

The SESSIONS.md entry is not optional and is not a post-hoc summary written from
memory. It is written as the last act of the session, with accurate information about
what was done and what the gates showed. A SESSIONS.md entry that says "tests passing"
when a test was left failing is a falsification of the project record.

---

## How It's Encoded

| Concern | Where encoded | How enforced |
|---|---|---|
| Session scope definition | `livery/CLAUDE-base.md` Session Contract; `livery/skills/session-open.md` | Every session prompt uses the template; scope stated before code is written |
| Baseline verification | `livery/skills/session-open.md` Step 1 | Mandatory first step; failing baseline stops the session |
| Reference document loading | `livery/skills/session-open.md` Step 4; `livery/CLAUDE-base.md` Reference Documents table | Named in session prompt; naming-as-design-signal note active throughout |
| Session opening log entry | `livery/skills/session-open.md` Step 5 | Written before any code |
| TDD cycle within session | `livery/CLAUDE-base.md` TDD section | Non-negotiable; Red phase confirmed before Green begins |
| Session gates (all green) | `livery/CLAUDE-base.md` Session Contract | Binary: all pass or session is not complete |
| SESSIONS.md entry format | `livery/CLAUDE-base.md` Session Contract; `AGENTIC_WORKFLOW.md` Phase 3 | Template specified; Red Flag Audit and Naming Review sections required |
| Prism baseline delta | `livery/skills/run-validation.md` Step 5 | Saved and compared at session end |
| Scope creep prevention | `livery/CLAUDE-base.md` Anti-Patterns (Scope creep) | Explicit anti-pattern; deferred items recorded in SESSIONS.md |

---

## How It's Enforced in Practice

### Before code is written: `session-open` skill

Step 1 verifies the baseline is clean — all tests passing, no lint errors. If the
baseline is broken, the session's first task is to fix it, recorded as such.

Step 3 explicitly states scope boundaries. Both what is in scope and what is out of
scope are written down before any code is written.

Step 5 writes the session opening entry in SESSIONS.md before any code. This is not
post-hoc — it is pre-committed. The scope is recorded; if it changes during the
session, the change is noted.

### During the session: TDD cycle

Each piece of behaviour follows red/green/refactor. The human reviews the red
(failing test) before implementation begins. This is the primary quality gate —
not a post-session audit, but a within-session human checkpoint.

Any scope expansion during a session requires explicit acknowledgment: the agent
states that it is about to do something outside the original scope, records why,
and either gets approval or records it as a deferred item.

### At session end: the five validation gates

`run-validation` skill runs five commands in sequence. All must pass. Then:
- `review-for-red-flags` skill is run
- `review-docs` skill is run (if public items were added or modified)
- `naming-review` skill is run (if new names were introduced)

Then the SESSIONS.md entry is written with: scope, decisions, files changed, Prism
baseline delta, Red Flag Audit results, Naming Review results, and any deferred items.

### At milestone: architectural review

Every major milestone (feature group, subsystem, version increment) includes a
human-led architectural review. This is not a session review — it is a review of
the accumulated decisions across multiple sessions. The questions are: does
ARCHITECTURE.md still accurately describe the code? Have any Red Flags accumulated
that individual session reviews missed? Is the Prism baseline trend positive?

---

## What Failure Looks Like

**The unbounded session.** A session with no defined scope that ends after "I
implemented a few things." No stopping condition was defined, so none was verified.
The codebase is in an unknown state. The next session inherits that unknown state
and will spend time characterising it before it can do useful work.

**The session that ends "mostly done".** One failing test, noted in the session log
as "will fix next time." The next session opens with a broken baseline (Step 1 of
`session-open` reveals the failure), spends thirty minutes diagnosing it, finds that
the fix requires context from the previous session that was not recorded, and spends
another thirty minutes recovering that context from the code. Total cost: sixty
minutes of the next session spent on a twenty-minute fix that was left unfinished.

**The silent scope expansion.** An agent implementing `mint-meta::parse_citation`
notices that `mint-check::SecurityCategory` could be improved, makes the improvement,
and notes it briefly in the session log. The improvement was not tested against the
session's stopping condition (which only covered `mint-meta`). The change in
`mint-check` is untested. Two sessions later, a security check regression is
discovered. Tracing it back to the undocumented scope expansion takes more time
than the original improvement saved.

**The SESSIONS.md that nobody trusts.** Session log entries that are vague ("worked
on parsing"), incomplete (no Prism delta, no design decisions), or falsified
("all tests passing" when they were not). After a few such entries, the SESSIONS.md
is no longer reliable institutional memory — it is noise. Future sessions stop reading
it. The institutional memory is lost. Every session that follows must re-derive
context that was already known and thrown away.

---

## Connection to Other Concerns

**Design.** The session scope boundary and the architecture boundary are the same
boundary. An agent that respects the session scope will not cross crate boundaries
without justification. An agent that documents its structural decisions will keep
ARCHITECTURE.md honest. Session discipline is architecture discipline at the micro
level.

**Validation.** Continuous validation (the three-granularity approach in
`docs/validation-approach.md`) is only possible if sessions end in verified states.
Per-session validation gates feed into the Prism baseline delta tracking, which feeds
into milestone architectural reviews. The session is the unit of validation as well
as the unit of work.

**Testing.** The TDD discipline (red before green) is a session-level constraint
enforced by the session structure. The red phase review is a within-session human
checkpoint that requires the session to pause before implementation begins. Without
session structure — specifically, the defined stopping condition that includes all
tests passing — the TDD discipline has no enforcement mechanism at its boundaries.
