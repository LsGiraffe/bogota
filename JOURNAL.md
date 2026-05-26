# Journal

Learning log for the bogota project — Rust, AI, and nand2tetris.

---

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

