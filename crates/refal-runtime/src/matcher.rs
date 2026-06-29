//! Pattern matching for Refal object expressions.

use std::collections::HashMap;

use refal_ast::{Symbol, Term, TermKind, Variable, VariableKind};

use crate::Value;

pub type Bindings = HashMap<VariableKey, Vec<Value>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableKey {
    pub kind: VariableKind,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchError {
    NoMatch,
    CallsAreNotPatterns,
}

pub fn match_pattern(pattern: &[Term], input: &[Value]) -> Result<Bindings, MatchError> {
    match_from(pattern, input, Bindings::new())
}

fn match_from(
    pattern: &[Term],
    input: &[Value],
    bindings: Bindings,
) -> Result<Bindings, MatchError> {
    let Some((first, rest_pattern)) = pattern.split_first() else {
        return if input.is_empty() {
            Ok(bindings)
        } else {
            Err(MatchError::NoMatch)
        };
    };

    match &first.kind {
        TermKind::Symbol(symbol) => {
            let Some((first_input, rest_input)) = input.split_first() else {
                return Err(MatchError::NoMatch);
            };
            if symbol_matches(symbol, first_input) {
                match_from(rest_pattern, rest_input, bindings)
            } else {
                Err(MatchError::NoMatch)
            }
        }
        TermKind::Bracket(inner_pattern) => {
            let Some((Value::Bracket(inner_input), rest_input)) = input.split_first() else {
                return Err(MatchError::NoMatch);
            };
            let bindings = match_from(inner_pattern, inner_input, bindings)?;
            match_from(rest_pattern, rest_input, bindings)
        }
        TermKind::Variable(variable) => match variable.kind {
            VariableKind::Symbol => {
                match_single(variable, input, rest_pattern, bindings, |value| {
                    !matches!(value, Value::Bracket(_))
                })
            }
            VariableKind::Term => match_single(variable, input, rest_pattern, bindings, |_| true),
            VariableKind::Expression => match_expression(variable, input, rest_pattern, bindings),
        },
        TermKind::Call { .. } => Err(MatchError::CallsAreNotPatterns),
    }
}

fn symbol_matches(symbol: &Symbol, value: &Value) -> bool {
    match (symbol, value) {
        (Symbol::Char(left), Value::Char(right)) => left == right,
        (Symbol::Identifier(left), Value::Identifier(right)) => left == right,
        (Symbol::Number(left), Value::Number(right)) => left == right,
        _ => false,
    }
}

fn match_single(
    variable: &Variable,
    input: &[Value],
    rest_pattern: &[Term],
    bindings: Bindings,
    accepts: impl Fn(&Value) -> bool,
) -> Result<Bindings, MatchError> {
    let Some((first_input, rest_input)) = input.split_first() else {
        return Err(MatchError::NoMatch);
    };
    if !accepts(first_input) {
        return Err(MatchError::NoMatch);
    }

    let key = VariableKey::from(variable);
    let value = vec![first_input.clone()];
    let bindings = bind_or_check(bindings, key, value)?;
    match_from(rest_pattern, rest_input, bindings)
}

fn match_expression(
    variable: &Variable,
    input: &[Value],
    rest_pattern: &[Term],
    bindings: Bindings,
) -> Result<Bindings, MatchError> {
    let key = VariableKey::from(variable);
    if let Some(bound) = bindings.get(&key) {
        if input.starts_with(bound) {
            return match_from(rest_pattern, &input[bound.len()..], bindings);
        }
        return Err(MatchError::NoMatch);
    }

    for split in 0..=input.len() {
        let value = input[..split].to_vec();
        let attempt = bind_or_check(bindings.clone(), key.clone(), value)?;
        if let Ok(final_bindings) = match_from(rest_pattern, &input[split..], attempt) {
            return Ok(final_bindings);
        }
    }

    Err(MatchError::NoMatch)
}

fn bind_or_check(
    mut bindings: Bindings,
    key: VariableKey,
    value: Vec<Value>,
) -> Result<Bindings, MatchError> {
    if let Some(existing) = bindings.get(&key) {
        if existing == &value {
            Ok(bindings)
        } else {
            Err(MatchError::NoMatch)
        }
    } else {
        bindings.insert(key, value);
        Ok(bindings)
    }
}

impl From<&Variable> for VariableKey {
    fn from(variable: &Variable) -> Self {
        Self {
            kind: variable.kind,
            name: variable.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use refal_ast::{Span, Variable};

    use super::*;

    fn span() -> Span {
        Span { start: 0, end: 0 }
    }

    fn char_term(ch: char) -> Term {
        Term {
            kind: TermKind::Symbol(Symbol::Char(ch)),
            span: span(),
        }
    }

    fn var(kind: VariableKind, name: &str) -> Term {
        Term {
            kind: TermKind::Variable(Variable {
                kind,
                name: name.to_string(),
            }),
            span: span(),
        }
    }

    #[test]
    fn matches_literal_symbols() {
        let bindings = match_pattern(&[char_term('A')], &[Value::Char('A')]).unwrap();
        assert!(bindings.is_empty());
    }

    #[test]
    fn rejects_literal_mismatch() {
        assert_eq!(
            match_pattern(&[char_term('A')], &[Value::Char('B')]),
            Err(MatchError::NoMatch)
        );
    }

    #[test]
    fn s_variable_matches_non_bracket_symbol() {
        let bindings =
            match_pattern(&[var(VariableKind::Symbol, "X")], &[Value::Char('A')]).unwrap();
        assert_eq!(
            bindings[&VariableKey {
                kind: VariableKind::Symbol,
                name: "X".to_string()
            }],
            vec![Value::Char('A')]
        );
    }

    #[test]
    fn s_variable_rejects_bracket() {
        assert_eq!(
            match_pattern(
                &[var(VariableKind::Symbol, "X")],
                &[Value::Bracket(vec![Value::Char('A')])]
            ),
            Err(MatchError::NoMatch)
        );
    }

    #[test]
    fn t_variable_matches_single_bracket_term() {
        let input = Value::Bracket(vec![Value::Char('A')]);
        let bindings = match_pattern(
            &[var(VariableKind::Term, "X")],
            std::slice::from_ref(&input),
        )
        .unwrap();
        assert_eq!(
            bindings[&VariableKey {
                kind: VariableKind::Term,
                name: "X".to_string()
            }],
            vec![input]
        );
    }

    #[test]
    fn e_variable_backtracks_until_rest_matches() {
        let pattern = vec![
            var(VariableKind::Expression, "Left"),
            char_term('x'),
            var(VariableKind::Expression, "Right"),
        ];
        let input = vec![
            Value::Char('a'),
            Value::Char('b'),
            Value::Char('x'),
            Value::Char('c'),
        ];
        let bindings = match_pattern(&pattern, &input).unwrap();

        assert_eq!(
            bindings[&VariableKey {
                kind: VariableKind::Expression,
                name: "Left".to_string()
            }],
            vec![Value::Char('a'), Value::Char('b')]
        );
        assert_eq!(
            bindings[&VariableKey {
                kind: VariableKind::Expression,
                name: "Right".to_string()
            }],
            vec![Value::Char('c')]
        );
    }

    #[test]
    fn repeated_variable_must_match_same_value() {
        assert!(
            match_pattern(
                &[
                    var(VariableKind::Symbol, "X"),
                    var(VariableKind::Symbol, "X")
                ],
                &[Value::Char('A'), Value::Char('A')]
            )
            .is_ok()
        );

        assert_eq!(
            match_pattern(
                &[
                    var(VariableKind::Symbol, "X"),
                    var(VariableKind::Symbol, "X")
                ],
                &[Value::Char('A'), Value::Char('B')]
            ),
            Err(MatchError::NoMatch)
        );
    }

    #[test]
    fn matches_nested_brackets() {
        let pattern = vec![Term {
            kind: TermKind::Bracket(vec![char_term('A')]),
            span: span(),
        }];
        let input = vec![Value::Bracket(vec![Value::Char('A')])];

        assert!(match_pattern(&pattern, &input).is_ok());
    }
}
