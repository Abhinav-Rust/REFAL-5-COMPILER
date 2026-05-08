use crate::evaluator::{Evaluator, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref OPEN_FILES: Mutex<HashMap<i64, File>> = Mutex::new(HashMap::new());
}

pub fn call_builtin(_evaluator: &Evaluator, name: &str, args: Vec<Value>) -> Result<Vec<Value>, String> {
    match name {
        "Prout" => {
            let mut out = String::new();
            for arg in &args {
                match arg {
                    Value::Symbol(s) => out.push_str(s),
                    Value::Number(n) => out.push_str(&n.to_string()),
                    Value::StringLit(s) => out.push_str(s),
                    Value::Bracket(inner) => out.push_str(&format!("({:?})", inner)), // Simple rep for now
                }
                out.push(' ');
            }
            println!("{}", out.trim_end());
            Ok(vec![])
        }
        "Card" => {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let mut vals = vec![];
                    // Very simple naive string to characters parsing for Card right now
                    for ch in input.trim_end().chars() {
                         vals.push(Value::StringLit(ch.to_string()));
                    }
                    Ok(vals)
                }
                Err(_) => Ok(vec![]),
            }
        }
        "Open" => {
            // <Open 'r' 1 'filename.txt'>
            if args.len() < 3 {
                return Err("Open expects at least 3 arguments".to_string());
            }
            let mode = match &args[0] {
                Value::Symbol(s) | Value::StringLit(s) => s.as_str(),
                _ => return Err("Open mode must be string or symbol".to_string()),
            };
            let fd = match &args[1] {
                Value::Number(n) => *n,
                _ => return Err("Open fd must be a number".to_string()),
            };
            let mut filename = String::new();
            for arg in &args[2..] {
                match arg {
                    Value::Symbol(s) | Value::StringLit(s) => filename.push_str(s),
                    Value::Number(n) => filename.push_str(&n.to_string()),
                    _ => return Err("Open filename must be string/symbol/number parts".to_string()),
                }
            }

            let file_res = match mode {
                "r" => File::open(&filename),
                "w" => OpenOptions::new().write(true).create(true).truncate(true).open(&filename),
                "a" => OpenOptions::new().append(true).create(true).open(&filename),
                _ => return Err(format!("Unknown mode '{}'", mode)),
            };

            match file_res {
                Ok(file) => {
                    let mut map = OPEN_FILES.lock().unwrap();
                    map.insert(fd, file);
                    Ok(vec![])
                }
                Err(e) => Err(format!("Failed to open file {}: {}", filename, e)),
            }
        }
        "Get" => {
             // <Get 1>
             if args.len() != 1 {
                 return Err("Get expects 1 argument".to_string());
             }
             let fd = match &args[0] {
                 Value::Number(n) => *n,
                 _ => return Err("Get fd must be a number".to_string()),
             };

             let mut map = OPEN_FILES.lock().unwrap();
             if let Some(file) = map.get_mut(&fd) {
                 let mut contents = String::new();
                 // Very simplistic: reading entire file. In real Refal-5 <Get> reads one line/expr
                 // Modifying to read a line would require BufReader caching
                 match file.read_to_string(&mut contents) {
                     Ok(_) => {
                         let mut vals = vec![];
                         for ch in contents.chars() {
                             vals.push(Value::StringLit(ch.to_string()));
                         }
                         Ok(vals)
                     }
                     Err(e) => Err(format!("Failed to read fd {}: {}", fd, e)),
                 }
             } else {
                 Err(format!("Invalid fd {}", fd))
             }
        }
        "Put" => {
             // <Put 1 e.Data>
             if args.is_empty() {
                 return Err("Put expects at least 1 argument".to_string());
             }
             let fd = match &args[0] {
                 Value::Number(n) => *n,
                 _ => return Err("Put fd must be a number".to_string()),
             };

             let mut out = String::new();
             for arg in &args[1..] {
                 match arg {
                     Value::Symbol(s) => out.push_str(s),
                     Value::Number(n) => out.push_str(&n.to_string()),
                     Value::StringLit(s) => out.push_str(s),
                     Value::Bracket(inner) => out.push_str(&format!("{:?}", inner)),
                 }
             }

             let mut map = OPEN_FILES.lock().unwrap();
             if let Some(file) = map.get_mut(&fd) {
                 if let Err(e) = file.write_all(out.as_bytes()) {
                      return Err(format!("Failed to write fd {}: {}", fd, e));
                 }
                 Ok(vec![])
             } else {
                 Err(format!("Invalid fd {}", fd))
             }
        }
        "Close" => {
             if args.len() != 1 {
                 return Err("Close expects 1 argument".to_string());
             }
             let fd = match &args[0] {
                 Value::Number(n) => *n,
                 _ => return Err("Close fd must be a number".to_string()),
             };
             let mut map = OPEN_FILES.lock().unwrap();
             map.remove(&fd);
             Ok(vec![])
        }
        _ => Err(format!("Unknown built-in function: {}", name)),
    }
}
