# Refal-5 Compiler Specifications (for Jules)

## 1. Core Requirements
- **Abstract Machine**: Full implementation of the **REMA (Refal Abstract Machine)**.
- **Pattern Matching**: Native support for:
    - `e.x` (Expression: any number of terms)
    - `s.x` (Symbol: a single atom)
    - `t.x` (Term: a symbol or a bracketed expression)
- **Infinite Lookahead**: The engine must support the recursive pattern matching algorithm with backtracking as defined by Turchin.

## 2. Standard Library
- Minimal support for `Prout` (Standard Output) and `Card` (Standard Input).
- Support for basic arithmetic operations on symbols.

## 3. Integration & Deployment
- **Target**: Must be usable from a Rust harness (via FFI or as a standalone CLI tool).
- **Source Format**: Standard `.ref` files.
- **Error Handling**: Deterministic reporting of pattern match failures.

## 4. Performance
- Must handle text scanning for readings (~1000 words) in < 100ms.
