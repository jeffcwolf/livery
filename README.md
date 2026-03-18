# Livery

A methodology for producing high-quality code with agentic coding tools.

> **Status: Active development. Interfaces change without notice. Use at your own risk.**
> See [Stability](#stability) below before depending on this in your own projects.

---

## What this is

Livery is a structured approach to using AI coding agents — primarily Claude Code —
to write, review, and ship production-quality software. It is not a framework or a
library. It is a set of documents, conventions, and discipline that you embed in a
project repo and hand to an agent at the start of every session.

The core idea is that agentic coding tools are powerful but stateless. Without a
persistent constitution — explicit standards, a recorded design rationale, a session
log, quality gates that run before the session closes — the agent optimises for
local coherence and loses track of the whole. Livery is the whole.

The methodology is built around four principles:

- **Design before code.** A `SPEC.md` and `ARCHITECTURE.md` exist before any
  implementation begins. The agent reads them at the start of every session.
- **TDD, strictly.** Red before green, always. Refactor after green, always.
  Property-based tests are mandatory for any function that transforms data.
- **Measurable quality gates.** Every session closes against a defined checklist.
  "Mostly passing" is not passing.
- **Session continuity.** Every decision, every deviation, every Prism baseline delta
  is recorded in `SESSIONS.md`. A session six months later starts from a complete
  picture, not just source code.

---

## What's in this repo

CLAUDE-base.md        The base agent constitution.

WORKFLOW.md           The standard phase sequence: Spec → Architecture →
                      Setup → Session Execution → Release.

conversion.md         Workflow deviations for conversion projects.

adapter-superpowers.md  Runtime adapter: maps Superpowers to Livery's
                        runtime interface.

context-management.md   Optional context window protocol. Activated when
                        context exhaustion patterns appear.

standards/            Coding standards referenced by CLAUDE-base.md.
  ousterhout.md       Ousterhout's principles as executable rules.
  readable-code.md    ARC naming, commenting, and control flow rules.
  rust-specifics.md   Rust-specific applications of both.
  user.md             User-specific conventions (optional).
  project.md.template Template for project-specific conventions.

skills/               Reusable skill files invoked by the agent.
  session-open.md     Open a session: verify baseline, load context.
  review-for-red-flags.md   Design audit against Ousterhout's Red Flags.
  review-docs.md      Documentation quality review.
  naming-review.md    Naming audit against ARC criteria.
  run-validation.md   Complete validation pipeline.
  update-architecture.md  Keep ARCHITECTURE.md honest after changes.
  add-crate.md        Add a new workspace crate.
  bug-fix.md          Reproduce → red test → fix → green.

feedback/             Self-correction system.
  feedback-loop.md    Pattern detection and proposal protocol.
  enforcement.md      Rule escalation register.
  known-patterns.md   Recurring patterns observed across sessions.
  proposals/          Proposed changes awaiting human review.

docs/                 Rationale documents (human-facing).
  INDEX.md            System map and navigation guide.
  (6 rationale files)

bin/                  Pre-compiled tooling.
  prism               Automated quality gate binary.
  README.md           Build and rebuild instructions.

---

## How to use it

The intended use is as a git submodule inside your project repo:

```bash
git submodule add https://github.com/jeffcwolf/livery livery
git submodule update --init --recursive
```

Then create a project-specific `CLAUDE.md` at your repo root that begins:

```markdown
> **Extends:** `livery/CLAUDE-base.md` — read that file first, then this one.
```

At the start of every Claude Code session, the agent reads `livery/CLAUDE-base.md`
followed by your project's `CLAUDE.md`. The session prompt tells it which skill
files are relevant.

---

## Reference implementation

[`mint`](https://github.com/jeffcwolf/mint) is a Rust CLI for publishing research
software as citable scholarly artifacts. It is being built using Livery from the
ground up, and is the primary reference implementation of this methodology. The
`mint` repo contains the full session prompt sequence, `SESSIONS.md` logs, and a
controlled comparison document measuring the methodology's effect against a
baseline tool built without it.

---

## Stability

**This repository is under active development.** Specifically:

- File names, paths, and section headings in `CLAUDE-base.md` and the skill files
  may change between commits. If you depend on this as a submodule, pin to a commit
  hash rather than a branch.
- The methodology itself is evolving as it is tested against real projects. Practices
  that prove unhelpful will be removed; practices that prove essential will be added.
- There is no versioning scheme yet. This will be addressed when the methodology
  stabilises.
- No backwards-compatibility guarantees are made at this stage.

**Use at your own risk.** This is shared in the spirit of working in public, not as
a finished product. If you adopt it and something breaks or produces bad output, that
is on you. That said, issues and feedback are welcome.

---

## License

MIT. See `LICENSE`.

---

## Author

Jeff Wolf. I'm sharing this because the methodology is not proprietary and working
in public tends to produce better work. If you find it useful or have improvements,
open an issue or a pull request.