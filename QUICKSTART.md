# QUICKSTART.md — From Blank Repo to First Coding Session

> **What this is:** A step-by-step guide for starting a new Rust project
> with Livery, Prism, and Claude Code on the web. Follow this once per
> project. When you're done, you'll have a fully constituted project
> ready for Phase 3 (session execution).
>
> **Assumes:** You have a GitHub account, a blank repo, Rust installed
> locally, and a Claude Code subscription. You do not need to be an
> expert in Rust or in Livery — this guide tells you exactly what to do.
>
> **Time:** About 2–3 hours across three phases, most of it in Claude
> Code sessions.

---

## Before You Start

Make sure you have:

- [ ] A blank GitHub repo (initialized with at least a README or .gitignore
  so you can clone it)
- [ ] Rust toolchain installed locally (`rustup`, `cargo`)
- [ ] Git installed locally
- [ ] Access to Claude Code on the web (claude.ai with Claude Code enabled)

---

## Phase 0 — Specification

> *What are you building? For whom? What is explicitly excluded?*

Phase 0 happens in conversation, not in code. The output is a `SPEC.md`
file.

### Step 0.1 — Clone your repo and add Livery

```bash
git clone https://github.com/YOUR_USER/YOUR_PROJECT.git
cd YOUR_PROJECT
git submodule add https://github.com/jeffcwolf/livery livery
git submodule update --init --recursive
```

Verify Livery is present:

```bash
ls livery/CLAUDE-base.md    # should exist
ls livery/bin/prism          # should exist (pre-compiled binary)
```

### Step 0.2 — Write SPEC.md

Use the `write-spec` skill. You have two options:

**Option A — In Claude Chat or Claude Code on the web.** Open a session and
say:

> Follow the `livery/skills/write-spec.md` skill. Walk me through writing a
> SPEC.md for [brief description of your project]. Work through each step
> one at a time.

The skill will guide you through eight steps: problem statement, user
persona, feature list, non-feature list, success criteria, risk register,
non-negotiable constraints, and the review gate. It will push back if your
answers are vague. This is intentional.

**Option B — Write it yourself.** Read `livery/skills/write-spec.md` and
draft each section. Then ask Claude to evaluate the draft against the
acceptance criteria.

### Step 0.3 — Save and commit

Save the output as `SPEC.md` at your repo root:

```bash
# copy or paste the SPEC.md content into the file
git add SPEC.md livery .gitmodules
git commit -m "Phase 0: Add Livery submodule and SPEC.md"
git push
```

### Phase 0 gate

Before proceeding, confirm:

- [ ] `SPEC.md` exists with all eight sections
- [ ] The non-feature list covers every plausible assumption
- [ ] Every success criterion is mechanically checkable
- [ ] Every risk has an isolation strategy

---

## Phase 1 — Architecture

> *Define the crate structure, dependency graph, and public API contracts
> before writing any code.*

### Step 1.1 — Open a Claude Code session

Go to Claude Code on the web. Point it at your GitHub repo. In your
opening prompt, say:

```
## Phase 1 — Architecture

**Read first:**
1. Run `git submodule update --init --recursive`
2. Read `livery/CLAUDE-base.md` in full
3. Read `SPEC.md`
4. Read `livery/standards/ousterhout.md` (the design standard)

**Task:**
Write `ARCHITECTURE.md` for this project. It must include:

1. A Crate Map — one entry per crate with: name, responsibility
   (one sentence), what complexity it hides, and its public API stub
   (concrete Rust function signatures, not prose descriptions).

2. A Dependency Graph — a Mermaid diagram showing which crates depend
   on which. Library crates do not depend on the CLI crate.

3. An Information Hiding Inventory — for each crate, what internal
   details are hidden from callers.

4. ADRs (Architectural Decision Records) — one per major design choice.
   Each ADR has: Context, Decision, Consequences.

5. Run the Design Process Checklist from `livery/standards/ousterhout.md`
   against the architecture and record the results.

**Gate checklist — do not declare this phase complete until ALL of these
are present in ARCHITECTURE.md:**
- [ ] Every crate has a concrete public API stub with Rust function
      signatures (not prose descriptions)
- [ ] The Mermaid dependency graph exists and is valid
- [ ] The Information Hiding Inventory has one entry per crate
- [ ] At least one ADR exists
- [ ] The Ousterhout Design Process Checklist has been run and results
      recorded
- [ ] The CLI crate contains argument parsing and dispatch only —
      no business logic

**Hard constraint:** Do not write any implementation code. This phase
produces only ARCHITECTURE.md.
```

### Step 1.2 — Review the output

Claude Code may time out during long sessions and silently skip steps.
Check the gate checklist yourself:

- Open `ARCHITECTURE.md` in the repo
- Verify every checkbox in the gate checklist above is actually present
- If anything is missing, open a follow-up session asking Claude Code
  to complete the missing sections specifically

