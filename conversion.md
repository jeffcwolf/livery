# conversion.md — Applying Livery to an Existing Codebase

> **When to use this document:** When the project being built with Livery already exists
> as a working codebase — either one you wrote without Livery, or one you are rebuilding
> from scratch using an existing implementation as a reference. Read this document
> alongside `livery/WORKFLOW.md`. This document describes *deviations* from the standard
> workflow, not a replacement of it.
>
> **Reference example:** The conversion of `release-scholar` → `mint`. The original
> `release-scholar` is a working Rust CLI. `mint` is a ground-up rebuild using Livery,
> with `release-scholar` as the correctness oracle.

---

## The Core Difference

In a greenfield project, `SPEC.md` is written from a problem statement and user need.
Correctness is defined by the design. There is no prior implementation to compare against.

In a conversion, **the existing codebase is the specification**. It defines what "correct"
means — not perfectly, not permanently, but operationally. The existing tool works. The
rebuild must work at least as well. This changes the nature of every phase.

The most important implication: **the existing codebase is your reference model.** This
is the same pattern as the property-test reference model in `livery/CLAUDE-base.md`,
applied at the project level. The naive, known-correct implementation exists. The task is
to produce a better-structured one that agrees with it on all inputs.

---

## Phase -1 — Survey (new phase, before Phase 0)

> *Understand what you have before designing what you want.*

In a greenfield project, Phase 0 starts from a problem statement. In a conversion, Phase 0
must start from an honest account of what already exists. Phase -1 produces that account.

### What to produce: `<project>/SURVEY.md`

A survey document with four sections:

**1. What it does** — A functional description of the existing codebase. What commands
or entry points exist, what they accept, what they produce. Be precise enough that
`SPEC.md` can be written by studying this document, not by re-reading the source code.

**2. What works well** — Features that are correct, well-designed, or worth preserving
in the rebuild. These become firm scope items in `SPEC.md`.

**3. What is problematic** — Known bugs, design problems, missing features, or
architectural issues. These are the motivation for the rebuild — the reasons "fix the
existing code" is a worse answer than "rebuild with Livery."

**4. The Prism baseline** — Run `prism check`, `prism audit`, and `prism stats --json`
on the existing codebase. Save the output. This is the objective starting point: the
existing code's quality metrics, without judgment. The rebuild is complete when the new
codebase meets or exceeds this baseline on every metric *and* passes the Livery quality
gates the original does not.

### Why this phase matters

Without a survey, `SPEC.md` will be written from memory or impression — and memory
is unreliable about what a codebase actually does vs. what you think it does. The survey
forces a systematic reading of the existing implementation before any design decisions
are made.

The survey also surfaces the non-obvious scope decisions. In the `mint` example: the
survey of `release-scholar` revealed that the `mirror` command is the narrowest-audience
feature and adds forge-specific complexity without generality. Without the survey, that
decision might have been made by gut feel. With it, it was made from evidence.

---

## Phase 0 — Design Specification (modified)

The standard Phase 0 applies with two additions:

**`SPEC.md` is written by studying `SURVEY.md`**, not from scratch. The problem
statement describes the gap between what the existing tool does and what a better-designed
tool should do. The feature scope is grounded in what the existing tool does, with
deliberate additions and deliberate removals.

**The non-feature list is informed by the survey's "problematic" section.** Features
that are excluded are excluded because the survey showed them to be narrow-audience,
forge-specific, out of scope, or better addressed by a different tool — not because they
weren't thought of.

**The v1.0 success checklist gets an additional item:**

```
- [ ] The rebuilt tool produces identical results to the original on the
      reference test suite (the set of real inputs used to validate the original).
```

This is the project-level reference model check. It is the most important correctness
gate in a conversion project.

---

## Phase 1 — Architecture (modified)

The standard Phase 1 applies with one structural addition:

**Produce `<project>/ARCHITECTURE-target.md`** (not `ARCHITECTURE.md`).

The target architecture is the intended design — what the rebuilt codebase will look like
when complete. It is written exactly as the standard Phase 1 document.

Optionally, if the existing codebase has a coherent enough structure to be worth
documenting, produce **`<project>/ARCHITECTURE-current.md`** alongside it. This
documents what the existing code does, not what it should do. The gap between the two
documents is the scope of the rebuild.

