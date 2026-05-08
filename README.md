# REFAL-5 COMPILER (Bootstrapped)

This repository contains the Sovereign Specification Refal-5 deterministic compiler, optimized for the AA2 Vedic Astrology System's Validation Loop.

## Bootstrapping Architecture

Since REFAL-5 currently has no compiler here, we use a classic 3-stage bootstrapping approach:

### STAGE 0 — Seed Compiler (Rust)
Located in `/src/seed/`. This is a minimal REFAL-5 interpreter written in Rust. It does NOT support the full REFAL-5 spec, but has exactly enough features (pattern matching, e. variables with backtracking, basic I/O builtins) to interpret a basic REFAL-5 program.

**To build and run Stage 0:**
```bash
cd src/seed
cargo build --release
./target/release/seed <path_to_refal_file.ref>
```

### STAGE 1 — Real Compiler (REFAL-5)
Located in `/src/refal/`. This will be the full-featured REFAL-5 compiler written in REFAL-5 itself, encompassing a Lexer, Parser, Pattern matching engine, and Evaluator.
*(To be developed in future sessions).*

### STAGE 2 — Self-Host
The Stage 0 seed (Rust) will be used to run the Stage 1 source code (REFAL-5) to compile itself. If the output is consistent, the compiler is fully self-hosted.

## Progress
- [x] Stage 0 Rust Lexer
- [x] Stage 0 Rust Parser
- [x] Stage 0 Rust REMA Evaluator with Pattern Matching (including `s.`, `t.`, `e.` and infinite lookahead)
- [x] Stage 0 Rust Builtins (`<Prout>`, `<Card>`, `<Open>`, `<Get>`, `<Put>`, `<Close>`)
- [ ] Stage 0 CLI execution hook
- [ ] Stage 1 REFAL-5 implementation
