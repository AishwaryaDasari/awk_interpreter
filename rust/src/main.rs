use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process;

use awk_rust::ast::{Node, Rule};
use awk_rust::interp::Interp;
use awk_rust::lexer::Lexer;
use awk_rust::parser::Parser;

fn needs_input(node: &Node) -> bool {
    let Node::Program(rules) = node;
    rules.iter().any(|r| {
        matches!(
            r,
            Rule::Pattern(..) | Rule::BeginFile(..) | Rule::EndFile(..)
        )
    })
}

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

    let mut interp = Interp::new();

    let begin_out = interp.exec_begin(&node);
    print!("{}", begin_out);

    if files.is_empty() {
        if needs_input(&node) {
            interp.env.set_filename("");
            for line in io::stdin().lock().lines() {
                if let Ok(l) = line {
                    let out = interp.exec_line(&node, &l);
                    print!("{}", out);
                }
            }
        }
    } else {
        for file in &files {
            interp.env.set_filename(file);

            let bf_out = interp.exec_begin_file(&node);
            print!("{}", bf_out);

            let content = match fs::read_to_string(file) {
                Ok(c) => c,
                Err(e) => {
                    let err = e.to_string();
                    let msg = err.split('(').next().unwrap_or("").trim();
                    eprintln!("{}", msg);
                    continue;
                }
            };
            interp.in_range = false;
            for line in content.lines() {
                let out = interp.exec_line(&node, line);
                print!("{}", out);
            }

            let ef_out = interp.exec_end_file(&node);
            print!("{}", ef_out);
        }
    }

    let end_out = interp.exec_end(&node);
    print!("{}", end_out);
}
