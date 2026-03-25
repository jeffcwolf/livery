# Skill: write-spec

> **When to invoke:** At the start of Phase 0, when writing the project's `SPEC.md`
> from scratch or evaluating an existing draft against the required sections.

---

## Execution Mode

This skill is run as a conversation, one step at a time. Present the step, ask
the human to answer it, evaluate the answer against the step's acceptance
criteria, and push back if the criteria are not met. Do not proceed to the next
step until the current step's output is accepted. Do not draft answers for the
human — ask questions that force the human to articulate the answer.

**Fast path:** If the human provides a complete draft `SPEC.md`, skip the
step-by-step conversation. Instead, evaluate the draft against every step's
acceptance criteria and report which steps pass and which need revision. Work
through the failing steps one at a time.

**Environment:** This skill can be used in Claude Chat (for drafting) or Claude
Code (for review). The output is a `SPEC.md` file in the project directory.

---

## Procedure

### Step 1 — Problem statement

Write one paragraph, maximum. Force constraints: one problem, one user, one
context. If the paragraph contains "and also" or describes two problems, split
or choose.

**Acceptance criteria:**
- Exactly one paragraph
- Identifies exactly one problem, one user, one context
- Does not contain "and also" or equivalent conjunctions linking separate problems
- No vague language: "makes it easier," "improves the workflow," or similar
  claims without a concrete comparison. Push back: easier than what? Measured how?

### Step 2 — User persona and usage context

Who uses this and in what situation? Not a marketing persona — a concrete
description of the person, their skill level, and the moment they reach for
this tool. If there are multiple user types, the primary one goes here; others
go in a "Secondary users" note.

**Acceptance criteria:**
- Describes a specific person (role, skill level, technical background)
- Describes the specific moment or situation they reach for this tool
- Is concrete, not aspirational ("researchers who publish software" not "anyone
  who wants better metadata")
- Secondary users, if any, are separated into a distinct note

### Step 3 — Feature list with explicit boundary

List every feature in v1 scope. For each feature, ask: "What's the adjacent
thing someone might assume is included?" Write that adjacent thing into the
Non-feature list (Step 4).

This is the non-feature elicitation step — it is the most important step in the
skill because unstated exclusions are where scope creep enters.

**Acceptance criteria:**
- Every feature is a concrete capability, not a category ("parse CITATION.cff
  files" not "file support")
- For each feature, at least one adjacent assumption has been identified and
  recorded for Step 4
- No feature is a restatement of the problem — each adds a specific capability

### Step 4 — Non-feature list

Every item here has a one-sentence rationale for exclusion. "Not in v1" is not
a rationale. "Excluded because it requires network access and v1 is offline-only"
is a rationale.

**Acceptance criteria:**
- Every non-feature has a one-sentence rationale that names a specific reason
  for exclusion
- No rationale is "not in v1," "out of scope," "future work," or equivalent
  deferrals without justification
- Every adjacent assumption identified in Step 3 appears here

### Step 5 — Success criteria

Each criterion must be mechanically checkable. Reject subjective criteria.

- "Works well" → not acceptable
- "Parses all valid CITATION.cff files without error" → acceptable
- "Handles edge cases" → not acceptable
- "Processes files with empty optional fields, multi-author entries, and Unicode
  titles without panic" → acceptable

Each criterion becomes a checkbox in the Phase 6 v1.0 gate.

**Acceptance criteria:**
- Every criterion describes a specific, observable outcome
- Every criterion can be verified by running a command, inspecting an output, or
  checking a measurable property — no human judgment required
- No criterion uses "well," "properly," "correctly," "appropriately," or
  similar subjective qualifiers without a concrete definition

### Step 6 — Risk register

Identify the parts of the design that are technically uncertain or likely to
require iteration. For each risk, state:

1. **What could go wrong** — the specific failure mode
2. **Mitigation** — what the design does to reduce likelihood or impact
3. **Isolation strategy** — how the architecture contains the blast radius so
   that if the risk materialises, only a bounded part of the system is affected

**Acceptance criteria:**
- At least one risk identified (a project with zero risks has not been examined
  carefully enough)
- Each risk has all three components: failure mode, mitigation, isolation
- Isolation strategies reference architectural boundaries (modules, crates,
  interfaces), not process steps

### Step 7 — Non-negotiable constraints

Hard constraints that cannot be relaxed. Examples: "Must run offline." "Must not
panic on malformed input." "Must compile on stable Rust." These become invariants
that every session checks against.

**Acceptance criteria:**
- Each constraint is absolute (not "should" or "ideally" — "must")
- Each constraint is testable or enforceable by the build system, linter, or
  test suite
- No constraint contradicts a feature in Step 3 or a success criterion in Step 5

### Step 8 — Review gate

The spec is complete when all sections exist AND the human has read the full
document and confirmed:

- (a) The non-feature list covers every plausible assumption
- (b) Every success criterion is checkable
- (c) Every risk has an isolation strategy

If any of these fail, revise before proceeding to Phase 1.

**Acceptance criteria:**
- All seven prior steps have accepted outputs
- The human has explicitly confirmed (a), (b), and (c) above
- The final `SPEC.md` is written to the project directory

---

## Stopping Condition

The skill is complete when `SPEC.md` exists in the project directory, all eight
steps have been accepted, and the human has confirmed the review gate.