When the rebuild is complete, `ARCHITECTURE-current.md` is deleted (or archived) and
`ARCHITECTURE-target.md` is renamed to `ARCHITECTURE.md`. The conversion is structurally
complete when these two documents would be identical.

For simple conversions (small codebase, clear structure, short rebuild timeline), the
current architecture document may not be worth producing. The target architecture is
always required.

---

## Phase 3 — Session Execution (modified)

Two additions to the standard session discipline:

### The reference validation step

Every session that implements behaviour that exists in the original codebase must include
a reference validation step in its stopping conditions:

```
- [ ] Run the original tool and the rebuilt tool against the same input.
      Outputs must match (or the difference must be a documented intentional improvement).
```

This is done manually in early sessions and automated in later sessions as the test
suite matures. By the end of the project, this check runs automatically in CI.

The reference validation is the project-level application of the shared-assumption
countermeasure: both implementations processing the same input is the strongest possible
correctness check, because the two implementations were written independently.

### Session order follows dependency order, not feature order

In a greenfield project, sessions can be ordered by priority — most important features
first. In a conversion, sessions follow the **dependency graph of the architecture**:
leaf crates (no internal dependencies) first, root crate last.

For `mint`: `mint-config` and `mint-meta` first (leaf crates), then `mint-check` and
`mint-archive` (depend only on leaves), then `mint-publish`, then `mint-cli` last. This
ensures each crate can be validated against the reference implementation independently
before the full system is assembled.

---

## Phase 4 — Validation (modified)

### Prism semantics during conversion

The Prism baseline from `SURVEY.md` is the starting point, not the target. During active
conversion sessions, the rebuilt codebase will have:

- Lower test coverage than the original (tests are being rebuilt too)
- Possibly different complexity metrics
- Different module depth ratios (because the architecture is changing)

**Do not use Prism as a gate until the conversion is structurally complete** (i.e., until
all crates are implemented and the CLI is wired). Before that point, use Prism as a
*diagnostic* — track the deltas in `SESSIONS.md`, note trends, flag concerns, but do not
block sessions on Prism failures.

Once the conversion is structurally complete, switch Prism to gate mode (`--strict`) and
treat it as the standard per-session validation gate.

The SESSIONS.md entry format gets a conversion-specific addition during the diagnostic
phase:

```markdown
**Prism diagnostic (conversion phase):**
- Reference baseline (from SURVEY.md): [key metrics]
- Current: [key metrics]
- Delta: [what has improved, what is still below baseline, what is expected to improve]
```

---

## The Conversion Complete Criteria

A conversion is complete — and the project can switch to standard greenfield/maintenance
mode — when:

- [ ] All features in `<project>/SPEC.md` v1 scope are implemented
- [ ] The rebuilt tool produces correct results on the reference test suite
- [ ] `prism check . --strict` passes (not just diagnostic)
- [ ] `<project>/ARCHITECTURE-target.md` accurately describes the actual code
- [ ] The Prism baseline meets or exceeds the survey baseline on all metrics
- [ ] `ARCHITECTURE-current.md` is archived or deleted (the current architecture
      *is* the target architecture)
- [ ] `SURVEY.md` is moved to `<project>/docs/SURVEY-archived.md` — it is
      historical context, not active reference

At this point the project transitions fully to the standard Livery workflow. The
conversion document is no longer loaded in session prompts.

---

## When Rebuilding vs. Refactoring

Conversion covers two distinct cases that share the same workflow:

**Ground-up rebuild** (the `mint` pattern) — the existing codebase is used as a
reference oracle but no code is carried forward. The rebuild starts from an empty
repository. The existing codebase is run in parallel to validate correctness.

**Structural conversion** (applying Livery to an existing repo) — the existing code
lives in the repository and is progressively restructured to meet Livery standards.
No parallel implementation; the existing code is the starting point.

The structural conversion is higher-risk because regressions are harder to isolate —
the reference oracle and the implementation under development are the same codebase.
Mitigate this by: writing tests for existing behaviour *before* restructuring it,
keeping each session's scope to a single structural concern, and never restructuring
and adding features in the same session.

The ground-up rebuild is lower-risk and produces cleaner results. When the existing
codebase is poorly structured enough to warrant applying Livery, it is often better
to rebuild than to restructure.