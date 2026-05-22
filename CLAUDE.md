# Bogota

A personal learning project combining three tracks:
- **Rust** — language training and systems programming
- **AI/LLM** — deep-diving into AI concepts and tooling
- **nand2tetris** — building a computer from logic gates up to understand the full stack

## Project layout

```
src/         Rust source code
JOURNAL.md   Development journal — dated entries on progress, insights, blockers
```

## Rust

- Edition: 2024
- Run: `cargo run`
- Test: `cargo test`
- Check: `cargo check`

## Journal

Use `/journal` to append a new dated entry to `JOURNAL.md`.

## Style

- Keep code simple; this is a learning context, so clarity beats cleverness
- Only commit when explicitly asked to

## Commits

Use conventional commits: `type(scope): description`
e.g. `feat(p1): implement NAND gate`, `fix(alu): correct half adder carry bit`

## Pull Requests

Short and pragmatic. Skip any section with nothing meaningful to say.

```
## What was built

One or two sentences: which component, which Rust constructs.

## What was learned

Only if something non-obvious clicked. Skip otherwise.

## How to verify

How to run or test it.
```
