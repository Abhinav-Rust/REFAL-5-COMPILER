use std::{env, fs, process};

use refal_ast::Span as AstSpan;
use refal_syntax::{Lexer, Parser};

fn main() {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!("Usage: refal <check|dump-ast> <file.ref>");
        process::exit(2);
    };
    let Some(path) = args.next() else {
        eprintln!("missing input file");
        process::exit(2);
    };

    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(&source).tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!(
                "{}",
                render_diagnostic("lex error", &source, error.span.start, &error.message)
            );
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "{}",
                render_diagnostic("parse error", &source, error.span.start, &error.message)
            );
            process::exit(1);
        }
    };

    if let Err(diagnostics) = refal_semantics::check_program(&program) {
        for diagnostic in diagnostics {
            eprintln!(
                "{}",
                render_ast_diagnostic(
                    "semantic error",
                    &source,
                    diagnostic.span,
                    &diagnostic.message
                )
            );
        }
        process::exit(1);
    }

    match command.as_str() {
        "check" => println!("{path}: check ok"),
        "dump-ast" => println!("{program:#?}"),
        other => {
            eprintln!("unknown command `{other}`");
            process::exit(2);
        }
    }
}

fn render_ast_diagnostic(kind: &str, source: &str, span: AstSpan, message: &str) -> String {
    render_diagnostic(kind, source, span.start, message)
}

fn render_diagnostic(kind: &str, source: &str, offset: usize, message: &str) -> String {
    let position = SourceMap::new(source).position(offset);
    format!("{kind} at {}:{}: {message}", position.line, position.column)
}

struct SourceMap<'a> {
    source: &'a str,
}

impl<'a> SourceMap<'a> {
    fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn position(&self, offset: usize) -> SourcePosition {
        let mut line = 1;
        let mut column = 1;

        for (index, ch) in self.source.char_indices() {
            if index >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        SourcePosition { line, column }
    }
}

struct SourcePosition {
    line: usize,
    column: usize,
}
