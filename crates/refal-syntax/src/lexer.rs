use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Entry,
    Extern,
    Identifier(String),
    Variable { kind: char, name: String },
    Number(String),
    Char(char),
    LBrace,
    RBrace,
    LParen,
    RParen,
    LAngle,
    RAngle,
    Comma,
    Colon,
    Equals,
    Semicolon,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    pending: VecDeque<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
            pending: VecDeque::new(),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let done = token.kind == TokenKind::Eof;
            tokens.push(token);
            if done {
                return Ok(tokens);
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        if let Some(token) = self.pending.pop_front() {
            return Ok(token);
        }

        self.skip_ignored()?;
        let start = self.cursor;
        let Some(ch) = self.bump() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span { start, end: start },
            });
        };

        let kind = match ch {
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '<' => TokenKind::LAngle,
            '>' => TokenKind::RAngle,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '=' => TokenKind::Equals,
            ';' => TokenKind::Semicolon,
            '\'' | '"' => return self.lex_quoted_chars(start, ch),
            '$' => self.lex_directive(start)?,
            c if is_ident_start(c) => self.lex_identifier_or_variable(c, start)?,
            c if c.is_ascii_digit() => self.lex_number(c),
            other => {
                return Err(LexerError {
                    message: format!("unexpected character `{other}`"),
                    span: Span {
                        start,
                        end: self.cursor,
                    },
                });
            }
        };

        Ok(Token {
            kind,
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn lex_directive(&mut self, start: usize) -> Result<TokenKind, LexerError> {
        let word = self.take_while(|c| c.is_ascii_alphabetic());
        match word.as_str() {
            "ENTRY" => Ok(TokenKind::Entry),
            "EXTERN" | "EXTRN" | "EXTERNAL" => Ok(TokenKind::Extern),
            _ => Err(LexerError {
                message: format!("unsupported directive `${word}`"),
                span: Span {
                    start,
                    end: self.cursor,
                },
            }),
        }
    }

    fn lex_identifier_or_variable(
        &mut self,
        first: char,
        start: usize,
    ) -> Result<TokenKind, LexerError> {
        if matches!(first, 's' | 't' | 'e') && self.peek() == Some('.') {
            self.bump();
            let name = self.take_while(is_ident_continue);
            if name.is_empty() {
                return Err(LexerError {
                    message: format!("variable `{first}.` is missing a name"),
                    span: Span {
                        start,
                        end: self.cursor,
                    },
                });
            }
            return Ok(TokenKind::Variable { kind: first, name });
        }

        let mut ident = String::from(first);
        ident.push_str(&self.take_while(is_ident_continue));
        Ok(TokenKind::Identifier(ident))
    }

    fn lex_number(&mut self, first: char) -> TokenKind {
        let mut number = String::from(first);
        number.push_str(&self.take_while(|c| c.is_ascii_digit()));
        TokenKind::Number(number)
    }

    fn lex_quoted_chars(&mut self, start: usize, delimiter: char) -> Result<Token, LexerError> {
        let mut chars = Vec::new();
        while let Some(ch) = self.bump() {
            if ch == delimiter {
                if chars.is_empty() {
                    return Err(LexerError {
                        message: "empty character literal".to_string(),
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }

                let mut chars = chars.into_iter();
                let first = chars.next().expect("checked non-empty literal");
                for ch in chars {
                    self.pending.push_back(Token {
                        kind: TokenKind::Char(ch),
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }

                return Ok(Token {
                    kind: TokenKind::Char(first),
                    span: Span {
                        start,
                        end: self.cursor,
                    },
                });
            }
            chars.push(ch);
        }

        Err(LexerError {
            message: "unterminated character literal".to_string(),
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn skip_ignored(&mut self) -> Result<(), LexerError> {
        loop {
            self.take_while(char::is_whitespace);
            if self.peek() == Some('*') && self.at_line_start() {
                self.take_while(|ch| ch != '\n');
                continue;
            }
            if self.source[self.cursor..].starts_with("/*") {
                let start = self.cursor;
                self.cursor += 2;
                while self.cursor < self.source.len()
                    && !self.source[self.cursor..].starts_with("*/")
                {
                    self.bump();
                }
                if self.source[self.cursor..].starts_with("*/") {
                    self.cursor += 2;
                } else {
                    return Err(LexerError {
                        message: "unterminated block comment".to_string(),
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                continue;
            }
            return Ok(());
        }
    }

    fn at_line_start(&self) -> bool {
        self.source[..self.cursor]
            .rsplit_once('\n')
            .map_or(self.cursor == 0, |(_, prefix)| prefix.trim().is_empty())
    }

    fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }
            value.push(ch);
            self.bump();
        }
        value
    }

    fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '-'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_basic_refal_function() {
        let tokens = Lexer::new("$ENTRY Go { (e.1) = e.1; }").tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Entry);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("Go".to_string()));
        assert_eq!(
            tokens[4].kind,
            TokenKind::Variable {
                kind: 'e',
                name: "1".to_string()
            }
        );
    }

    #[test]
    fn tokenizes_extern_directive_aliases() {
        let tokens = Lexer::new("$EXTERN Prout; $EXTRN Card; $EXTERNAL Open;")
            .tokenize()
            .unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Extern);
        assert_eq!(tokens[3].kind, TokenKind::Extern);
        assert_eq!(tokens[6].kind, TokenKind::Extern);
    }

    #[test]
    fn tokenizes_quoted_text_as_character_sequence() {
        let tokens = Lexer::new("'OK'").tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Char('O'));
        assert_eq!(tokens[1].kind, TokenKind::Char('K'));
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn rejects_empty_variable_name() {
        let error = Lexer::new("e.").tokenize().unwrap_err();
        assert!(error.message.contains("missing a name"));
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        let error = Lexer::new("$ENTRY Go { =; } /* unfinished")
            .tokenize()
            .unwrap_err();

        assert_eq!(error.message, "unterminated block comment");
        assert_eq!(error.span, Span { start: 17, end: 30 });
    }

    #[test]
    fn ignores_classic_line_comments() {
        let tokens = Lexer::new("* module entry\n$ENTRY Go { =; }")
            .tokenize()
            .unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Entry);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("Go".to_string()));
    }

    #[test]
    fn tokenizes_double_quoted_text_as_character_sequence() {
        let tokens = Lexer::new("\"OK\"").tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Char('O'));
        assert_eq!(tokens[1].kind, TokenKind::Char('K'));
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }
}
