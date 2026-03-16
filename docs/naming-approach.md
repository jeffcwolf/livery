# Naming Approach — Rationale and Enforcement

> **The short version:** Names are the primary interface between code and the human
> (or agent) reading it. A name that can be misunderstood will be misunderstood. Naming
> difficulty is not a vocabulary problem — it is a design signal.

---

## The Core Principle

Every name in code is a claim about what the named thing does, is, or contains. A name
that is vague, ambiguous, or misleading causes the reader to build a wrong mental model
— and a wrong mental model causes bugs. In an agentic codebase, names are particularly
important because agents in future sessions read names as specifications. An agent that
encounters `process_data` will infer that it processes data; it will not infer the
specific transformation that was intended. That inference may be wrong, and the agent
will write code based on the wrong inference.

The primary test for any name is: **could it be misunderstood?** Not "is it clear to
me right now" — but "could a reasonable programmer, reading this without context, assign
it a different meaning than I intend?" If yes, rename. The cost of renaming is measured
in seconds. The cost of a misunderstood name compounds across every future reader and
every future session.

---

## Why This Approach

### Why ARC's naming principles specifically

Boswell & Foucher's *The Art of Readable Code* was chosen for naming because it
provides the most operationally specific guidance available. The core test — "could
this name be misunderstood?" — is a test that can be applied mechanically to any name.
Other naming philosophies state preferences ("be descriptive", "avoid abbreviations")
that are too vague to enforce consistently.

ARC's specific contributions:
- The misunderstanding test (primary test for every name)
- The banned-word list (`Manager`, `Handler`, `data`, `info`, etc.) — words that are
  always wrong because they describe almost nothing
- The scope-proportionality principle — short names for short scopes, precise names
  for wide scopes
- The important-information attachment rule — units, encodings, and states belong in
  names or types

### Why naming difficulty is a design signal

This is the connection that makes the naming approach more than a style guide. When
you cannot name a function clearly, the most likely cause is not a vocabulary
limitation — it is that the function has more than one responsibility, or that its
purpose is not well-defined, or that it occupies no coherent place in the design.

The convergence with Ousterhout is precise: his "hard to describe" Red Flag and ARC's
naming difficulty signal are the same observation from different angles. Ousterhout
says: if you cannot describe what a module does in one sentence without "and", it has
two responsibilities. ARC says: if you cannot find a clear, precise name for a
function, the function is doing too much. Both resolve the same way: split the function,
clarify the responsibility, then the name will follow.

This is why the naming-as-design-signal rule is active throughout every session, not
just during the naming review at the end. When a name is difficult to choose, the
right response is to consult `livery/standards/ousterhout.md` before choosing a compromise
name. A compromise name on a poorly-designed function leaves the design problem intact
while giving the impression it has been addressed.

### Why Rust-specific naming conventions matter

Rust has a strong community convention set that readers rely on for semantic
understanding. Violating these conventions — `new()` that can fail, `as_xxx()` that
allocates, a struct named `XxxData` rather than just `Xxx` — creates a cognitive
mismatch between what the convention predicts and what the code delivers. This is a
specific form of the misunderstanding test failure: the reader's prior knowledge of
Rust conventions leads them to a wrong understanding.

The `livery/standards/rust-specifics.md` naming section encodes these conventions as rules,
not preferences, because an agent that violates them creates code that is subtly
surprising to every Rust programmer who reads it.

---

## How It's Encoded

| Concern | Where encoded | How enforced |
|---|---|---|
| Misunderstanding test | `livery/standards/readable-code.md` Part I | `livery/skills/naming-review.md` Step 2; Refactor Pass 2 |
| Banned words | `livery/standards/readable-code.md` Part I | `livery/skills/naming-review.md` Step 3 |
| Scope proportionality | `livery/standards/readable-code.md` Part I | `livery/skills/naming-review.md` Step 4 |
| Boolean naming (`is_`, `has_`) | `livery/standards/readable-code.md` Part I | `livery/skills/naming-review.md` Step 5; `livery/standards/rust-specifics.md` |
| Important-information attachment | `livery/standards/readable-code.md` Part I | `livery/skills/naming-review.md` Step 6 |
| Naming as design signal | `livery/standards/readable-code.md` Part V; `livery/CLAUDE-base.md` Refactor Pass 2; `livery/skills/session-open.md` Step 4 | Active throughout every session |
| Rust conversion conventions (`as_/to_/into_`) | `livery/standards/rust-specifics.md` Part IV | `livery/skills/naming-review.md` Step 5 |
| `new()` infallibility | `livery/standards/rust-specifics.md` Part IV | `livery/skills/naming-review.md` Step 5 |
| Test function naming | `livery/standards/rust-specifics.md` Part IV | Test review in `livery/skills/review-for-red-flags.md` |
| Concrete over abstract names | `livery/standards/readable-code.md` Part I | Refactor Pass 2; `livery/skills/naming-review.md` Step 2 |

---

## How It's Enforced in Practice

