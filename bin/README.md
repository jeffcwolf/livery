# livery/bin/ — Pre-compiled Tooling

This directory contains pre-compiled binaries used by the Livery system
during sessions. The agent runs these directly — no Rust toolchain or
network access required.

## prism

The Prism binary is Livery's automated quality gate. It measures code
quality metrics (cyclomatic complexity, cognitive complexity, module
depth, doc coverage, test ratios, dependency health) and checks them
against thresholds defined in the project's `prism.toml`.

**Source:** https://github.com/jeffcwolf/prism

**Target:** Linux x86_64, statically linked.

**Used by:**
- The agent at session start: `livery/bin/prism stats . --json`
- The agent at session end: `livery/bin/prism check . --strict`
- See CLAUDE-base.md §Automated Quality Gate Protocol for the full
  sequence.

## Rebuilding

Rebuild when Prism's source changes. This is infrequent.

On a Linux machine:

```bash
cd ~/code/prism
cargo build --release
strip target/release/prism
cp target/release/prism ~/code/livery/bin/prism
chmod +x ~/code/livery/bin/prism
```

Cross-compiling from macOS:

```bash
cd ~/code/prism
cargo install cross    # if not already installed
cross build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/prism ~/code/livery/bin/prism
chmod +x ~/code/livery/bin/prism
```

Commit the binary directly. A stripped Prism binary is typically 8–12MB.

## If the binary doesn't work

If the agent cannot execute `livery/bin/prism` (wrong architecture,
permissions, missing file), it follows the fallback protocol:

1. Report the error to the human.
2. List the Prism commands that need to be run manually.
3. Leave `[PRISM: manual]` placeholders in the SESSIONS.md entry.
4. Do not declare the session complete until Prism data is recorded.

See `adapter-superpowers.md` §7.4 or CLAUDE-base.md §Automated Quality
Gate Protocol for the full fallback procedure.