# Skill: add-crate

> **When to invoke:** When a new library crate is needed in the workspace.
> This should be rare — the architecture defines the crate set. A new crate requires
> a new ADR in `<project>/ARCHITECTURE.md` justifying its existence before this skill runs.
> **Load:** `<project>/ARCHITECTURE.md`, `livery/livery/standards/ousterhout.md`

---

## Pre-conditions

Before executing this skill, confirm all of the following:

- [ ] An ADR in `<project>/ARCHITECTURE.md` documents why this crate exists, what complexity it
      hides, and why it cannot live in an existing crate
- [ ] The Ousterhout deep-module test passes: you can state in one sentence what
      complexity this crate hides
- [ ] The dependency graph has been updated in `<project>/ARCHITECTURE.md` with the new edge(s)
- [ ] The public API stub for the new crate is written in `<project>/ARCHITECTURE.md`

If any pre-condition is unmet, stop. Write the ADR first.

---

## Procedure

### Step 1 — Create the crate scaffold

```bash
mkdir -p crates/<crate-name>/src
```

Create `crates/<crate-name>/Cargo.toml`:
```toml
[package]
name = "<crate-name>"
version = "0.1.0"
edition = "2024"

[dependencies]
thiserror = { workspace = true }
# Add other workspace dependencies as needed

[dev-dependencies]
tempfile  = { workspace = true }
proptest  = { workspace = true }
```

Create `crates/<crate-name>/src/lib.rs`:
```rust
//! <One-sentence description of what this crate hides.>
//!
//! Entry point: `<primary_public_function>`.
//! See `<project>/ARCHITECTURE.md` for the module design and public API contract.
```

### Step 2 — Register in the workspace

Add to the root `Cargo.toml` `[workspace] members` list:
```toml
"crates/<crate-name>",
```

Add to `[workspace.dependencies]`:
```toml
<crate-name> = { path = "crates/<crate-name>" }
```

### Step 3 — Add the dependency to consuming crates

For each crate that depends on the new crate (per `<project>/ARCHITECTURE.md`), add to its
`Cargo.toml`:
```toml
[dependencies]
<crate-name> = { workspace = true }
```

### Step 4 — Implement the error type first (TDD entry point)

Before any other implementation, write the error type in `src/lib.rs`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum <CrateName>Error {
    // variants to be added as tests demand them
}
```

Then write the first failing test for the first public function. Begin the TDD cycle.

### Step 5 — Verify workspace integrity

```bash
cargo check --workspace
cargo test --workspace
```

Both must pass before the session that adds the crate ends.

### Step 6 — Update ARCHITECTURE.md

- Confirm the crate's entry in the Crate Map section is accurate
- Confirm the Dependency Graph Mermaid diagram is updated
- Confirm the Information Hiding Inventory has an entry for the new crate
- Confirm the Public API Stubs section has the final stubs (not draft)
