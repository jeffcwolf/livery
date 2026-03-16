# Skill: update-architecture

> **When to invoke:** Any session that changes a crate's public API, adds or removes
> a crate, changes a dependency relationship, or makes a structural decision that
> differs from what `<project>/ARCHITECTURE.md` describes.
> **Load:** `<project>/ARCHITECTURE.md`

---

## The Core Rule

`<project>/ARCHITECTURE.md` must always reflect the actual code. If the document and the code
diverge, the document is wrong — unless the divergence was intentional (in which case
it must be recorded as an ADR). An agent reading `<project>/ARCHITECTURE.md` in a future session
must be able to trust it completely.

---

## Procedure

### Step 1 — Identify what changed

At the end of a session, review the diff of all files changed. Identify:

- New public functions or types added to a library crate
- Public functions or types removed or renamed
- New crates added to the workspace
- Changed dependency relationships between crates
- Any structural decision that differed from the documented design

### Step 2 — Update the relevant sections

For each change, update the corresponding section(s) in `<project>/ARCHITECTURE.md`:

| What changed | Section(s) to update |
|---|---|
| New public function/type | Public API Stubs for that crate |
| Removed/renamed public item | Public API Stubs; check if Information Hiding Inventory is affected |
| New crate | Crate Map, Dependency Graph (Mermaid), Information Hiding Inventory, Public API Stubs, Workspace Cargo.toml structure |
| Changed dependency | Dependency Graph (Mermaid diagram) |
| New structural decision | Architecture Decision Records (new ADR) |

### Step 3 — Update the Mermaid dependency graph

Regenerate using Prism to verify accuracy:

```bash
prism map . --mermaid
```

Compare against the `graph TD` block in `<project>/ARCHITECTURE.md`. Update the block to match.
The Prism output is ground truth — the doc must match it.

### Step 4 — Write an ADR if needed

An ADR is required when:
- A structural decision departs from the original design
- A new crate is added (always requires ADR — see `add-crate` skill)
- A public API was redesigned during implementation rather than following the stub

ADR format:
```markdown
### ADR-NNN: <Title>

**Context:** What situation prompted this decision? What were the constraints?

**Decision:** What was decided?

**Consequences:** What are the results — positive and negative?

**Source:** [session N / standards audit / implementation finding]
```

Number ADRs sequentially. Never renumber or delete existing ADRs.

### Step 5 — Verify the document is consistent

After updating, read through `<project>/ARCHITECTURE.md` as if you were an agent starting a new
session. Ask:

- Does the Crate Map accurately describe every crate?
- Do the API Stubs match the actual public interfaces in the code?
- Does the Mermaid graph match `prism map --mermaid` output?
- Does the Information Hiding Inventory still describe what each crate actually hides?

If any answer is no, fix it before ending the session.
