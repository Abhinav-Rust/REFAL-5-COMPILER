# Language Scope

The first target is Classic Refal-5 semantics.

The compiler should prefer correctness over breadth. A small feature with exact behavior is more valuable than a large feature that only works on demos.

## Initial Scope

- Functions and `$ENTRY`.
- Sentences with pattern, optional conditions, and result.
- Object expressions.
- Structural brackets.
- Function calls.
- Symbol, term, and expression variables: `s.`, `t.`, `e.`.
- Repeated variable matching.
- Character and number symbols.

## Planned Scope

- Full built-in function set.
- External declarations.
- Include/module workflow.
- Better string ergonomics while preserving Refal semantics.
- Optimization-oriented Core Refal.

## Out Of Scope For The First Compiler Pass

- Copying other compiler implementations.
- Undocumented extensions before Classic Refal behavior is stable.
- Overclaiming self-hosting before it is verified.
