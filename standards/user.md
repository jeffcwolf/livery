# user.md — User-Specific Standards

> **Status: optional.** This file is for conventions specific to you as a
> developer, independent of any particular project. Leave it empty if you
> have no user-level preferences beyond what CLAUDE-base.md provides.
>
> **Scope:** Preferences that apply to all your projects — coding style
> choices, tool preferences, naming conventions, documentation habits.
> Project-specific rules belong in `project/standards/project.md` instead.
>
> **Precedence:** User standards extend the Livery defaults. Where they
> conflict with shipped standards (language-rust.md, ousterhout.md, arc.md),
> user standards win. Where they conflict with project standards
> (project.md), project standards win.

---

## How to Use This File

Add your conventions as sections below. Examples of what belongs here:

- "I prefer `expect()` with a message over `unwrap()` even in test code"
- "All my projects use `tracing` instead of `log`"
- "I write commit messages in imperative mood, max 50 chars for the
  subject line"
- "I prefer French-language doc comments" (or any language preference)
- "I use 2-space indentation in TOML files"

Examples of what does **not** belong here (use project.md instead):

- "This project uses `reqwest` instead of `ureq`"
- "The `api` crate must not depend on the `cli` crate"
- "Test fixtures live in `tests/fixtures/`, not alongside source"

---

<!-- Add your conventions below this line -->