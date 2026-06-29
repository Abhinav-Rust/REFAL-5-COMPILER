//! Minimal interpreter layer over the runtime matcher.

use std::collections::HashMap;

use refal_ast::{Function, Item, Program, Symbol, Term, TermKind, Variable, Visibility};

use crate::Value;
use crate::matcher::{Bindings, MatchError, VariableKey, match_pattern};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    FunctionNotFound(String),
    NoMatchingSentence(String),
    ConditionsUnsupported,
    CallsUnsupported,
    UnboundVariable(String),
    Match(MatchError),
}

pub struct Evaluator<'a> {
    functions: HashMap<&'a str, &'a Function>,
}

impl<'a> Evaluator<'a> {
    pub fn new(program: &'a Program) -> Self {
        let functions = program
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Function(function) => Some((function.name.as_str(), function)),
                Item::Declaration(_) => None,
            })
            .collect();

        Self { functions }
    }

    pub fn evaluate_entry(&self, args: &[Value]) -> Result<Vec<Value>, EvalError> {
        let Some(entry) = self
            .functions
            .values()
            .find(|function| function.visibility == Visibility::Entry)
        else {
            return Err(EvalError::FunctionNotFound("$ENTRY".to_string()));
        };

        self.evaluate_function(&entry.name, args)
    }

    pub fn evaluate_function(&self, name: &str, args: &[Value]) -> Result<Vec<Value>, EvalError> {
        let Some(function) = self.functions.get(name) else {
            return Err(EvalError::FunctionNotFound(name.to_string()));
        };

        for sentence in &function.sentences {
            if !sentence.conditions.is_empty() {
                return Err(EvalError::ConditionsUnsupported);
            }

            match match_pattern(&sentence.pattern, args) {
                Ok(bindings) => return eval_terms(&sentence.result, &bindings),
                Err(MatchError::NoMatch) => continue,
                Err(error) => return Err(EvalError::Match(error)),
            }
        }

        Err(EvalError::NoMatchingSentence(name.to_string()))
    }
}

fn eval_terms(terms: &[Term], bindings: &Bindings) -> Result<Vec<Value>, EvalError> {
    let mut output = Vec::new();
    for term in terms {
        match &term.kind {
            TermKind::Symbol(symbol) => output.push(eval_symbol(symbol)),
            TermKind::Variable(variable) => output.extend(resolve_variable(variable, bindings)?),
            TermKind::Bracket(inner) => output.push(Value::Bracket(eval_terms(inner, bindings)?)),
            TermKind::Call { .. } => return Err(EvalError::CallsUnsupported),
        }
    }
    Ok(output)
}

fn eval_symbol(symbol: &Symbol) -> Value {
    match symbol {
        Symbol::Char(ch) => Value::Char(*ch),
        Symbol::Identifier(name) => Value::Identifier(name.clone()),
        Symbol::Number(number) => Value::Number(number.clone()),
    }
}

fn resolve_variable(variable: &Variable, bindings: &Bindings) -> Result<Vec<Value>, EvalError> {
    let key = VariableKey::from(variable);
    bindings.get(&key).cloned().ok_or_else(|| {
        EvalError::UnboundVariable(format!("{}.{}", variable_prefix(variable), variable.name))
    })
}

fn variable_prefix(variable: &Variable) -> char {
    match variable.kind {
        refal_ast::VariableKind::Symbol => 's',
        refal_ast::VariableKind::Term => 't',
        refal_ast::VariableKind::Expression => 'e',
    }
}

#[cfg(test)]
mod tests {
    use refal_ast::{Sentence, Span, Variable, VariableKind};

    use super::*;

    fn span() -> Span {
        Span { start: 0, end: 0 }
    }

    fn term(kind: TermKind) -> Term {
        Term { kind, span: span() }
    }

    fn var(kind: VariableKind, name: &str) -> Term {
        term(TermKind::Variable(Variable {
            kind,
            name: name.to_string(),
        }))
    }

    fn function(name: &str, visibility: Visibility, sentences: Vec<Sentence>) -> Function {
        Function {
            name: name.to_string(),
            visibility,
            sentences,
            span: span(),
        }
    }

    fn program(functions: Vec<Function>) -> Program {
        Program {
            items: functions.into_iter().map(Item::Function).collect(),
        }
    }

    #[test]
    fn evaluates_identity_entry() {
        let sentence = Sentence {
            pattern: vec![var(VariableKind::Expression, "X")],
            conditions: vec![],
            result: vec![var(VariableKind::Expression, "X")],
            span: span(),
        };
        let program = program(vec![function("Go", Visibility::Entry, vec![sentence])]);
        let evaluator = Evaluator::new(&program);

        let result = evaluator
            .evaluate_entry(&[Value::Char('A'), Value::Char('B')])
            .unwrap();

        assert_eq!(result, vec![Value::Char('A'), Value::Char('B')]);
    }

    #[test]
    fn evaluates_literal_result() {
        let sentence = Sentence {
            pattern: vec![],
            conditions: vec![],
            result: vec![term(TermKind::Symbol(Symbol::Char('O')))],
            span: span(),
        };
        let program = program(vec![function("Go", Visibility::Entry, vec![sentence])]);
        let evaluator = Evaluator::new(&program);

        assert_eq!(
            evaluator.evaluate_entry(&[]).unwrap(),
            vec![Value::Char('O')]
        );
    }

    #[test]
    fn tries_later_sentence_after_no_match() {
        let first = Sentence {
            pattern: vec![term(TermKind::Symbol(Symbol::Char('A')))],
            conditions: vec![],
            result: vec![term(TermKind::Symbol(Symbol::Char('X')))],
            span: span(),
        };
        let second = Sentence {
            pattern: vec![term(TermKind::Symbol(Symbol::Char('B')))],
            conditions: vec![],
            result: vec![term(TermKind::Symbol(Symbol::Char('Y')))],
            span: span(),
        };
        let program = program(vec![function("Go", Visibility::Entry, vec![first, second])]);
        let evaluator = Evaluator::new(&program);

        assert_eq!(
            evaluator.evaluate_entry(&[Value::Char('B')]).unwrap(),
            vec![Value::Char('Y')]
        );
    }
}
