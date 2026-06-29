# Refal Compiler

> A clean-room, production-oriented compiler project for Classic Refal-5 and a modern Refal toolchain.

Refal deserves a compiler project that feels usable in 2026: clear documentation, a predictable command-line interface, strong diagnostics, real tests, and a path to practical code generation. This repository is a ground-up rebuild toward that goal.

The project starts with a robust bootstrap front end and grows toward a self-hosting Refal compiler with production backends.

## Goals

- Implement Classic Refal-5 semantics with care.
- Provide a polished compiler CLI for real projects.
- Build excellent parser and semantic diagnostics.
- Support interpreter-driven development.
- Lower Refal into a documented Core Refal representation.
- Generate practical production code through a native backend.
- Keep the implementation clean-room and independently authored.
- Maintain public-repo quality from the first commit onward.

## Non-Goals

- This is not a copy of another Refal compiler.
- This is not a thin demo interpreter.
- This is not an academic-only artifact.
- This repository will not claim self-hosting or production readiness before tests prove it.

## Project Status

Early foundation phase.

| Component | Status |
| --- | --- |
| Clean repository reset | Complete |
| Public documentation baseline | Complete |
| Rust bootstrap workspace | Complete |
| AST model | Initial |
| Lexer | Initial |
| Parser | Initial |
| CLI syntax check | Initial |
| Semantic checker | Initial |
| Line/column diagnostics | Initial |
| Runtime object model | Initial |
| Pattern matcher | Initial |
| Interpreter | Initial |
| Core Refal lowering | Planned |
| Native code backend | Planned |
| Self-hosting compiler path | Planned |

## Architecture

```text
Refal source
  -> lexer
  -> parser
  -> AST
  -> semantic checker and diagnostics
  -> Core Refal / IR
  -> interpreter or native backend
```

Current workspace:

```text
crates/
  refal-ast      Shared compiler data structures
  refal-syntax   Lexer and parser
  refal-semantics Semantic validation
  refal-runtime  Runtime values, pattern matcher, and interpreter
  refal-cli      Command-line interface
docs/            Architecture, roadmap, clean-room policy
examples/        Small Refal programs
```

## Quick Start

```bash
cargo test
cargo run -p refal -- check examples/identity.ref
cargo run -p refal -- dump-ast examples/identity.ref
```

Example program:

```refal
$ENTRY Go {
  (e.1) = e.1;
}
```

## Compiler Direction

Rust is used for bootstrap infrastructure, tests, and early compiler construction. The compiler design is Refal-first: object expressions, structural brackets, `s.`, `t.`, and `e.` variables, sentence selection, conditions, and backtracking are the semantic center of the project.

The long-term plan includes:

1. A dependable interpreter for development.
2. A Core Refal lowering pipeline.
3. A production backend for practical generated code.
4. A self-hosting path where compiler components can be authored in Refal.

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Language Scope](docs/LANGUAGE-SCOPE.md)
- [Roadmap](docs/ROADMAP.md)
- [Clean-Room Policy](docs/CLEANROOM.md)
- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## Clean-Room Policy

This project is independently authored. Existing public Refal implementations may be studied as language references or behavioral comparison points, but their source code is not copied into this repository.

## License

MIT. See [LICENSE-MIT](LICENSE-MIT).
