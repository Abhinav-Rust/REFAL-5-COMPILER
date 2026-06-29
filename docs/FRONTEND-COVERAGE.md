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
| Identifier lexical rules | Partial | Current lexer is broader and does not enforce Classic limits/normalization |
| Non-negative integer macrodigits | Complete | Number token and AST symbol |
| Signed and unsigned real numbers | Missing | Only decimal digit strings are recognized |
| Quoted keyboard-character symbols | Partial | Escaping and quote behavior need explicit tests |
| `s.`, `t.`, `e.` variables | Complete | Lexer/parser/runtime tests |
| One-character variable shorthand | Missing | Forms such as `sX` and `e1` are not recognized |
| Invalid-token diagnostics with spans | Partial | Representative tests exist; golden coverage is incomplete |

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
| Calls prohibited in patterns | Partial | Runtime rejection exists; parser/semantic diagnostic is not yet explicit |
| Optional semicolons between top-level definitions | Complete | Parser test covers separated definitions |
| Full malformed-program golden suite | Missing | Current negative examples cover only semantic errors |

## Milestone 2 Exit Criteria

- Every row above is `Complete`, or explicitly documented as excluded from the
  Classic Refal-5 target with rationale.
- Positive and negative golden fixtures cover every lexical and grammar category.
- `cargo fmt --check`, `cargo test`, and `git diff --check` pass.
- README and roadmap report Milestone 2 as complete only after those checks.
