# standards/ — Layered Standards Directory

Standards are loaded at session start and apply throughout. They are
layered with clear precedence:

```
Project standards  (project repo: standards/project.md)   — wins over all
User standards     (livery/standards/user.md)              — wins over shipped
Shipped standards  (livery/standards/language-rust.md,     — defaults
                    livery/standards/ousterhout.md,
                    livery/standards/arc.md)
```

Later layers override earlier layers where they conflict. Where they
don't conflict, all layers apply simultaneously.

## Shipped Standards

These ship with Livery and encode the methodology's core standards.
They are extracted from CLAUDE-base.md for modularity — CLAUDE-base.md
references this directory rather than inlining all standards.

| File | Content |
|---|---|
| `language-rust.md` | Rust-specific code standards: error handling, type system, naming conventions, code organisation |
| `ousterhout.md` | Ousterhout's design principles: deep modules, information hiding, strategic programming |
| `arc.md` | ARC naming and expression criteria: misunderstanding test, banned words, three-pass refactoring |

## User Standards

`user.md` — optional, user-specific conventions that apply to all your
projects. Edit this file directly. See the file header for guidance.

## Project Standards

`project.md` lives in the project repo (not in the Livery submodule)
at `standards/project.md`. It contains project-specific conventions.
A template is provided here for reference; copy it to your project.