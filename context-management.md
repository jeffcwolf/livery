# context-management.md — Graduated Context Window Protocol

> **Status: optional.** This protocol is not active by default. It is
> wired into the system when context exhaustion patterns are observed —
> either by the human noticing problems, or by the feedback loop
> detecting recurring patterns such as "session ended with uncommitted
> work," "review skills skipped," or "SESSIONS.md entry written without
> Prism data."
>
> **To activate:** Add this line to the project's CLAUDE.md:
> ```
> Load and follow `livery/context-management.md` throughout this session.
> ```
> Or, to activate system-wide, add a reference in CLAUDE-base.md §Session
> Contract.
>
> **To activate automatically via the feedback loop:** If
> `known-patterns.md` records 2+ context-related patterns across sessions,
> write a proposal to activate this protocol. The proposal targets
> CLAUDE-base.md §Session Contract. See `feedback-loop.md` for the
> proposal process.
>
> **Why optional:** Superpowers' subagent-driven development is the
> primary mitigation for context exhaustion. Subagents start with fresh
> context; the orchestrator accumulates slowly. With short sessions and
> subagent dispatch, context exhaustion is rare. This protocol exists
> as a safety net for when it isn't rare.

---

## 1. The Problem

The agent's context window is finite. As a session progresses, it fills
with conversation history, file contents, tool outputs, and accumulated
state. When it fills completely, the agent loses access to early
instructions — including the constitution, the session scope, and the
quality standards. The result is degraded output, skipped steps, and
in the worst case, a session that ends with uncommitted work and no
record of what happened.

The usual mitigation — "make important instructions more prominent" —
does not work. Prominence helps at the margins but does not prevent
the fundamental problem: the agent cannot hold everything at once.
The structural mitigation is to *act earlier* — to begin the session
closing sequence before the context is exhausted, not after.

---

## 2. Detection

The agent cannot read an exact token counter. It estimates context
usage from:

- **Conversation length.** Number of turns, volume of text exchanged.
- **Files read.** Each file loaded into context consumes space
  proportional to its size.
- **Tool calls.** Each tool call and its output adds to context.
- **Subagent reports.** Orchestrator receives summaries from subagents;
  these accumulate.

These are imprecise proxies. The protocol is designed to be safe under
imprecision — the thresholds are conservative and the actions at each
level are idempotent (safe to trigger early, no harm if the zone
estimate is wrong).

---

## 3. Zones and Responses

### Green (estimated 0–50% context used)

Normal operation. No special behavior.

- Read files freely.
- Run tools as needed.
- Implement directly or via subagents as appropriate.

### Yellow (estimated 50–70% context used)

Early awareness. Two pre-emptive actions:

1. **Capture Prism baseline if not already done.** The baseline capture
   (`livery/bin/prism stats . --json > /tmp/prism-session-before.json`)
   should happen at session start per the standard protocol. If it was
   skipped or failed, do it now. Don't risk losing the ability to run
   it later.

