# Design Philosophy — Approach and Rationale

> **The short version:** We use John Ousterhout's *A Philosophy of Software Design*
> because it is the most actionable framework for fighting complexity in software —
> and complexity is the primary reason agentic codebases become unmaintainable.

---

## The Core Principle

Software complexity is not an inevitable consequence of software growing large. It is
the accumulated result of thousands of small decisions that each seemed reasonable in
isolation: adding a flag to an existing function instead of redesigning it, exposing
an internal type because it was convenient, writing a module for each stage of a
process rather than for each domain of knowledge. Ousterhout's framework names these
patterns, explains why they are harmful, and provides principles for avoiding them.

The central claim: **a module should hide significant complexity behind a simple
interface.** The ratio of complexity concealed to interface complexity — module depth —
is the primary metric of good design. Systems built from deep modules are easier to
read, easier to test, easier to change, and easier for agents to reason about correctly.

---

## Why This Approach

### Why Ousterhout specifically

Several design philosophies were considered. The reason Ousterhout was chosen over
alternatives (SOLID, Clean Architecture, Domain-Driven Design) is specificity.
Ousterhout's principles are stated precisely enough to be applied mechanically. "Deep
modules over shallow modules" is a principle with a test: count the interface complexity,
estimate the concealed complexity, compute the ratio. SOLID's "single responsibility
principle" is stated vaguely enough that reasonable engineers disagree about what it
means in practice. Ousterhout's equivalent — "if you cannot describe what a module does
in one sentence without 'and', it has two responsibilities" — is not vague.

This specificity is essential for agentic coding. An agent that has been told "follow
the single responsibility principle" will produce code that nominally complies while
violating the spirit. An agent that has been told "a module whose interface is nearly as
complex as its implementation is shallow and must be redesigned" can apply that rule to
code it just wrote.

### What failure looks like without this approach

In agentic codebases built without a design philosophy, the following patterns emerge
reliably across sessions:

**Shallow wrapper proliferation.** The agent creates a new struct for every concept
introduced, wrapping an existing type with a new name and forwarding all its methods.
Each wrapper is locally reasonable; collectively they create a maze of indirection that
hides nothing and adds cognitive overhead everywhere.

**Temporal decomposition.** The agent structures code around the sequence of operations
("first parse, then validate, then transform") rather than around domains of knowledge.
Each stage becomes a module. All stages share detailed knowledge of the same data
structure. A change to the data structure requires changes in every stage.

**Information leakage.** Internal types and data structures appear in public interfaces
because it was convenient. Callers now depend on implementation details. The
implementation can never change without breaking callers. The module's interface is
effectively its implementation.

**Tactical accumulation.** Each session makes locally defensible decisions — a flag
parameter here, a special case there. No single decision is egregious. Over twenty
sessions, the accumulated result is a codebase where every function has three boolean
parameters, every module has a catch-all `process` method, and no piece of code can
be understood without reading everything it depends on.

Ousterhout's framework makes these patterns recognisable and nameable *before* they
become entrenched. The Red Flags are the early warning system.

---

## How It's Encoded

| Concern | Where encoded | How enforced |
|---|---|---|
| Core principles (8 rules) | `livery/standards/ousterhout.md` Parts I | Refactor Pass 1; `review-for-red-flags` skill |
| Red Flags (13 patterns) | `livery/standards/ousterhout.md` Part II | `review-for-red-flags` skill — each named and checked |
| Design Process Checklist | `livery/standards/ousterhout.md` Part III | Phase 1 standards audit; end-of-session audit |
| Deep-module test | `livery/CLAUDE-base.md` Design Philosophy section | Stated as a hard constraint; referenced in `add-crate` skill pre-conditions |
| Strategic programming rule | `livery/CLAUDE-base.md` Design Philosophy section | The anti-hack rule: no hack without recording design debt |
| ADR requirement | `<project>/ARCHITECTURE.md` ADR log | `update-architecture` skill mandates an ADR for every structural decision |
| Module depth measurement | `prism audit` command | Session-end validation pipeline; Prism baseline delta tracking |

