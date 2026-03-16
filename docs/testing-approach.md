# Testing Approach — Rationale and Enforcement

> **The short version:** We use TDD with three test layers and mandatory property-based
> tests because agent-generated tests have a specific, systematic failure mode —
> the shared-assumption problem — that only a disciplined testing approach can prevent.

---

## The Core Principle

Tests in an agentic codebase serve a different purpose than tests in a human-authored
codebase. In a human codebase, tests primarily catch regressions. In an agentic
codebase, tests must also serve as the *specification* — the independent, objective
statement of what the code should do, written before the code exists. Without this,
the agent writes both the implementation and the tests, and both can share the same
wrong assumption. A test suite can be 100% green and completely wrong.

The discipline of writing the test first, having a human review it before implementation
begins, and using property-based tests over arbitrary inputs is what separates a test
suite that proves correctness from one that merely demonstrates that the code does what
it does.

---

## Why This Approach

### Why TDD

Test-Driven Development (red/green/refactor) is not used primarily for its benefits to
design quality (though those are real). It is used because it is the only reliable
way to prevent the shared-assumption problem in agentic code.

When an agent writes implementation first and tests after, the tests verify what the
code does. This is the wrong question. The right question is: does the code do what
it *should* do? That question can only be asked before the implementation exists. Once
the agent has written the implementation, its test-writing is contaminated by its
implementation-writing — both draw from the same mental model, which may be wrong.

The red phase — writing a failing test and confirming it fails — is the enforcement
mechanism. A test that is written after the implementation cannot be confirmed to fail
for the right reason. The confirmation of the red phase is what gives TDD its
correctness guarantee.

### Why Three Test Layers

A single test layer cannot cover all failure modes:

**Unit tests** (co-located, behaviour-named) cover the contract of individual functions.
They are fast, precise, and cheap to write. But they test example inputs, and example
inputs can share assumptions with the implementation. A unit test that passes does not
prove the implementation is correct for all inputs — only for the examples chosen.

**Property-based tests** (proptest/quickcheck) cover the contract over arbitrary inputs.
They test invariants ("the output always satisfies this constraint"), roundtrip
properties ("parse then serialise returns the original"), and equivalence with a
reference model ("the optimised implementation matches the naive implementation").
Property tests expose assumptions that example tests never surface — because the test
generator will eventually produce the input that the author's mental model treated as
impossible.

**Integration tests** (CLI subprocess via `assert_cmd`) cover the user-facing contract:
given this input to the CLI, the tool produces this output and this exit code. They
catch failures that unit tests miss because they test the full system path, including
the wiring in `mint-cli` that unit tests never reach.

All three layers are necessary. A codebase with only unit tests has no property
coverage. A codebase with only integration tests has no isolation — failures are hard
to diagnose. A codebase with only property tests has no readable specification of
the specific expected behaviours.

### Why Property Tests are Mandatory for Transformation Functions

Any function that parses input, transforms data, or performs algorithmic computation
is at risk for a specific class of bug: the edge case that the author never imagined.
Example-based tests cannot cover what the author did not imagine. Property-based tests
can — because they generate inputs the author did not choose.

For `mint-meta` specifically, the CFF roundtrip property
(`parse → serialise → parse` produces identical output) is the most important test in
the codebase. It is impossible to write as an example-based test, because you cannot
enumerate all valid `CITATION.cff` files. With proptest, you can generate thousands
of valid files and verify that the roundtrip property holds for all of them. If it
fails on any generated input, you have found a real bug — one you would never have
found by writing examples.

### Why Reference Models

For complex algorithms or critical business logic, a naive obviously-correct reference
implementation is written alongside the real implementation, and property tests verify
that the two agree on arbitrary inputs. This is the strongest possible test: it
separates the *specification* (the reference) from the *implementation* (the
optimised function), and uses the property testing framework to verify they are
equivalent.

The reference model is written independently of the implementation — it should be
obviously correct by inspection, even if slow or verbose. The real implementation can
be opaque as long as it matches the reference on all inputs.

---

## The Shared-Assumption Problem

This deserves its own section because it is the most dangerous failure mode in agentic
testing and the primary motivation for TDD discipline.

**The problem:** An agent writes implementation A. Then writes test T for A. Both A
and T are generated by the same model with the same context. If the model has a
systematic wrong belief about what A should do, A will implement that belief and T
will verify it. T passes. The code is wrong.

This is not a hypothetical — it is the default outcome when an agent writes tests
after implementation. The agent's test-writing is not independent of its
implementation-writing. It is the same process, drawing from the same (possibly wrong)
understanding.

**The countermeasures, in order of effectiveness:**

1. **Human review of the red phase.** The human reads the failing test *before*
   implementation begins and asks: "Does this test capture the actual contract, or just
   the implementation I'm about to see?" This is the only 100% reliable countermeasure.
   It requires human judgment and cannot be automated.

