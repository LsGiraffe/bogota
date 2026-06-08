# Journal

Learning log for the bogota project — Rust, AI, and nand2tetris.

---

## 2026-06-08

### What was worked on

Implemented NOT, AND, and OR gates in `src/gates/elementary.rs`, each built exclusively
from `nand()`. Three separate branches and PRs, one gate per cycle.

### Key insights

OR from NAND requires De Morgan's law: `OR(a, b) = NAND(NOT(a), NOT(b))`. The double
negation collapses the implicit AND inside NAND, leaving a pure OR. All three gates
follow naturally once you see that NAND is universal.

### Next steps

Continue with XOR, then move to MUX and DMUX in `mux.rs`.

## 2026-05-28

### What was worked on

Implemented `nand()` in `src/gates/elementary.rs` — truth table as a 4-case test first, then `!(a && b)`. TDD cycle confirmed: test panics on `todo!()`, passes after implementation.

### Key insights

`use super::*` in `#[cfg(test)]` modules is idiomatic Rust — the wildcard is scoped to a private, test-only block so the usual objections don't apply.

### Next steps

Build NOT, AND, OR, XOR in `elementary.rs` by composing calls to `nand()`.

## 2026-05-25

### What was worked on

Scaffolded the full nand2tetris module architecture in Rust: `src/gates/` with submodules
`elementary`, `mux`, `bus`, `mux_multi`. Updated `CLAUDE.md` with global project decisions —
pure Rust implementations, TDD throughout, `nand()` as the only primitive for stages 1–5.
Added GitHub Actions CI (fmt + build).

### Key insights

Mapping nand2tetris stages to Rust modules upfront enforces the dependency flow rule from
day one — each stage only reaches downward, mirroring how the hardware actually composes.

### Next steps

Implement `nand()` in `elementary.rs` — tests first.

## 2026-05-23

### What was worked on

Learned standard Rust project structure: crates, packages, module organisation, and `Cargo.toml` anatomy. Set up `src/lib.rs` as the library crate entry point with a `greeting()` function and co-located unit test, keeping `main.rs` as a thin binary wrapper.

### Key insights

Unit tests live in the same file as the code they test (`#[cfg(test)]`), not in `tests/` — that directory is for integration tests only. `main` itself is never unit tested; it stays thin by design.

### Next steps

Start nand2tetris Project 1 — boolean logic gates implemented in Rust.

## 2026-05-22

### What was worked on

Project setup: defined goals in the README, configured the `/journal` command to auto-synthesize entries from conversation history, and updated `CLAUDE.md` to commit only on explicit request.

### Next steps

Start nand2tetris Project 1 — boolean logic gates implemented in Rust.