The encoding is layered deliberately: the principle is stated concisely in `livery/CLAUDE-base.md`
(always in context), defined precisely in `livery/standards/ousterhout.md` (loaded when
needed), and measured mechanically by Prism (run at session end). Three layers of
enforcement for the most important concern in the system.

---

## How It's Enforced in Practice

### During Phase 1 (Architecture)

The `livery/standards/ousterhout.md` Design Process Checklist is run against every proposed
module *before any code is written*. This is the cheapest point to catch design
problems — a shallow module discovered in the architecture document costs one sentence
to fix; the same module discovered after three sessions of implementation costs days.

The checklist asks, for every module:
- Can you state in one sentence what complexity it hides?
- Were at least two alternative interfaces designed? (Design it twice)
- Is every public item truly public with at least two callers?
- Are there error conditions that could be eliminated by type constraints?

Findings from the Phase 1 audit are recorded as ADRs in `<project>/ARCHITECTURE.md` so future
sessions know what was considered and why the current design was chosen.

### During Phase 3 (Sessions) — Refactor Pass 1

Every TDD cycle has a mandatory Refactor phase. Pass 1 of that phase is Ousterhout:
load `livery/standards/ousterhout.md`, apply the Design Process Checklist to every new or
modified module, fix every Red Flag found. This happens within the session, not as
a post-session review — it is structurally part of the TDD cycle.

The reason Pass 1 is Ousterhout and not ARC: structure must be correct before surface
can be polished. Refactoring a name on a shallow module makes the shallow module
slightly prettier; it does not fix the shallow module.

### During Phase 3 (Sessions) — Post-session `review-for-red-flags` skill

At the end of every session, the `review-for-red-flags` skill is invoked. It works
through all 13 Red Flags explicitly, stating for each whether it is present or absent
in the session's code. This is a second pass — the first pass happens during Refactor,
but agents under time pressure during Refactor may miss subtle instances. The
post-session audit is the catch-all.

Any Red Flag found present must be fixed before the session closes, or recorded as
design debt in `<project>/SESSIONS.md` with a planned fix session.

### During Phase 4 (Validation) — Prism

`prism audit` measures module depth ratios mechanically. `prism check --strict`
enforces complexity thresholds. These provide objective, quantitative confirmation
that the design is not degrading across sessions. A module depth ratio that has
decreased between sessions is a design quality alert, even if no Red Flag was
explicitly identified.

---

## What Failure Looks Like

When this approach is applied inconsistently — rules stated but not enforced — the
specific failure modes are:

**Shallow modules that pass review.** A struct with three fields and three getters that
add no invariants. It looks like a domain type; it is a named tuple. It passes `cargo
clippy` and `prism check` but adds cognitive overhead everywhere it appears.

**Red Flags in the session log.** If `<project>/SESSIONS.md` Red Flag Audit sections show
"none found" across many sessions on a complex codebase, the audits are not being
run honestly. Real codebases generate Red Flags; the discipline is in catching and
fixing them, not in avoiding them.

**Architecture drift.** `<project>/ARCHITECTURE.md` describes a clean six-crate structure.
The actual code has business logic in `mint-cli`, CFF field knowledge in `mint-archive`,
and a `utils` module in `mint-meta` that does five unrelated things. The architecture
document and the code have diverged because `update-architecture` skill was not invoked
and ADRs were not written for structural departures.

---

## Connection to Other Concerns

**Testing.** Ousterhout's deep modules are testable by definition: a narrow public API
means fewer test cases are needed to cover the contract; hidden complexity means
implementation can change without breaking tests. Shallow modules are hard to test
meaningfully because their interface and implementation are nearly identical —
tests end up verifying the implementation, not the contract.

**Documentation.** The "implementation documentation contaminates interface" Red Flag
is a design problem stated as a documentation symptom. If you cannot write a doc
comment describing a module's contract without mentioning its implementation, the
module is not hiding its implementation. Difficulty writing a clean doc comment is a
design signal. See `docs/documentation-approach.md`.

**Naming.** The "hard to describe" Red Flag and ARC's naming principle converge: if
you cannot name or describe a module clearly without "and", it has two responsibilities.
Naming difficulty is a design problem. See `docs/naming-approach.md`.