### Step 1.3 — Commit

```bash
git add ARCHITECTURE.md
git commit -m "Phase 1: Architecture document"
git push
```

### Phase 1 gate

- [ ] `ARCHITECTURE.md` exists with all five sections
- [ ] Public API stubs are concrete Rust signatures
- [ ] Mermaid dependency graph is present and valid
- [ ] Ousterhout design checklist has been run

---

## Phase 2 — Project Constitution

> *Set up the rules, validation pipeline, and project-specific files so
> that every future session starts from the same baseline.*

### Step 2.1 — Create CLAUDE.md

This is the most important file in the project. Claude Code reads it at
the start of every session. Use this template — copy it, fill in the
bracketed sections:

```markdown
# CLAUDE.md — [Project Name]

## MANDATORY PRE-FLIGHT (do this before ANY other action)

1. Run `git submodule update --init --recursive` to ensure `livery/`
   is populated. If the directory is empty, do NOT proceed.
2. Read `livery/CLAUDE-base.md` in full. It is the base constitution.
3. Read `SPEC.md` for what this project does and does not do.
4. Read `ARCHITECTURE.md` for the crate structure and boundaries.
5. Read the last 2–3 entries of `SESSIONS.md` (if it exists).

> **Extends:** `livery/CLAUDE-base.md` — that file governs design
> philosophy, TDD workflow, quality gates, and coding standards. This
> file adds project-specific rules. Where the two conflict, this file
> wins.

---

## Reference Documents

| Document | Path |
|---|---|
| Base constitution | `livery/CLAUDE-base.md` |
| Design standards | `livery/standards/ousterhout.md` |
| Readability standards | `livery/standards/readable-code.md` |
| Rust standards | `livery/standards/rust-specifics.md` |
| Specification | `SPEC.md` |
| Architecture | `ARCHITECTURE.md` |
| Session log | `SESSIONS.md` |

---

## Crate Responsibilities

[Copy the crate map from ARCHITECTURE.md here — one line per crate
with its responsibility. Example:]

- `myproject-cli` — CLI argument parsing and subcommand dispatch only.
  No business logic.
- `myproject-core` — [what it does]

---

## Project-Specific Constraints

[List hard constraints from SPEC.md §7. Example:]

- Must compile on stable Rust
- Must produce all outputs with no network access
- Must not panic on malformed input

---

## Session Contract Commands

Every session ends by running these in order:

```bash
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings
scripts/validate.sh
```

---

## Project-Specific Anti-Patterns

[Add as they emerge during development. Examples:]

- Do not add external CDN dependencies in generated HTML
- Do not add traits with only one implementor

---

## Key Types

[Add after Phase 3 sessions establish the core types. Leave empty
for now.]
```

### Step 2.2 — Create the validation script

```bash
mkdir -p scripts
cat > scripts/validate.sh << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "=== Formatting ==="
cargo fmt --check

echo "=== Linting ==="
cargo clippy --workspace -- -D warnings

echo "=== Tests ==="
cargo test --workspace

echo "=== Prism quality gate ==="
if [ -x livery/bin/prism ]; then
    livery/bin/prism check . --strict --no-deps --no-coverage
else
    echo "WARNING: livery/bin/prism not executable. Run prism check manually."
fi

echo "=== All gates passed ==="
EOF
chmod +x scripts/validate.sh
```

### Step 2.3 — Create the workspace Cargo.toml

Based on your `ARCHITECTURE.md` crate map, create the workspace. Example
structure:

```bash
mkdir -p crates/myproject-cli/src
mkdir -p crates/myproject-core/src
```

Create the root `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/myproject-cli",
    "crates/myproject-core",
]
resolver = "2"

[workspace.dependencies]
myproject-core = { path = "crates/myproject-core" }
anyhow = "1"
thiserror = "2"
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Create minimal `Cargo.toml` and `lib.rs`/`main.rs` for each crate so the
workspace compiles. Don't write implementation — just enough to pass
`cargo check`.

### Step 2.4 — Create prism.toml

```toml
[quality]
min_doc_coverage = 80
max_cyclomatic = 20
max_cognitive = 30
max_fn_lines_warn = 100
max_fn_lines_fail = 200

[testing]
min_test_ratio = 1.0
require_integration_tests = true

