#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Entry,

    // Variables
    SVar(String),
    TVar(String),
    EVar(String),

    // Atoms
    Ident(String),
    Number(i64),
    StringLit(String),

    // Brackets
    LParen, // (
    RParen, // )
    LAngle, // <
    RAngle, // >
    LBrace, // {
    RBrace, // }

    // Punctuation
    Equal,    // =
    Semi,     // ;
    Comma,    // ,
    Colon,    // :

    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.input[self.pos..].starts_with("/*") {
                // Block comment
                self.advance();
                self.advance();
                while let Some(c) = self.peek() {
                    if c == '*' && self.input[self.pos..].starts_with("*/") {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
            } else if c == '*' && self.input[self.pos..].starts_with("***") {
                 // Line comment
                 while let Some(c) = self.peek() {
                     if c == '\n' {
                         break;
                     }
                     self.advance();
                 }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let c = match self.advance() {
            Some(c) => c,
            None => return Token::EOF,
        };

        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '<' => Token::LAngle,
            '>' => Token::RAngle,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '=' => Token::Equal,
            ';' => Token::Semi,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '\'' | '"' => {
                let quote = c;
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if ch == quote {
                        self.advance();
                        break;
                    }
                    s.push(ch);
                    self.advance();
                }
                Token::StringLit(s)
            }
            '$' => {
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        s.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                if s.to_uppercase() == "ENTRY" {
                    Token::Entry
                } else {
                    // Fallback to ident if it's not ENTRY (might be EXTERN, but we only strictly need ENTRY right now)
                    Token::Ident(format!("${}", s))
                }
            }
            _ if c.is_digit(10) || (c == '-' && self.peek().map_or(false, |p| p.is_digit(10))) => {
                let mut s = String::new();
                s.push(c);
                while let Some(ch) = self.peek() {
                    if ch.is_digit(10) {
                        s.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                Token::Number(s.parse().unwrap_or(0))
            }
            _ if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);
                while let Some(ch) = self.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        s.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                // Check for variables: s.NAME, t.NAME, e.NAME
                if (s == "s" || s == "t" || s == "e") && self.peek() == Some('.') {
                    self.advance(); // consume '.'
                    let mut var_name = String::new();
                    while let Some(ch) = self.peek() {
                        if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                            var_name.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return match s.as_str() {
                        "s" => Token::SVar(var_name),
                        "t" => Token::TVar(var_name),
                        "e" => Token::EVar(var_name),
                        _ => unreachable!(),
                    };
                }

                Token::Ident(s)
            }
            _ => Token::Ident(c.to_string()), // single char fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("$ENTRY Go { (e.1) = <Prout 'Hello'>; }");
        assert_eq!(lexer.next_token(), Token::Entry);
        assert_eq!(lexer.next_token(), Token::Ident("Go".into()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::EVar("1".into()));
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::LAngle);
        assert_eq!(lexer.next_token(), Token::Ident("Prout".into()));
        assert_eq!(lexer.next_token(), Token::StringLit("Hello".into()));
        assert_eq!(lexer.next_token(), Token::RAngle);
        assert_eq!(lexer.next_token(), Token::Semi);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
