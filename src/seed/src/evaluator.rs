use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Symbol(String),
    Number(i64),
    StringLit(String),
    Bracket(Vec<Value>),
}

pub type Environment = HashMap<String, Vec<Value>>;

pub struct Evaluator {
    program: Program,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn evaluate(&self, entry_func: &str, args: Vec<Value>) -> Result<Vec<Value>, String> {
        self.call_function(entry_func, args)
    }

    fn call_function(&self, name: &str, args: Vec<Value>) -> Result<Vec<Value>, String> {
        let func = self.program.functions.iter().find(|f| f.name == name);
        let func = match func {
            Some(f) => f,
            None => {
                // Check built-ins
                return crate::builtins::call_builtin(self, name, args);
            }
        };

        for rule in &func.rules {
            let mut env = HashMap::new();
            if let Some(matched_env) = self.match_pattern(&rule.pattern, &args, env.clone()) {
                env = matched_env;

                // Check conditions
                let mut conditions_passed = true;
                for cond in &rule.conditions {
                    let eval_res = self.evaluate_exprs(&cond.result, &env)?;
                    if let Some(new_env) = self.match_pattern(&cond.pattern, &eval_res, env.clone()) {
                        env = new_env;
                    } else {
                        conditions_passed = false;
                        break;
                    }
                }

                if conditions_passed {
                    return self.evaluate_exprs(&rule.result, &env);
                }
            }
        }

        Err(format!("No matching rule in function {} for args {:?}", name, args))
    }

    fn evaluate_exprs(&self, exprs: &[Expr], env: &Environment) -> Result<Vec<Value>, String> {
        let mut result = Vec::new();
        for expr in exprs {
            match expr {
                Expr::Ident(n) => result.push(Value::Symbol(n.clone())),
                Expr::Number(n) => result.push(Value::Number(*n)),
                Expr::StringLit(s) => result.push(Value::StringLit(s.clone())),
                Expr::SVar(n) => {
                    if let Some(v) = env.get(n) {
                        if v.len() == 1 {
                            result.push(v[0].clone());
                        } else {
                            return Err(format!("s.{} must be a single symbol, found {:?}", n, v));
                        }
                    } else {
                        return Err(format!("Unbound variable s.{}", n));
                    }
                }
                Expr::TVar(n) => {
                    if let Some(v) = env.get(n) {
                        if v.len() == 1 {
                            result.push(v[0].clone());
                        } else {
                            return Err(format!("t.{} must be a single term, found {:?}", n, v));
                        }
                    } else {
                        return Err(format!("Unbound variable t.{}", n));
                    }
                }
                Expr::EVar(n) => {
                    if let Some(v) = env.get(n) {
                        result.extend(v.clone());
                    } else {
                        return Err(format!("Unbound variable e.{}", n));
                    }
                }
                Expr::Bracket(inner) => {
                    let eval_inner = self.evaluate_exprs(inner, env)?;
                    result.push(Value::Bracket(eval_inner));
                }
                Expr::Call(name, args) => {
                    let eval_args = self.evaluate_exprs(args, env)?;
                    let call_res = self.call_function(name, eval_args)?;
                    result.extend(call_res);
                }
            }
        }
        Ok(result)
    }

    fn match_pattern(&self, pattern: &[Expr], input: &[Value], env: Environment) -> Option<Environment> {
        self.match_pattern_recursive(pattern, input, env)
    }

