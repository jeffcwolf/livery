# RP-stats Experiment — Pre-Registered Comparison Metrics

> **What this document is:** The measurement protocol for comparing two builds of the
> same tool (RP-stats): one built with Superpowers alone (naive), one built with
> Superpowers + Livery (hybrid). Written before either build begins. Metrics and
> thresholds are fixed here — no changes after run 1 starts.
>
> **Written:** 2026-03-19, before any code exists.

---

## Experiment Design

**Project:** RP-stats — scans a directory of Rust projects, collects per-project
statistics, writes a JSON registry, generates a static HTML dashboard with basic
sorting/filtering.

**Run 1 — Naive (Superpowers only).** No Livery methodology. Superpowers handles
brainstorming, planning, execution, review, and git workflow. The developer exercises
normal judgment but does not impose any external methodology. No SPEC.md, no
ARCHITECTURE.md, no CLAUDE.md constitution, no TDD mandate, no Prism gates, no
session log.

**Run 2 — Hybrid (Superpowers + Livery).** Full Livery methodology. Phase 0 spec,
Phase 1 architecture, Phase 2 constitution, Phase 3 TDD sessions with three-pass
refactoring, Phase 4 Prism gates, session log, reviews.

**Variable isolated:** The Livery constitution layer. Runtime (Superpowers), developer,
project scope, and language (Rust) are held constant.

**Build order:** Naive first, then hybrid. This simulates how most developers work
today and ensures the hybrid run does not inform the naive run. The hybrid run will
benefit from knowing the problem better — this is acknowledged and accepted as a
known bias. The naive run benefits from freshness and lack of methodology overhead.

---

## Measurement Tools

| Tool | What it measures | Installation |
|---|---|---|
| Prism (`livery/bin/prism`) | Cyclomatic complexity, cognitive complexity, function length, test counts, doc coverage, public API ratio, dependency counts | Pre-compiled binary |
| `cargo-tarpaulin` or `cargo-llvm-cov` | Line and branch test coverage | `cargo install cargo-tarpaulin` |
| `cargo-udeps` | Unused dependencies | `cargo install cargo-udeps` |
| Manual inspection | Qualitative metrics (see §3 and §4) | Human reviewer |

**Commands run on both codebases, identically:**

```bash
# Prism (quantitative structure + quality)
livery/bin/prism stats . --json > prism-stats.json
livery/bin/prism check . --strict

# Test coverage (quantitative)
cargo tarpaulin --workspace --out json > coverage.json

# Standard Rust gates
cargo test --workspace 2>&1 | tail -1
cargo fmt --check
cargo clippy --workspace -- -D warnings

# Dependency audit
cargo tree --depth 1 --workspace | wc -l

# Code duplication (manual grep-based or tokei + diff)
# Count lines of code with tokei if available, otherwise raw wc -l
find . -name '*.rs' -not -path '*/target/*' | xargs wc -l | tail -1
```

---

## Section 1: Size and Structure (Quantitative)

These metrics establish the basic shape of each codebase. Neither bigger nor smaller
is inherently better — the point is to see how the two approaches decompose the same
problem.

| Metric | Definition | Naive | Hybrid |
|---|---|---|---|
| Total Rust lines | Non-blank `.rs` lines (excluding `target/`) | | |
| Source files | Count of `.rs` files | | |
| Crate count | Number of crates in workspace (1 = single crate) | | |
| Module count | Number of `mod` declarations (proxy for decomposition) | | |
| Binary crate contains logic? | Does the binary crate contain business logic, or is it dispatch-only? | | |
| Deepest module nesting | Maximum depth of `mod` tree | | |
| Public API surface | `pub` items / total items (Prism `pub_ratio`) | | |
| Direct dependencies | Count from `Cargo.toml` (all crates combined) | | |
| Transitive dependencies | `cargo tree` total | | |

---

## Section 2: Tests and Test Quality (Quantitative + Qualitative)

This is the highest-priority comparison area. The hypothesis is that the hybrid
approach produces not just more tests but structurally different tests — property
tests, reference models, and tests that are independent of the implementation's
internal assumptions.

### 2a. Quantitative Test Metrics

