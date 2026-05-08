use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.current_token == token {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", token, self.current_token))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();
        while self.current_token != Token::EOF {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let mut is_entry = false;
        if self.current_token == Token::Entry {
            is_entry = true;
            self.advance();
        }

        let name = match &self.current_token {
            Token::Ident(n) => n.clone(),
            _ => return Err(format!("Expected function name, got {:?}", self.current_token)),
        };
        self.advance();

        self.expect(Token::LBrace)?;

        let mut rules = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            rules.push(self.parse_rule()?);
        }

        self.expect(Token::RBrace)?;

        Ok(Function {
            name,
            is_entry,
            rules,
        })
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let pattern = self.parse_expressions(vec![Token::Equal, Token::Comma])?;

        let mut conditions = Vec::new();
        while self.current_token == Token::Comma {
            self.advance();
            let cond_result = self.parse_expressions(vec![Token::Colon])?;
            self.expect(Token::Colon)?;
            let cond_pattern = self.parse_expressions(vec![Token::Equal, Token::Comma])?;
            conditions.push(Condition {
                result: cond_result,
                pattern: cond_pattern,
            });
        }

        self.expect(Token::Equal)?;
        let result = self.parse_expressions(vec![Token::Semi])?;
        self.expect(Token::Semi)?;

        Ok(Rule {
            pattern,
            conditions,
            result,
        })
    }

    fn parse_expressions(&mut self, stop_tokens: Vec<Token>) -> Result<Vec<Expr>, String> {
        let mut exprs = Vec::new();
        while !stop_tokens.contains(&self.current_token) && self.current_token != Token::EOF {
            exprs.push(self.parse_expr()?);
        }
        Ok(exprs)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.current_token.clone() {
            Token::SVar(n) => {
                self.advance();
                Ok(Expr::SVar(n))
            }
            Token::TVar(n) => {
                self.advance();
                Ok(Expr::TVar(n))
            }
            Token::EVar(n) => {
                self.advance();
                Ok(Expr::EVar(n))
            }
            Token::Ident(n) => {
                self.advance();
                Ok(Expr::Ident(n))
            }
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::LParen => {
                self.advance();
                let inner = self.parse_expressions(vec![Token::RParen])?;
                self.expect(Token::RParen)?;
                Ok(Expr::Bracket(inner))
            }
            Token::LAngle => {
                self.advance();
                let name = match &self.current_token {
                    Token::Ident(n) => n.clone(),
                    _ => return Err(format!("Expected function name in call, got {:?}", self.current_token)),
                };
                self.advance();
                let args = self.parse_expressions(vec![Token::RAngle])?;
                self.expect(Token::RAngle)?;
                Ok(Expr::Call(name, args))
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current_token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parser() {
        let code = "$ENTRY Go { (e.1) = <Prout 'Hello'>; }";
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        let prog = parser.parse_program().unwrap();

        assert_eq!(prog.functions.len(), 1);
        let func = &prog.functions[0];
        assert_eq!(func.name, "Go");
        assert!(func.is_entry);
        assert_eq!(func.rules.len(), 1);
        let rule = &func.rules[0];
        assert_eq!(rule.pattern, vec![Expr::Bracket(vec![Expr::EVar("1".into())])]);
        assert_eq!(rule.result, vec![Expr::Call("Prout".into(), vec![Expr::StringLit("Hello".into())])]);
    }

    #[test]
    fn test_parser_condition() {
        let code = "Func { e.1 , e.1 : e.2 = e.2; }";
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        let prog = parser.parse_program().unwrap();

        assert_eq!(prog.functions.len(), 1);
        let func = &prog.functions[0];
        let rule = &func.rules[0];
        assert_eq!(rule.pattern, vec![Expr::EVar("1".into())]);
        assert_eq!(rule.conditions.len(), 1);
        assert_eq!(rule.conditions[0].result, vec![Expr::EVar("1".into())]);
        assert_eq!(rule.conditions[0].pattern, vec![Expr::EVar("2".into())]);
        assert_eq!(rule.result, vec![Expr::EVar("2".into())]);
    }
}
