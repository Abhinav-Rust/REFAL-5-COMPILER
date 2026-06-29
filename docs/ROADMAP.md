# Roadmap

The roadmap is intentionally practical. The compiler must become useful for real programs, not merely demonstrate a few parser tricks.

## Milestone 1: Public-Grade Foundation

- Clean repository structure.
- Professional README.
- Clean-room policy.
- Initial lexer/parser/AST.
- CLI commands for syntax checking and AST dumping.
- CI-ready test command.

## Milestone 2: Classic Refal-5 Front End

- Complete token coverage.
- Parser for functions, declarations, calls, brackets, variables, symbols, numbers, and literals.
- Source locations throughout AST.
- Human-readable diagnostics.
- Golden tests for valid and invalid programs.

## Milestone 3: Semantic Checker

- Entry point validation.
- Function declaration checks.
- Variable binding checks.
- Pattern/result variable legality.
- Condition checks.
- Clear diagnostics.

Status: initial implementation started.

## Milestone 4: Runtime And Interpreter

- Object-expression runtime.
- Correct `s.`, `t.`, `e.` pattern matching.
- Backtracking and rollback.
- Built-in functions.
- Executable interpreter mode.

Status: runtime value model, initial matcher, and first interpreter slice started.

## Milestone 5: Core Refal Lowering

- Normalize high-level Refal into explicit Core Refal.
- Preserve source maps for diagnostics.
- Emit stable formatted Refal/Core Refal output.

## Milestone 6: Production Backend

- Lower Core Refal to compiler IR.
- Generate practical executable code.
- Provide optimization passes.
- Add conformance and performance tests.

## Milestone 7: Self-Hosting

- Rebuild compiler components in Refal.
- Compile compiler sources through the toolchain.
- Maintain Rust bootstrap as a verification harness.

## Quality Bar

Each milestone should include:

- tests,
- examples,
- documentation updates,
- clear CLI behavior,
- and a changelog entry.