[safety]
max_unsafe_blocks = 0
```

### Step 2.5 — Copy the project standards template

```bash
mkdir -p standards
cp livery/standards/project.md.template standards/project.md
```

Leave it empty for now. Add project conventions as they emerge.

### Step 2.6 — Create the skills directory

```bash
mkdir -p skills
```

You'll add project-specific skills as you need them. Common first skills:

- `skills/add-subcommand.md` — how to add a CLI subcommand for this project
- `skills/write-proptest.md` — property test patterns for your core types

These can be written during early Phase 3 sessions when the patterns
become clear.

### Step 2.7 — Verify everything compiles

```bash
cargo check --workspace
cargo test --workspace     # will pass trivially with no tests yet
cargo fmt --check
cargo clippy --workspace -- -D warnings
```

All four must pass. If they don't, fix the issues before proceeding.

### Step 2.8 — Commit

```bash
git add .
git commit -m "Phase 2: Project constitution, workspace scaffold, validation pipeline"
git push
```

### Phase 2 gate

- [ ] `CLAUDE.md` exists with the mandatory pre-flight block
- [ ] `scripts/validate.sh` exists and is executable
- [ ] `prism.toml` exists with quality thresholds
- [ ] `standards/project.md` exists (can be empty)
- [ ] `skills/` directory exists
- [ ] The workspace compiles (`cargo check --workspace`)
- [ ] All four validation commands pass

---

## You're Ready — Phase 3 Session Execution

Your project is now fully constituted. Every future Claude Code session
follows the same pattern. Use this template for your opening prompt:

```
## Session [N] — [Date]

**Scope:** [One sentence. What is being built or fixed.]

**Entry state:** [What exists. What is passing. E.g., "Empty workspace
scaffold, all gates pass, no implementation yet."]

**Read first:**
1. Run `git submodule update --init --recursive`
2. `livery/CLAUDE-base.md`
3. `CLAUDE.md`
4. `ARCHITECTURE.md` §[relevant crate]
5. `SESSIONS.md` (last 2–3 entries, if it exists)

**Task:**
[The specific implementation task. Reference the ARCHITECTURE.md public
API stub. Reference a skill if applicable:
"Follow `livery/skills/add-crate.md`"]

**Stopping condition:** This session is complete when:
- [ ] [Specific deliverable 1]
- [ ] [Specific deliverable 2]
- [ ] All tests pass (`cargo test --workspace`)
- [ ] `scripts/validate.sh` exits 0
- [ ] `livery/skills/review-for-red-flags.md` completed
- [ ] `SESSIONS.md` updated with this session's entry

**Hard constraints:**
- Follow `livery/CLAUDE-base.md` and `CLAUDE.md` in full.
- Do not implement anything outside this session's scope.
- Do not modify public APIs without updating ARCHITECTURE.md.
```

### First session tips

Your first Phase 3 session should be small and concrete. Good first
sessions:

- Implement a single crate's error type and one public function, with
  tests
- Add the CLI argument parsing and one subcommand that prints "not yet
  implemented"
- Implement the simplest end-to-end path through the application

Bad first sessions:

- "Implement the whole project"
- "Set up the infrastructure" (that's what Phase 2 just did)
- Anything that touches more than two crates

---

## Quick Reference — File Map

After completing Phases 0–2, your repo should look like this:

```
YOUR_PROJECT/
├── SPEC.md                  # Phase 0: what you're building
├── ARCHITECTURE.md          # Phase 1: how it's structured
├── CLAUDE.md                # Phase 2: agent constitution
├── SESSIONS.md              # Phase 3+: created in first session
├── Cargo.toml               # workspace root
├── prism.toml               # quality thresholds
├── standards/
│   └── project.md           # project-specific conventions
├── skills/                  # project-specific skills (added as needed)
├── scripts/
│   └── validate.sh          # validation pipeline
├── crates/
│   ├── myproject-cli/       # CLI crate
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── myproject-core/      # library crate(s)
│       ├── Cargo.toml
│       └── src/lib.rs
├── livery/                  # git submodule — do not edit
│   ├── CLAUDE-base.md
│   ├── WORKFLOW.md
│   ├── standards/
│   ├── skills/
│   ├── feedback/
│   ├── docs/
│   └── bin/prism
└── .gitmodules
```

---

## Troubleshooting

**Livery directory is empty in Claude Code.** Claude Code clones the
repo but does not automatically initialize submodules. Your `CLAUDE.md`
pre-flight block tells the agent to run
`git submodule update --init --recursive`. If this fails, the agent
should report the error and not proceed.

**Claude Code times out and skips steps.** Every phase prompt above
includes a gate checklist. After the session, manually verify each
checkbox. If items are missing, open a follow-up session asking for
the specific missing items — not a redo of the entire phase.

**`livery/bin/prism` is not executable.** The pre-compiled binary is
built for Linux x86_64 (Claude Code's sandbox architecture). If you're
running locally on a different platform, install Prism from source:
`cargo install --path crates/prism-cli` (from the Prism repo). The
`validate.sh` script already handles the case where the binary is
missing.

**"Read this file first" is ignored by the agent.** This is why the
`CLAUDE.md` template uses a `MANDATORY PRE-FLIGHT` header with
numbered steps instead of a passive note. If the agent still skips it,
start your session prompt with an explicit instruction:
"Before doing anything else, read `livery/CLAUDE-base.md` in full and
confirm you have done so."