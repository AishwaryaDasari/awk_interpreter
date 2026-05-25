use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process;

use awk_interpreter::interp::Interp;
use awk_interpreter::lexer::Lexer;
use awk_interpreter::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: awk '{{print}}' file.txt");
        process::exit(1);
    }

    let program = &args[1];
    let files: Vec<&str> = if args.len() > 2 {
        args[2..].iter().map(|s| s.as_str()).collect()
    } else {
        vec![]
    };

    let lexer = Lexer::new(program);
    let mut parser = Parser::new(lexer);
    let node = match parser.parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("syntax error: {}", e);
            process::exit(1);
        }
    };

    if files.is_empty() {
        let stdin = io::stdin();
        let mut interp = Interp::new();
        for line in stdin.lock().lines() {
            if let Ok(l) = line {
                let out = interp.exec(&node, &l);
                print!("{}", out);
            }
        }
    } else {
        let mut interp = Interp::new();
        for file in &files {
            let content = fs::read_to_string(file).unwrap_or_else(|e| {
                eprintln!("error reading {}: {}", file, e);
                String::new()
            });
            for line in content.lines() {
                let out = interp.exec(&node, line);
                print!("{}", out);
            }
        }
    }
}