| Metric | Definition | Naive | Hybrid |
|---|---|---|---|
| Unit test count | Functions annotated `#[test]` in `src/` | | |
| Integration test count | Functions annotated `#[test]` in `tests/` | | |
| Doc test count | Testable examples in `///` doc comments | | |
| Total test count | Sum of above | | |
| Test-to-code ratio | Tests per 100 lines of Rust (Prism `test_ratio`) | | |
| Property test count | Tests using `proptest` or `quickcheck` | | |
| Line coverage (%) | `cargo-tarpaulin` line coverage | | |
| Branch coverage (%) | `cargo-tarpaulin` branch coverage (if available) | | |
| Assertion density | Average assertions per test function | | |

### 2b. Qualitative Test Assessment

For each codebase, a human reviewer answers these questions. Score each 1–5
(1 = poor, 5 = excellent) with a brief justification.

| Criterion | Question | Naive | Hybrid |
|---|---|---|---|
| Independence | Do tests verify behaviour through the public API, or do they reach into internals? | | |
| Shared assumptions | Could a systematic misunderstanding in the implementation also produce passing tests? Inspect the 3 most complex test functions. | | |
| Edge cases | Are boundary conditions tested (empty input, missing files, malformed TOML, zero-crate workspaces)? | | |
| Error paths | Are error conditions tested, not just happy paths? | | |
| Readability | Can a reader understand what each test verifies without reading the implementation? | | |
| Naming | Do test names describe the behaviour being verified, not the function being called? | | |
| Setup/teardown | Is test setup isolated (temp dirs, fixtures), or do tests depend on ambient state? | | |
| Determinism | Are all tests deterministic? Any reliance on filesystem ordering, timestamps, or system state? | | |

---

## Section 3: Documentation Coverage and Quality (Quantitative + Qualitative)

### 3a. Quantitative Documentation Metrics

| Metric | Definition | Naive | Hybrid |
|---|---|---|---|
| Doc coverage (%) | Public items with `///` doc comments / total public items (Prism) | | |
| `cargo doc` clean? | Does `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` pass? | | |
| Doc test count | (Repeated from §2 — tests embedded in doc comments) | | |
| README exists? | Does a README.md exist with usage instructions? | | |
| ARCHITECTURE.md exists? | Does any architectural documentation exist? | | |
| Inline comments | Approximate count of `//` comments (excluding `///` and `//!`) | | |
| Comment-to-code ratio | Inline comment lines / total code lines | | |

### 3b. Qualitative Documentation Assessment

Score each 1–5 with justification.

| Criterion | Question | Naive | Hybrid |
|---|---|---|---|
| Contract-oriented | Do doc comments describe the contract (what, when, guarantees, panics) rather than restating the code? | | |
| Why, not what | Do inline comments explain *why*, not *what*? Are obvious-what comments absent? | | |
| Examples | Do doc comments on complex functions include usage examples? | | |
| Error documentation | Are error conditions and return types documented? | | |
| Module-level docs | Do modules (`//!`) have overview documentation explaining their responsibility? | | |
| Staleness risk | Does the documentation reference implementation details that would break if the code changes? | | |

---

## Section 4: Code Quality (Quantitative + Qualitative)

### 4a. Standard Complexity Metrics

These are the industry-standard quantitative metrics for code quality.

| Metric | Definition | Naive | Hybrid |
|---|---|---|---|
| Max cyclomatic complexity | Highest McCabe complexity of any single function (Prism) | | |
| Avg cyclomatic complexity | Mean across all functions | | |
| Max cognitive complexity | Highest SonarSource cognitive complexity (Prism) | | |
| Avg cognitive complexity | Mean across all functions | | |
| Functions over 50 lines | Count (Prism `fns_over_50_lines`) | | |
| Functions over 30 lines | Count (manual) | | |
| Longest function (lines) | | | |
| Max function parameters | Highest parameter count on any function | | |
| Avg function parameters | Mean parameter count | | |
| Code duplication | Approximate duplicated blocks (manual review or tool) | | |
| `unsafe` blocks | Count of `unsafe` usages | | |
| `unwrap()` / `expect()` in non-test code | Count — proxy for error handling quality | | |
| Clippy warnings | Count at default lint level | | |

### 4b. Structural Design Assessment

Score each 1–5 with justification. These are the Ousterhout-derived metrics that
no standard tool measures but that determine long-term maintainability.