2. **Property tests over example tests.** Property tests generate inputs the agent
   did not choose, exposing assumptions the agent did not know it was making.

3. **Reference models.** An independently-written naive implementation provides an
   objective standard that neither the agent's implementation nor the agent's examples
   can contaminate.

4. **Behaviour-named tests.** Tests named for the behaviour they verify
   (`parse_citation_rejects_missing_version`) rather than the implementation they test
   (`test_parse_function`) are slightly harder to write incorrectly — the name forces
   the author to state what should be true before writing the assertion.

---

## How It's Encoded

| Concern | Where encoded | How enforced |
|---|---|---|
| TDD sequence | `livery/CLAUDE-base.md` TDD section | Non-negotiable workflow; session never skips red |
| Human review of red phase | `livery/CLAUDE-base.md` TDD section; `livery/skills/session-open.md` | Stated explicitly as part of the session-open procedure |
| Three test layers | `livery/CLAUDE-base.md` Testing Standards | Co-location rule, naming rule, proptest mandate |
| Property tests mandatory for parsers/transformers | `livery/CLAUDE-base.md` Testing Standards | Explicit rule; `mint/skills/write-proptest.md` provides patterns |
| Reference models | `livery/CLAUDE-base.md` Testing Standards | Rule stated; applied to complex algorithms |
| Test quality gate | `livery/CLAUDE-base.md` Testing Standards | Mutation test criterion stated explicitly |
| Behaviour-named tests | `livery/standards/rust-specifics.md` Part IV | Naming convention for test functions |
| Proptest strategies for mint types | `mint/skills/write-proptest.md` | Concrete strategy implementations for CFF types |

---

## How It's Enforced in Practice

### During the Red phase

The agent writes the failing test. The human reads it and asks: does this test capture
the actual contract? This is a human-in-the-loop step — it cannot be automated. The
test must fail for the right reason before implementation begins.

For property tests, the strategy must generate realistic inputs — not arbitrary
strings, but constrained strings that match the domain (ORCID format, semver tags,
CFF-valid titles). This is why `mint/skills/write-proptest.md` exists: without concrete
strategy patterns, agents will use `String::arbitrary()` which generates garbage
inputs that fail for the wrong reasons.

### During the Green phase

The implementation is written to pass the test. No more. The agent is explicitly
prohibited from writing implementation that the test does not demand. Anticipatory
code is a specific anti-pattern: it adds untested behaviour that was not reviewed
in the red phase.

### During session-end validation

`cargo test --workspace` must pass. Failing tests are a hard stop — the session does
not close until they pass. `cargo test --doc` must also pass, catching stale doctests.

### At milestones

`cargo-mutants` is run on critical modules (especially `mint-meta` and `mint-check`)
to verify test strength. If a mutation survives — meaning the implementation can be
changed without breaking any test — the test suite has a gap that must be filled.

---

## What Failure Looks Like

**Tests that pass but verify nothing.** `assert!(result.is_ok())` tells you the
function returned Ok; it tells you nothing about what was in the Ok. This is the most
common weak test pattern. It is prohibited by `livery/CLAUDE-base.md` — all assertions must
unwrap or match so failures are informative.

**No property tests on the CFF parser.** If `mint-meta` has only example-based tests,
there are CITATION.cff files in the wild that will cause incorrect behaviour — files
with unusual Unicode in titles, multiple authors with no ORCIDs, optional fields in
unexpected combinations. Property tests with a realistic strategy will find these
before users do.

**Green-phase implementation diverges from red-phase contract.** The test says
`parse_citation` should return `Err(MetaError::MissingField)` for a file without a
`title` field. The implementation returns `Ok` with an empty title. The test passes
because it was written to match the implementation rather than capture the contract.
This is the shared-assumption problem in its purest form.

**Tests named for implementation.** `test_parse_function`, `test_orcid_regex`,
`test_process`. These names do not specify what should be true — they only specify
what is being tested. A test named `parse_citation_returns_err_for_missing_title`
cannot silently accept an empty-title Ok result without breaking.

---

## Connection to Other Concerns

**Design.** Deep modules (Ousterhout) are testable by definition: narrow public APIs
require fewer test cases to cover the contract. A module with a wide public API — one
that leaks internal structure — requires tests for both the contract and the
implementation details, and those tests break whenever the implementation changes.
Good design makes testing cheaper.

**Documentation.** A behaviour-named test is a form of documentation. The test name
`parse_citation_rejects_missing_title` is a spec: it says that a missing title is
a rejection condition. A collection of well-named tests is a machine-executable
specification of the module's behaviour. This is why test naming is a documentation
concern as much as a testing concern.

**Validation.** Test results feed directly into the validation pipeline. The session
contract requires `cargo test --workspace` to pass before the session closes. Tests
are not separate from validation — they are the foundation it rests on.
