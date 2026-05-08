# TASK: Build a Sovereign Refal-5 Compiler/Interpreter

## Context
You are building the "Buddhi" (Intellect) layer for the AA2 Vedic Astrology System. This layer must provide deterministic validation of natural language readings against symbolic chart data (CHR).

## Technical Requirements
1. **Language**: Refal-5 (Recursive Functions Algorithmic Language).
2. **Engine Core**: Implement the **REMA (Refal Abstract Machine)** model.
3. **Symbolic Representation**: 
    - The engine must handle **S-expressions** (atoms and brackets).
    - It must support the three primary variable types: `e.` (Expression), `s.` (Symbol), `t.` (Term).
4. **Pattern Matching Algorithm**: 
    - Implement Turchin's recursive matching with **infinite lookahead**.
    - Specifically, ensure that patterns like `<Verify e.CHR e.Reading>` can backtrack through the `e.Reading` string to find semantic anchors (e.g., "combust", "retrograde") regardless of position.
5. **Output**: The compiler should target a static library (for Rust FFI) or provide a CLI that outputs JSON-formatted validation results.

## Deliverables
- A working Refal-5 interpreter/compiler.
- A test suite validating the "Mercury Combustion" case:
    - Input Fact: `(Mercury (Status Combust Retro))`
    - Input Text: "Your Mercury is burnt and backward."
    - Output: `(Match Success)`