2. **Prefer subagent dispatch.** Any remaining implementation tasks
   should be dispatched to subagents rather than done directly in the
   orchestrator context. The orchestrator conserves its remaining
   context for coordination, reviews, and the session entry. If
   subagents are not available (runtime doesn't support them), note
   this and continue, but be aware that the orange zone will arrive
   sooner.

### Orange (estimated 70–85% context used)

Begin closing sequence. Do not start new implementation work in the
orchestrator.

1. **Assess remaining scope.** Are there unfinished tasks from the
   plan? If yes, decide:
   - Can they be dispatched to a subagent? → dispatch.
   - Are they small enough to complete quickly? → complete, then
     proceed to step 2 immediately.
   - Are they too large? → defer to next session. Record in
     SESSIONS.md as a deferred item.

2. **Run quality gate.**
   ```bash
   cargo test --workspace
   cargo fmt --check
   cargo clippy --workspace -- -D warnings
   livery/bin/prism check . --strict
   livery/bin/prism stats . --json > /tmp/prism-session-after.json
   ```
   These commands consume context (their output is added to the
   conversation). Run them now while there is room to act on failures.

3. **Run review skills.** Red Flag Audit, Naming Review, Documentation
   Review. These read files and produce findings — context-consuming
   operations. Run them now, not later.

4. **Compute Prism delta.** Run
   `livery/bin/prism diff /tmp/prism-session-before.json .` to compare
   the before-snapshot against the current state. Hold the output for
   the SESSIONS.md entry.

5. **If Prism gate fails:** fix the violations if the fix is small and
   well-understood. If the fix requires substantial work, record it as
   a deferred item and note `[PRISM: N violations deferred]` in the
   session entry. An honest incomplete session is better than a context-
   exhausted session that attempts a fix and fails silently.

### Red (estimated 85%+ context used)

Preserve and stop. Every action in this zone must be as small as
possible.

1. **Commit all current work.** `git add` and `git commit` everything
   in its current state, even if incomplete. The commit message should
   note `[WIP: context limit reached]` if work is unfinished.

2. **Write the SESSIONS.md entry.** Use whatever data is available.
   If reviews didn't run, note:
   ```
   **Red Flag Audit:** [deferred: context limit]
   **Naming Review:** [deferred: context limit]
   **Docs Review:** [deferred: context limit]
   ```
   If Prism didn't run, note:
   ```
   **Prism baseline delta:** [PRISM: deferred to session N+1 preflight]
   ```

3. **Commit the session entry.**

4. **Stop.** Do not attempt new work. Do not attempt to "squeeze in"
   one more task. The session is over.

5. **The next session's preflight** must: run any deferred review skills
   on the files touched in the prior session, capture the Prism delta
   that was missed, and record the results as an addendum to the prior
   session's SESSIONS.md entry.

---

## 4. Priorities Under Pressure

When context is limited, prioritize in this order:

1. **Committed code.** Uncommitted work is lost if the session dies.
   Commit early, commit often. A series of small commits that can be
   squashed later is better than one large uncommitted changeset.

2. **SESSIONS.md entry.** The session record is the institutional
   memory. Without it, future sessions start from scratch. An
   incomplete entry with `[deferred]` markers is valuable. No entry
   at all is a loss.

3. **Prism data.** The quality gate and delta are the objective quality
   record. If you must choose between running Prism and running review
   skills, run Prism — it's automated and less context-consuming than
   the review skills.

4. **Review skills.** Red Flag Audit, Naming Review, Docs Review. These
   are valuable but they are also the most context-consuming
   (they read files and produce detailed findings). They can be
   deferred to the next session's preflight without data loss.

5. **Implementation completeness.** An incomplete but clean session —
   with committed code, a session entry, and Prism data — is always
   preferred over a complete but undocumented session.

---

## 5. How This Interacts with Subagents

When the runtime supports subagent dispatch (e.g., Superpowers'
subagent-driven development):

- **Subagents are context-free.** Each subagent starts fresh. It does
  not inherit the orchestrator's context. Dispatching work to subagents
  does not fill the orchestrator's context with implementation details.

- **The orchestrator fills slowly.** Its context grows from: the
  constitution (read at session start), the plan, coordination messages,
  subagent summaries, and tool outputs from its own actions (Prism,
  reviews, git commands). This is much less than an agent doing all
  implementation directly.

- **The yellow zone action "prefer subagent dispatch" is the primary
  defense.** By shifting remaining work to subagents in the yellow
  zone, the orchestrator preserves its context for the closing sequence.

- **Without subagents (e.g., pure Livery, or a runtime without
  subagent support), context fills faster.** The thresholds in §3 are
  designed for the worst case. With subagents, you may rarely leave
  the green zone in a normal session. Without subagents, you will
  reach orange regularly in longer sessions.

---

## 6. Signals That This Protocol Should Be Activated

If this protocol is not yet active, watch for these signals in
`known-patterns.md` and SESSIONS.md:

- Sessions ending with `[deferred: context limit]` markers
- Sessions where the SESSIONS.md entry is noticeably thinner than
  earlier sessions (fewer details, missing audit results)
- Sessions where the agent mentions running out of room or needing
  to hurry
- Prism data missing from session entries
- Uncommitted work discovered at the start of the next session
- Review skills producing no findings late in sessions (not because
  the code is clean, but because the skills weren't run thoroughly)

Two or more of these across sessions is the trigger. Write a proposal
per `feedback-loop.md` to activate this protocol system-wide.

---

*Protocol version: 1.0. Written 2026-03-18. Status: optional.*