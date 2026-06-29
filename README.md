<div align="center">

# Refal-5 Compiler

**A clean-room, production-oriented compiler for Classic Refal-5 — built in Rust.**

[![CI](https://github.com/Abhinav-Rust/REFAL-5-COMPILER/actions/workflows/ci.yml/badge.svg)](https://github.com/Abhinav-Rust/REFAL-5-COMPILER/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-2024_edition-orange.svg)](https://www.rust-lang.org/)
[![Status: Active Development](https://img.shields.io/badge/status-active_development-brightgreen.svg)](#project-status)

</div>

---

## What Is Refal?

r<b>EFAL</b> (r<b>E</b>cursive <b>F</b>unctions <b>A</b>lgorithmic <b>L</b>anguage) is one of the oldest high-level programming languages still in active scholarly use. It was created in the Soviet Union in the 1960s by Valentin Turchin as a language for symbolic computation and meta-programming — tasks like writing compilers, transforming programs, and working with structured symbolic data. Unlike most languages of its era, r<b>EFAL</b> was not built around numbers or sequential instructions. It was built around *rewriting*: you describe patterns that match expressions, and the language transforms them according to your rules.

r<b>EFAL</b>’s computational model is deceptively powerful. A r<b>EFAL</b> program is a set of *functions*, each consisting of ordered *sentences*. A sentence is a pattern on the left and a result expression on the right. The runtime scans an *active expression* — a chain of symbols and structured brackets — matches it against available patterns, and replaces the matched portion with the result. This process repeats until no active calls remain. The result is a language that feels somewhere between Prolog (pattern matching), Haskell (functional composition), and Lisp (symbolic data), but with its own distinct flavour rooted in pure term rewriting.

**Classic r<b>EFAL</b>-5** is the most widely documented dialect, defined by Sergei Romanenko’s specification. It is the dialect this compiler targets. The ideas behind r<b>EFAL</b> also gave Turchin the foundation for **sup<b>ER</b>compilation** — a powerful program transformation technique in which an interpreter symbolically drives its own execution, folding repeated configurations into loops and eliminating entire layers of abstraction at compile time. sup<b>ER</b>compilation remains an active area of research in program optimisation and partial evaluation. Despite r<b>EFAL</b>’s age, its core ideas are as relevant as ever: symbolic pattern matching is the right tool for a broad class of problems in language processing, AI, theorem proving, and formal verification. This project exists to make r<b>EFAL</b>-5 accessible to a modern developer with a modern toolchain — not as a museum piece, but as a practical programming tool.

---

## A Taste of Refal

Below is a valid r<b>EFAL</b>-5 program. The `$ENTRY Go` function is the program’s entry point. `Prout` is a built-in that prints a character string.

```
$EXTERN Prout;

$ENTRY Go {
  = <Prout 'Hello, Refal'>;
}
```

Pattern matching on a recursive function looks like this — here, reversing a sequence of symbols:

```
Reverse {
  /* base case: empty expression */
  =  ;

  /* recursive case: peel the head, reverse the tail, append head at the end */
  s.Head e.Rest = <Reverse e.Rest> s.Head;
}
```

Variables in r<b>EFAL</b> carry their type in their prefix: `s.` matches a single symbol, `e.` matches any expression (zero or more terms), and `t.` matches a single term (which may itself be a bracketed structure). This typed variable system is what makes r<b>EFAL</b>’s pattern matching both precise and expressive.

---

## This Project

r<b>EFAL</b> deserves a compiler that feels usable in 2026: clear documentation, a predictable command-line interface, strong diagnostics, real tests, and a path to practical code generation. This repository is a ground-up, clean-room rebuild toward that goal — independently authored, with production standards from the first commit.

The project starts with a robust bootstrap front end written in Rust and grows toward a self-hosting r<b>EFAL</b> compiler with production backends.

### Goals

- Implement Classic r<b>EFAL</b>-5 semantics with care and completeness.
- Provide a polished compiler CLI for real projects.
- Build excellent parser and semantic diagnostics.
- Support interpreter-driven development.
- Lower r<b>EFAL</b> into a documented Core r<b>EFAL</b> representation.
- Generate practical production code through a native backend.
- Keep the implementation clean-room and independently authored.
- Maintain public-repo quality from the first commit onward.

### Non-Goals

- This is not a copy of another r<b>EFAL</b> compiler.
- This is not a thin demo interpreter.
- This is not an academic-only artifact.
- This repository will not claim self-hosting or production readiness before tests prove it.

---

## Project Status

**Milestone 1 is complete.** Work is currently focused on closing Milestone 2 against the [Classic r<b>EFAL</b>-5 frontend coverage matrix](docs/FRONTEND-COVERAGE.md). See the full [Roadmap](docs/ROADMAP.md) for planned milestones.

| Component | Status |
|---|---|
| Rust bootstrap workspace | ✅ Complete |
| Clean repository structure | ✅ Complete |
| Public documentation baseline | ✅ Complete |
| CI pipeline | ✅ Complete |
| MIT license | ✅ Complete |
| AST model (`refal-ast`) | 🔶 Initial |
| Lexer (`refal-syntax`) | 🔷 Milestone 2 — classic quotes, comments, real numbers, variables |
| Parser (`refal-syntax`) | 🔷 Milestone 2 — core grammar and top-level separators |
| CLI (`check`, `dump-ast`, `run`) | 🔶 Initial |
| Semantic checker (`refal-semantics`) | 🔶 Initial — entry points, declarations, unbound variables |
| Diagnostics with source positions | 🔶 Initial |
| Golden test suite | 🔷 Planned for Milestone 2 |
| Core r<b>EFAL</b> lowering | 🔷 Planned for Milestone 3 |
| Native backend / code generation | 🔷 Planned for later milestones |

---

## Repository Layout

```
REFAL-5-COMPILER/
├── crates/
│   ├── refal-ast/        # AST node types and data model
│   ├── refal-syntax/     # Lexer and parser
│   ├── refal-semantics/  # Semantic checker
│   ├── refal-runtime/    # Runtime support layer
│   └── refal-cli/        # Command-line interface (check, dump-ast, run)
├── examples/             # Sample .ref programs
├── docs/
│   ├── ARCHITECTURE.md   # Crate structure and design decisions
│   ├── ROADMAP.md        # Milestone plan
│   ├── FRONTEND-COVERAGE.md  # Lexer/parser coverage matrix
│   ├── LANGUAGE-SCOPE.md # What dialect features are in scope
│   └── CLEANROOM.md      # Clean-room authorship policy
├── .github/workflows/    # CI (cargo test on every push)
├── CONTRIBUTING.md
├── CHANGELOG.md
└── LICENSE-MIT
```

---

## Building

**Prerequisites:** A stable Rust toolchain. Install via [rustup](https://rustup.rs/) if you do not have one.

```sh
git clone https://github.com/Abhinav-Rust/REFAL-5-COMPILER.git
cd REFAL-5-COMPILER
cargo build
```

Run the test suite:

```sh
cargo test
```

---

## Using the CLI

The CLI is in early development. The following commands are available:

```sh
# Check a .ref file for syntax and semantic errors
cargo run -p refal-cli -- check examples/hello.ref

# Dump the parsed AST in a human-readable format
cargo run -p refal-cli -- dump-ast examples/hello.ref

# Run a .ref program (interpreter — initial stage)
cargo run -p refal-cli -- run examples/hello.ref
```

The CLI and interpreter are at an initial stage. Not all r<b>EFAL</b>-5 programs will execute correctly yet. See the [frontend coverage matrix](docs/FRONTEND-COVERAGE.md) for what is currently supported.

---

## Documentation

| Document | Description |
|---|---|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Crate design and compiler pipeline |
| [ROADMAP.md](docs/ROADMAP.md) | Milestone plan and completion criteria |
| [FRONTEND-COVERAGE.md](docs/FRONTEND-COVERAGE.md) | Lexer/parser coverage tracking |
| [LANGUAGE-SCOPE.md](docs/LANGUAGE-SCOPE.md) | Dialect features in and out of scope |
| [CLEANROOM.md](docs/CLEANROOM.md) | Clean-room authorship policy |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |
| [CHANGELOG.md](CHANGELOG.md) | What has changed |

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. The clean-room policy in [docs/CLEANROOM.md](docs/CLEANROOM.md) applies to all contributions.

---

## License

This project is licensed under the [MIT License](LICENSE-MIT).