| Criterion | Question | Naive | Hybrid |
|---|---|---|---|
| Module depth | Do modules hide significant complexity behind simple interfaces, or are they shallow wrappers? | | |
| Information hiding | Are internal representations hidden from public APIs? Could the implementation change without breaking callers? | | |
| Abstraction quality | Do abstractions correspond to real domain concepts, or are they arbitrary groupings? | | |
| Error handling | Are errors defined out of existence where possible? Are error types informative, not stringly-typed? | | |
| Trait usage | Are traits used for genuine polymorphism, or are there gratuitous trait abstractions? | | |
| Separation of concerns | Are I/O, parsing, logic, and presentation cleanly separated? | | |
| Configuration leakage | Does config/CLI logic leak into core modules, or is it contained at the boundary? | | |

### 4c. Naming Quality Assessment

Score each 1–5 with justification.

| Criterion | Question | Naive | Hybrid |
|---|---|---|---|
| Accuracy | Do names precisely describe what the thing does / contains? | | |
| Scope proportionality | Are short names used only for tiny scopes? Are long names used for broad-scope items? | | |
| Consistency | Are similar concepts named with similar patterns across crates? | | |
| Banned words | Are vague names (`data`, `info`, `manager`, `handler`, `process`, `utils`) absent? | | |
| Misleading names | Are there names that suggest a different behaviour than what the code does? | | |

---

## Section 5: Process Metrics

These measure the development process itself, not the output.

| Metric | Definition | Naive | Hybrid |
|---|---|---|---|
| Wall-clock time | Total time from first prompt to "done" (hours, approximate) | | |
| Session count | Number of Claude Code sessions used | | |
| Total prompts | Approximate number of human prompts/messages | | |
| Rework cycles | Times something was built, then torn down and rebuilt | | |
| Scope changes | Features added or removed after initial description | | |
| Final working state? | Does `cargo test --workspace && cargo clippy` pass cleanly? | | |

---

## Section 6: Summary Scorecard

Roll up the qualitative scores into category averages.

| Category | Max Score | Naive | Hybrid | Delta |
|---|---|---|---|---|
| Test quality (§2b, 8 criteria) | 40 | | | |
| Documentation quality (§3b, 6 criteria) | 30 | | | |
| Structural design (§4b, 7 criteria) | 35 | | | |
| Naming quality (§4c, 5 criteria) | 25 | | | |
| **Total qualitative** | **130** | | | |

Quantitative highlights (not scored, but compared):

| Metric | Naive | Hybrid | Better? |
|---|---|---|---|
| Test-to-code ratio | | | |
| Property test count | | | |
| Line coverage (%) | | | |
| Doc coverage (%) | | | |
| Max cyclomatic complexity | | | |
| Functions over 50 lines | | | |
| `unwrap()` in non-test code | | | |
| Wall-clock time | | | |

---

## Section 7: Interpretation Guidelines

Written before the experiment to prevent post-hoc rationalization.

**Strong evidence for Livery:** Hybrid scores ≥20% higher on the qualitative
scorecard AND has property tests where naive has none AND has lower max complexity
AND the time overhead is ≤2x.

**Moderate evidence for Livery:** Hybrid scores ≥10% higher on qualitative scorecard
with clear structural differences (more crates, better separation of concerns) but
time overhead is >2x.

**Weak or no evidence:** Qualitative scores are within 10%, or the hybrid takes >3x
as long for marginal improvement, or the naive run spontaneously produces good
structure (which would suggest the methodology's value is in the developer's head,
not the documents).

**Evidence against Livery:** Naive scores comparably or better on the qualitative
scorecard, or the hybrid's overhead is so large that the quality difference doesn't
justify it for a project of this size.

**Confounds to acknowledge regardless of outcome:**
- The developer built the naive version first, so the hybrid benefits from problem
  familiarity. This biases toward the hybrid.
- The developer designed Livery, so they are maximally skilled at applying it. A
  different developer might get less benefit. This biases toward the hybrid.
- The project is small (est. 800–1200 lines). Livery's benefits may be more
  pronounced on larger projects where design degradation compounds over sessions.
  A positive result here is a lower bound on the methodology's value.
- Superpowers is already a competent runtime. Comparing Livery+Superpowers against
  raw Claude Code with no tooling would be a weaker, less interesting comparison.

---

*Pre-registered 2026-03-19. Do not modify this document after run 1 begins.*