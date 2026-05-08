use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod evaluator;
mod builtins;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: seed <filename.ref>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        std::process::exit(1);
    });

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    // Find $ENTRY function (usually named Go, but we can search for the first one marked entry)
    let entry_func = program.functions
        .iter()
        .find(|f| f.is_entry)
        .map(|f| f.name.clone())
        .unwrap_or_else(|| "Go".to_string());

    let evaluator = Evaluator::new(program);

    if let Err(e) = evaluator.evaluate(&entry_func, vec![]) {
        eprintln!("Runtime error: {}", e);
        std::process::exit(1);
    }
}
