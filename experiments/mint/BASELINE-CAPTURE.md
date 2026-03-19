# BASELINE-CAPTURE.md
## How to Capture the release-scholar Baseline

> **When to run this:** Before writing a single line of mint code.
> The baseline must be captured from the original release-scholar codebase
> in its current state. Once mint exists, these numbers are the immutable
> "before" side of the comparison.
>
> **Where to record results:** Copy each output directly into
> `mint/docs/comparison/COMPARISON.md` in the corresponding `[Paste here]` slot.
> Commit the filled-in baseline to git before starting Phase 3.
> This makes the before-measurement tamper-evident.

---

## Setup

Clone release-scholar into a clean directory alongside mint:

```bash
cd ~/code
git clone https://github.com/jeffcwolf/release-scholar
cd release-scholar
```

Install Prism if not already installed:

```bash
cd ~/code/prism
cargo install --path crates/prism-cli
```

Install cargo-mutants if not already installed:

```bash
cargo install cargo-mutants
```

---

## Step 1 — Prism Stats (paste into §3.1 and §1.3)

```bash
cd ~/code/release-scholar
prism stats . --json > ~/code/mint/docs/comparison/release-scholar-stats.json
prism stats .
```

Record from the output:
- Lines of code, files, crates
- Unit tests, integration tests, doctests, test ratio
- Doc coverage %, pub items documented
- Unsafe blocks
- pub_items, total_items, pub_ratio
- max_fn_lines, fns_over_50_lines

---

## Step 2 — Prism Audit (paste into §1.2 and §2.1)

```bash
prism audit .
```

Record:
- Every module listed with its depth ratio
- Every `[SHALLOW]` flag
- Every function with its cyclomatic, nesting, and cognitive complexity scores
- The highest and lowest depth ratios

---

## Step 3 — Prism Check (paste into §5.1 and §5.2)

```bash
prism check . --fix-suggestions
```

Record each gate result (PASS/WARN/FAIL) and the overall verdict.
Note: do not run with `--strict` — use default thresholds for the baseline,
since release-scholar was not built against Livery quality gates.

---

## Step 4 — Prism Map (paste into §1.1)

```bash
prism map . --mermaid
```

Paste the full Mermaid diagram. This visualises the flat structure of
release-scholar (likely a single node, or a single crate with no internal
dependency graph).

---

## Step 5 — Mutation Testing (paste into §3.2)

This will take 5–30 minutes depending on codebase size.

```bash
cargo mutants --workspace 2>&1 | tee ~/code/mint/docs/comparison/release-scholar-mutants.txt
```

Record:
- Total mutations generated
- Mutations caught (tests failed)
- Mutations survived (tests passed — the critical number)
- Survival rate = survived / total × 100

---

## Step 6 — Test Without Credentials (paste into §6.1)

```bash
# Unset any Zenodo credentials
unset ZENODO_TOKEN
unset ZENODO_SANDBOX_TOKEN

# Run the test suite
cargo test --workspace 2>&1 | tee ~/code/mint/docs/comparison/release-scholar-test-no-creds.txt
```

Record:
- How many tests ran
- How many tests were skipped or failed due to missing credentials
- Whether any meaningful integration testing is possible offline

---

## Step 7 — Extension Point Analysis (paste into §6.2)

Answer this question manually: if you wanted to add a seventh check category
to release-scholar's `check` command, which files would need to change?

Scan `src/` and identify all locations where check category logic is registered,
dispatched, or aggregated. Count the files.

```bash
grep -rn "check\|audit\|scan\|validate" src/ | grep -v test | grep -v "//"
```

Record:
- List of files that would need modification
- Total count of change sites

---

## Step 8 — Property Test Check (paste into §6.3)

Verify the absence of property tests:

```bash
grep -rn "proptest\|quickcheck\|arbitrary" src/ Cargo.toml
```

Expected result: no matches. Record the output.

Optionally (if time permits): write a proptest strategy for release-scholar's
CITATION.cff parsing logic and run it. This is the most compelling evidence for
the shared-assumption problem. Any failures found are real bugs in a production tool.

---

## Step 9 — Test Name Sample (paste into §3.3)

Extract 20 test names from the release-scholar test suite:

```bash
grep -rn "#\[test\]" src/ -A 1 | grep "fn " | head -20
```

For each test name, classify as:
- **Behaviour-oriented**: describes what should be true
  (e.g., `parse_citation_rejects_missing_version`)
- **Implementation-oriented**: describes what is being called
  (e.g., `test_parse_function`, `test_citation_check`)

---

## Step 10 — Doc Quality Sample (paste into §4.2)

Select 10 public functions from release-scholar (vary across modules).
For each, read the doc comment and classify:
- **Contract-oriented**: describes what it guarantees and when it fails
- **Implementation-oriented**: describes how it works internally
- **Absent**: no doc comment
- **Minimal**: comment exists but adds nothing beyond the function name

```bash
# Find all pub fn declarations
grep -rn "pub fn" src/ | grep -v test | head -30
```

Select 10 representative ones. Record each function name, its doc comment
(or "none"), and the classification.

---

## Step 11 — Commit the Baseline

Once all slots in `COMPARISON.md` are filled:

```bash
cd ~/code/mint
git add docs/comparison/
git commit -m "Add release-scholar baseline measurements before mint Phase 3"
```

This commit is the tamper-evident stake in the ground. The before-measurement
is now permanent and dated. The mint values will be added in a later commit
when v1.0 is complete.

---

## What to Do If Prism Cannot Analyse release-scholar

release-scholar is a single-crate project without a workspace. Prism's workspace
analysis should still work on it via `Cargo.toml` at the root. If any Prism command
fails, record the error message and note "not applicable — flat crate" in the
comparison table. The absence of workspace structure is itself a finding.