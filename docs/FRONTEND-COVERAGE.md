# Classic Refal-5 Frontend Coverage

This matrix is the completion contract for Milestone 2. A row is complete only when
the lexer/parser behavior and positive and negative tests are present.

Primary clean-room references:

- Refal-5 syntax reference: https://www.refal.net/refer_r5.html
- Refal-5 Programming Guide and Reference Manual:
  https://www.refal.net/english/doc/turchin/ref5_eng/html/

Refal-5 lambda and Refal-05 implementations are not normative sources for this
matrix. They may be used later for explicitly documented compatibility research,
but their extensions must not silently enter the Classic Refal-5 frontend.

## Lexical Coverage

| Requirement | Status | Evidence or gap |
| --- | --- | --- |
| Structural `()` and call `<>` brackets | Complete | Lexer and parser tests |
| `{}`, `=`, `;`, `:`, `,` separators | Complete | Lexer and parser tests |
| `$ENTRY` | Complete | Lexer and parser tests |
| `$EXTERNAL`, `$EXTERN`, `$EXTRN` | Complete | Alias lexer test and declaration parser test |
| Single-quoted character strings | Complete | Multi-character lexer test |
| Double-quoted character strings | Complete | Lexer test covers multi-character text |
| Inter-token whitespace | Complete | Exercised throughout parser tests |
| `/* ... */` comments | Complete | Includes unterminated-comment diagnostic |
| Line comments beginning with `*` | Complete | Lexer test covers a comment before a definition |
| Identifier lexical rules | Complete | Uppercase-start diagnostics, 15-character limits, variable index limits, and Classic name equivalence tests |
| Non-negative integer macrodigits | Complete | Number token and AST symbol |
| Signed and unsigned real numbers | Complete | Lexer test covers decimal, exponent, and signed forms |
| Quoted keyboard-character symbols | Complete | Single and double quote forms, opposite-quote content, and empty literal diagnostics |
| `s.`, `t.`, `e.` variables | Complete | Lexer/parser/runtime tests |
| One-character variable shorthand | Complete | Lexer test covers letter and digit indices |
| Invalid-token diagnostics with spans | Complete | CLI golden tests cover identifier and malformed-number lex errors with line/column output |

## Grammar Coverage

| Requirement | Status | Evidence or gap |
| --- | --- | --- |
| Empty and non-empty expressions | Complete | Sentence and term parser tests |
| Symbols, variables, structural terms, calls | Complete | Parser tests |
| Function definitions | Complete | Parser tests |
| `$ENTRY` function definitions | Complete | Parser tests |
| External declarations | Complete | Parser test |
| Multiple names in external declarations | Complete | Parser test |
| Sentence alternatives | Complete | Runtime/parser examples |
| Empty patterns and results | Complete | Hello example |
| Condition chains | Complete | Parser and interpreter tests |
| Calls prohibited in patterns | Complete | Semantic checker and CLI golden tests reject calls in patterns |
| Optional semicolons between top-level definitions | Complete | Parser test covers separated definitions |
| Full malformed-program golden suite | Complete | Negative fixtures cover unresolved calls, unbound variables, lexical errors, malformed numbers, and pattern calls |

## Milestone 2 Exit Criteria

- Every row above is `Complete`.
- Positive and negative golden fixtures cover lexical and grammar categories in scope.
- `cargo fmt --check`, `cargo test`, and `git diff --check` must pass before each push.
- README and roadmap report Milestone 2 as complete.