    fn match_pattern_recursive(&self, pattern: &[Expr], input: &[Value], mut env: Environment) -> Option<Environment> {
        if pattern.is_empty() {
            if input.is_empty() {
                return Some(env);
            } else {
                return None;
            }
        }

        let first_pat = &pattern[0];
        let rest_pat = &pattern[1..];

        match first_pat {
            Expr::Ident(n) => {
                if !input.is_empty() && matches!(&input[0], Value::Symbol(s) if s == n) {
                    return self.match_pattern_recursive(rest_pat, &input[1..], env);
                }
            }
            Expr::Number(n) => {
                if !input.is_empty() && matches!(&input[0], Value::Number(m) if m == n) {
                    return self.match_pattern_recursive(rest_pat, &input[1..], env);
                }
            }
            Expr::StringLit(s) => {
                if !input.is_empty() && matches!(&input[0], Value::StringLit(st) if st == s) {
                    return self.match_pattern_recursive(rest_pat, &input[1..], env);
                }
            }
            Expr::Bracket(inner_pat) => {
                if !input.is_empty() {
                    if let Value::Bracket(inner_input) = &input[0] {
                        if let Some(new_env) = self.match_pattern_recursive(inner_pat, inner_input, env.clone()) {
                            return self.match_pattern_recursive(rest_pat, &input[1..], new_env);
                        }
                    }
                }
            }
            Expr::SVar(n) => {
                if !input.is_empty() {
                    let val = &input[0];
                    if let Value::Bracket(_) = val {
                        return None; // s-vars match symbols (including numbers/strings), not brackets
                    }
                    if let Some(bound) = env.get(n) {
                        if bound.len() == 1 && &bound[0] == val {
                            return self.match_pattern_recursive(rest_pat, &input[1..], env);
                        }
                    } else {
                        env.insert(n.clone(), vec![val.clone()]);
                        return self.match_pattern_recursive(rest_pat, &input[1..], env);
                    }
                }
            }
            Expr::TVar(n) => {
                if !input.is_empty() {
                    let val = &input[0];
                    // t-vars match any single term (symbol or bracket)
                    if let Some(bound) = env.get(n) {
                        if bound.len() == 1 && &bound[0] == val {
                            return self.match_pattern_recursive(rest_pat, &input[1..], env);
                        }
                    } else {
                        env.insert(n.clone(), vec![val.clone()]);
                        return self.match_pattern_recursive(rest_pat, &input[1..], env);
                    }
                }
            }
            Expr::EVar(n) => {
                if let Some(bound) = env.get(n) {
                    let bound_len = bound.len();
                    if input.len() >= bound_len && &input[..bound_len] == bound.as_slice() {
                        return self.match_pattern_recursive(rest_pat, &input[bound_len..], env);
                    }
                } else {
                    // Backtracking for e-var
                    for i in 0..=input.len() {
                        let mut new_env = env.clone();
                        new_env.insert(n.clone(), input[..i].to_vec());
                        if let Some(final_env) = self.match_pattern_recursive(rest_pat, &input[i..], new_env) {
                            return Some(final_env);
                        }
                    }
                }
            }
            _ => return None, // Calls inside patterns are not standard refal without conditions
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse_and_eval(code: &str, input: Vec<Value>) -> Result<Vec<Value>, String> {
        let mut parser = Parser::new(Lexer::new(code));
        let program = parser.parse_program().unwrap();
        let eval = Evaluator::new(program);
        eval.evaluate("Go", input)
    }

    #[test]
    fn test_eval_simple() {
        let code = "$ENTRY Go { s.1 = s.1; }";
        let res = parse_and_eval(code, vec![Value::Symbol("Hello".into())]).unwrap();
        assert_eq!(res, vec![Value::Symbol("Hello".into())]);
    }

    #[test]
    fn test_eval_evar_backtracking() {
        let code = "$ENTRY Go { e.1 'combust' e.2 = e.1 'SUCCESS' e.2; }";
        let input = vec![
            Value::StringLit("Mercury".into()),
            Value::StringLit("is".into()),
            Value::StringLit("combust".into()),
            Value::StringLit("today".into()),
        ];
        let res = parse_and_eval(code, input).unwrap();
        assert_eq!(res, vec![
            Value::StringLit("Mercury".into()),
            Value::StringLit("is".into()),
            Value::StringLit("SUCCESS".into()),
            Value::StringLit("today".into()),
        ]);
    }

    #[test]
    fn test_eval_conditions() {
        let code = "$ENTRY Go { e.1 , e.1 : e.A 'test' e.B = e.A; }";
        let input = vec![
            Value::StringLit("hello".into()),
            Value::StringLit("test".into()),
            Value::StringLit("world".into()),
        ];
        let res = parse_and_eval(code, input).unwrap();
        assert_eq!(res, vec![Value::StringLit("hello".into())]);
    }
}

#[cfg(test)]
mod mercury_tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_mercury_validation() {
        let code = "
        ValidateMercury {
            (e.1 ('Planet' 'Mercury' ('Status' e.Status)) e.2) e.Text
            , e.Text : e.A 'combust' e.B
            , e.Text : e.C 'retrograde' e.D = 'SUCCESS';

            (e.Facts) e.Text = 'FAILURE';
        }
        ";
        let mut parser = Parser::new(Lexer::new(code));
        let program = parser.parse_program().unwrap();
        let _eval = Evaluator::new(program);

        // We simulate the AST of the argument
        let _input = vec![
            Value::Bracket(vec![
                Value::Bracket(vec![
                    Value::StringLit("Planet".into()),
                    Value::StringLit("Mercury".into()),
                    Value::Bracket(vec![
                        Value::StringLit("Status".into()),
                        Value::StringLit("Combust".into()),
                        Value::StringLit("Retrograde".into()),
                    ])
                ])
            ]),
            // Text argument: sequence of string literal word atoms? No, in standard refal strings
            // are sequences of characters. But the user wrote 'combust' as a string literal.
            // Wait, let's treat it as string literals matching exact tokens for this test as that's how we parsed it.
            // In our lexer 'Your career ...' becomes a single string literal token!
            // Wait! The user provided 'Your career path ... combust ...' as a SINGLE string.
            // A pattern like `e.A 'combust' e.B` expects 'combust' to be a string atom.
            // But if the text is one big string `'Your... combust... '`, it won't match a sequence of `e.A StringLit('combust') e.B`.
            // Let's print out what `Token::StringLit` actually outputs.
        ];
        // We'll inspect what's happening.
    }
}