### During Refactor Pass 2 (within every TDD cycle)

The second pass of the TDD Refactor phase is ARC names. Every name written during
Green is reviewed against the misunderstanding test. Green phase names are often
placeholders (`result`, `val`, `data`, `check`) — Refactor Pass 2 is when they are
replaced with precise, domain-specific names.

The critical rule in Pass 2: if you cannot find a clear name without "and" or a vague
word, do not choose a compromise — return to Pass 1 and examine the design. Naming
difficulty at this stage is a design signal, not a vocabulary problem.

### Session-wide naming signal monitoring

The `session-open` skill makes the naming-as-design-signal rule explicit from the
start of every session. At any point during a session, if naming is difficult:

1. Write the one-sentence description of what the item does
2. If it contains "and" → consult `livery/standards/ousterhout.md` and split
3. If only a vague word fits → ask "what exactly does this hide?" (deep-module test)
4. Only if 1–3 do not apply → invent a precise domain-specific name and document it

### Session-end `naming-review` skill

Six steps:
1. Collect all new names introduced this session
2. Misunderstanding test on every name
3. Banned-word scan
4. Scope-proportionality check
5. Rust-specific convention check (`as_/to_/into_`, `new()`, boolean prefixes)
6. Important-information attachment check

Any rename found is applied before the session closes. Renames are listed in the
`<project>/SESSIONS.md` entry so future sessions see the history.

---

## The Banned Words — Rationale for Each

These words are banned because they describe almost nothing:

| Word | Why banned | What to use instead |
|---|---|---|
| `Manager` | Implies ownership and control but specifies neither the domain nor the operations | The specific operations: `TokenStore`, `DepositRegistry`, `ConfigResolver` |
| `Handler` | Implies response to an event but specifies neither the event nor the response | The specific action: `dispatch`, `reject`, `forward`, `record` |
| `Processor` | Implies transformation but specifies neither the input, output, nor transformation | The specific transformation: `CitationTransformer`, `ArchiveBuilder` |
| `Helper` | Implies support but specifies nothing | The specific support: `OrcidValidator`, `FileSorter` |
| `Utils` | A miscellany module — if it exists, the items in it have no coherent home | Move each item to the module it belongs to |
| `data` | Every variable contains data | Name what the data represents: `metadata`, `payload`, `record` |
| `info` | Every struct contains information | Name what the information describes: `statistics`, `report`, `summary` |
| `get` | Ambiguous about whether it fetches from I/O, computes, or reads from memory | `fetch` (I/O), `compute` (calculation), `read` (memory/field access) |
| `check` | Ambiguous about what is verified and what the result means | `validate` (with error), `verify` (with bool), `detect` (pattern matching) |
| `process` | Describes the activity but not the transformation | The specific transformation verb: `normalise`, `archive`, `transform`, `apply` |
| `flag` | A boolean with no domain meaning | A boolean named for its domain meaning: `is_sandbox`, `has_failures` |

---

## What Failure Looks Like

**The `process_data` function.** Present in almost every agentic codebase that lacks
naming discipline. What does it process? What is the data? What does it produce?
Three questions with no answers. An agent in a future session will call this function
when it should not, or avoid it when it should call it, because the name provides
no signal about when it is appropriate.

**The `UserManager` struct.** What does it manage? What operations does it expose?
What invariants does it maintain? The name answers none of these questions. It could
be a database layer, a validation service, an event coordinator, or all three. The
name forces every reader to examine the entire type to understand its purpose — work
that a precise name would have made unnecessary.

**The `is_valid: bool` that means "is invalid".** A boolean named `is_valid` whose
true value means the validation failed. This happens when a function is renamed
without updating its return value semantics. The naming review's boolean-naming rule
(`is_`, `has_`, `should_` for true-positive booleans) catches this — `is_valid` must
mean "validation passed" in this system.

**The `result` variable at wide scope.** A local variable named `result` is acceptable
if it spans three lines. A field named `result` in a struct is never acceptable — it
describes nothing about what the field contains. Every field in a struct has a name;
that name should describe the domain concept the field represents.

---

## Connection to Other Concerns

**Design.** Naming difficulty is the most reliable early signal of a design problem.
When a function cannot be named clearly, the design audit should be triggered
immediately — before a compromise name is chosen. See `docs/design-philosophy.md`,
Ousterhout's "hard to describe" Red Flag.

**Documentation.** The ARC rule "do not comment bad names — fix the names" creates a
direct connection. A comment that exists to explain what a variable name means is
evidence that the name should be replaced. Naming and documentation quality are
audited in adjacent Refactor passes (Pass 2 and Pass 3) for this reason. See
`docs/documentation-approach.md`.

**Testing.** Test function names are specifications. A test named
`parse_citation_rejects_missing_title` is a precise claim about the module's
behaviour. Weak test names (`test_parse_function`) are a form of vague naming — they
describe what is being tested but not what should be true. The naming approach applies
to test names as fully as to production code names.
