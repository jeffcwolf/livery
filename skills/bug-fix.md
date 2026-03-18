# Skill: bug-fix

> **When to invoke:** Whenever fixing a defect — a behaviour that is wrong, a crash,
> a check that produces incorrect results, or an output that does not match the
> contract in `<project>/SPEC.md` or `<project>/ARCHITECTURE.md`.

---

## The Fundamental Rule

**Never fix a bug without first writing a test that reproduces it.**

A bug fix without a test is a guess. The guess may be right, but without a test the
bug can silently return in a future session and will not be caught. The test is not
overhead — it is the proof that the fix is correct and the guarantee it stays fixed.

---

## Procedure

### Step 1 — Understand the bug precisely

Before writing any code, state the bug in one sentence:

> "When [input condition], [function/command] produces [actual output], but should
> produce [expected output]."

If you cannot state the bug this precisely, you do not understand it yet. Investigate
further before proceeding.

### Step 2 — Write a failing test (Red)

Write the minimal test that reproduces the bug exactly:

```rust
#[test]
fn <what_was_wrong>_is_now_correct() {
    // Arrange: set up the exact input condition that triggered the bug
    let input = ...;

    // Act: call the function that was wrong
    let result = the_function(input);

    // Assert: the expected correct output
    assert_eq!(result, expected);
}
```

For CLI bugs, use `assert_cmd`:
```rust
#[test]
fn check_does_not_flag_valid_orcid_with_trailing_slash() {
    let project = setup_project_with_orcid("https://orcid.org/0000-0002-1234-5678/");
    mint()
        .args(["check", project.path().to_str().unwrap()])
        .assert()
        .success(); // not failure
}
```

Confirm the test fails. If it passes, the test does not reproduce the bug — investigate
further.

### Step 3 — Locate the root cause

With the failing test as a guide, find the root cause. Distinguish:

- **Symptom:** what the user observed or the test asserts
- **Root cause:** the specific line or logic that produces the wrong result

Do not fix the symptom. Fix the root cause.

### Step 4 — Fix the implementation (Green)

Write the minimum change that makes the failing test pass without breaking any
existing tests.

```bash
cargo test --workspace
```

All tests must pass — the new test and all existing ones.

### Step 5 — Check for related failures

Ask: could the same root cause produce similar bugs elsewhere? If yes:
- Write tests for those cases too
- Fix them in the same session if they are small
- Record them as follow-up items in `<project>/SESSIONS.md` if they are large

### Step 6 — Refactor if the fix revealed a design problem

Sometimes a bug exists because the design made it easy to write. If the fix revealed
that:
- A function was doing too much (and the bug lived in the part it shouldn't own)
- An invariant was expressed as a runtime check that should be a type constraint
- Two modules shared knowledge they shouldn't

...then apply the appropriate Ousterhout principle from `livery/livery/standards/ousterhout.md` and
refactor. The bug is a symptom; the design problem is the disease.

### Step 7 — Run the full validation pipeline

Follow the `run-validation` skill. A bug fix is not complete until all gates pass.

### Step 8 — Record in `<project>/SESSIONS.md`

```
**Bug fixed:** [one-sentence description]
**Root cause:** [where and why the bug existed]
**Test added:** [test name and file]
**Design change (if any):** [none / description]
```
