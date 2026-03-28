# QUICKSTART.md — From Blank Repo to First Coding Session

> **What this is:** Start a new Rust project with Livery, Prism, and
> Claude Code. Follow this once per project. When you're done, you'll
> be ready to start coding sessions.
>
> **Time:** Two conversations (spec + architecture) and one command.

---

## The Full Path

```
1. Create repo, clone it                     ← 2 minutes
2. Install Livery                            ← copy-paste, 10 seconds
3. Write SPEC.md (Claude Code session)       ← 30–60 minutes
4. Write ARCHITECTURE.md (Claude Code session) ← 30–60 minutes
5. Run livery/bin/scaffold                   ← 10 seconds
6. Review, commit, push                      ← 5 minutes
7. Start coding sessions (Phase 3)           ← ready
```

---

## Step 1 — Create and clone your repo

Create a new repository on GitHub (initialize it with at least a README
so it's not empty). Clone it locally:

```bash
git clone https://github.com/YOUR_USER/YOUR_PROJECT.git
cd YOUR_PROJECT
```

## Step 2 — Install Livery

Run this in your project root:

```bash
git submodule add https://github.com/jeffcwolf/livery livery && git submodule update --init --recursive
```

Verify it worked:

```bash
ls livery/CLAUDE-base.md    # should exist
ls livery/bin/prism          # should exist
```

## Step 3 — Write SPEC.md (Phase 0)

This is a conversation, not code. Open a Claude Code web session (or
Claude Chat) and say:

> Follow the `livery/skills/write-spec.md` skill. Walk me through
> writing a SPEC.md for [brief description of your project]. Work
> through each step one at a time.

The skill guides you through eight steps: problem statement, user
persona, feature list, non-feature list, success criteria, risk
register, non-negotiable constraints, and the review gate. It will
push back if your answers are vague. This is intentional — vagueness
here becomes scope creep later.

Save the output as `SPEC.md` in your project root.

**Phase 0 gate — confirm before proceeding:**

- [ ] `SPEC.md` exists with all eight sections
- [ ] Every success criterion is mechanically checkable
- [ ] Every risk has an isolation strategy

## Step 4 — Write ARCHITECTURE.md (Phase 1)

Open another Claude Code session and say:

> Follow the `livery/skills/write-architecture.md` skill. Read
> `SPEC.md` first, then walk me through designing the architecture
> one step at a time.

The skill guides you through seven steps: crate map, dependency graph,
public API stubs, information hiding inventory, ADRs, design checklist
audit, and the review gate. It proposes designs; you approve or revise.

Save the output as `ARCHITECTURE.md` in your project root.

**Important:** The crate map must use the exact table format the skill
specifies — `scaffold` parses it to generate your workspace:

```markdown
## Crate Map

| Crate | Type | Responsibility |
|---|---|---|
| myproject-cli | bin | CLI argument parsing and dispatch |
| myproject-core | lib | Core analysis engine |
```

**Phase 1 gate — verify these are present in ARCHITECTURE.md:**

- [ ] Crate Map table with the three-column format above
- [ ] Mermaid dependency graph
- [ ] Public API stubs with concrete Rust function signatures
- [ ] Information hiding inventory
- [ ] At least one ADR
- [ ] Design checklist results

**Tip:** Claude Code may time out and silently skip steps. Check each
checkbox yourself. If anything is missing, open a follow-up session
asking for the specific missing sections.

## Step 5 — Run scaffold (Phase 2)

One command generates everything:

```bash
livery/bin/scaffold
```

This reads your `SPEC.md` and `ARCHITECTURE.md` and creates:

| File | Purpose |
|---|---|
| `CLAUDE.md` | Agent constitution with mandatory pre-flight |
| `Cargo.toml` | Workspace with all crates from your architecture |
| `crates/*/` | Per-crate scaffold (Cargo.toml, stub src files) |
| `scripts/validate.sh` | Validation pipeline (fmt, clippy, test, prism) |
| `prism.toml` | Quality thresholds |
| `standards/project.md` | Project-specific conventions (empty, fill as needed) |
| `skills/` | Project skills directory (empty, add as needed) |
| `SESSIONS.md` | Session log (empty, first entry written in Phase 3) |
| `.gitignore` | Standard Rust + Livery ignores |

Preview what it will create without writing anything:

```bash
livery/bin/scaffold --dry-run
```

## Step 6 — Review, commit, push

1. **Review `CLAUDE.md`** — confirm the crate responsibilities and
   constraints are accurate. Edit if needed.

2. **Verify it compiles:**

   ```bash
   cargo check --workspace
   ```

3. **Commit everything:**

   ```bash
   git add .
   git commit -m "Phases 0–2: Spec, architecture, project constitution"
   git push
   ```

## Step 7 — Start coding (Phase 3)

Your project is fully constituted. Every Claude Code session follows
the same template. Copy this into your session opening prompt:

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
- Create REFACTOR-EVIDENCE.md during refactoring (three passes).
- Run `scripts/validate.sh` at session end — all 9 gates must pass.
```

### First session tips

Good first sessions (small, concrete):

- Implement one library crate's error type and first public function,
  with tests
- Add CLI argument parsing and one subcommand that prints "not yet
  implemented"
- Implement the simplest end-to-end path through the application

Bad first sessions:

- "Implement the whole project"
- "Set up the infrastructure" (that's what scaffold just did)
- Anything touching more than two crates

---

## File Map After Setup

```
YOUR_PROJECT/
├── SPEC.md                  Phase 0 — what you're building
├── ARCHITECTURE.md          Phase 1 — how it's structured
├── CLAUDE.md                Phase 2 — agent constitution (generated)
├── SESSIONS.md              Phase 3+ — session log (generated, empty)
├── Cargo.toml               Workspace root (generated)
├── prism.toml               Quality thresholds (generated)
├── standards/
│   └── project.md           Project conventions (generated, empty)
├── skills/                  Project skills (generated, empty)
├── scripts/
│   └── validate.sh          Validation pipeline (generated)
├── crates/
│   ├── myproject-cli/       Binary crate (generated)
│   └── myproject-core/      Library crate(s) (generated)
├── livery/                  Git submodule — do not edit
│   ├── CLAUDE-base.md
│   ├── WORKFLOW.md
│   ├── QUICKSTART.md        This file
│   ├── standards/
│   ├── skills/
│   ├── feedback/
│   ├── docs/
│   └── bin/
│       ├── prism
│       └── scaffold
└── .gitmodules
```

---

## Updating Livery

Livery is pinned to a specific commit. To pull the latest version,
run this from your project root (not from inside `livery/`):
```bash
git submodule update --remote livery && git add livery && git commit -m "Update livery to latest"
```

---


## Troubleshooting

**Livery directory is empty in Claude Code.** Claude Code clones repos
but does not initialize submodules automatically. The generated
`CLAUDE.md` includes an explicit pre-flight instruction to run
`git submodule update --init --recursive`. If the agent skips it,
start your prompt with: "Before doing anything else, run
`git submodule update --init --recursive` and confirm it succeeded."

**Scaffold fails to parse ARCHITECTURE.md.** The crate map table must
use the exact format the `write-architecture` skill specifies: three
columns (`Crate`, `Type`, `Responsibility`), pipe-delimited. If
scaffold can't find it, check the section heading is `## Crate Map`
and the table starts on the line after the separator row.

**Claude Code times out and skips steps.** Every phase in this guide
includes a gate checklist. After each Claude Code session, manually
verify the checkboxes. If items are missing, open a follow-up session
asking for the specific missing sections — not a redo of the entire
phase.

**`livery/bin/prism` is not executable.** The pre-compiled binary is
built for Linux x86_64 (Claude Code's sandbox). For local use on
macOS, install from source: `cargo install --path crates/prism-cli`
(from the Prism repo). The `validate.sh` script handles the missing
binary gracefully.

**`cargo check` fails after scaffold.** Usually a Rust edition issue.
The scaffold generates `edition = "2024"` — if your Rust toolchain
is older, change it to `"2021"` in each `Cargo.toml` or run
`rustup update stable`.