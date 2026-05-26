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

    let mut i = 1;
    let mut field_sep = None;

    while i < args.len() && args[i].starts_with('-') {
        let arg = &args[i];
        if arg == "-F" {
            i += 1;
            if i < args.len() {
                field_sep = Some(args[i].clone());
                i += 1;
            } else {
                eprintln!("error: -F requires an argument");
                process::exit(1);
            }
        } else if arg.starts_with("-F") {
            let sep = arg[2..].to_string();
            field_sep = Some(sep);
            i += 1;
        } else {
            eprintln!("unknown option: {}", arg);
            process::exit(1);
        }
    }

    if i >= args.len() {
        eprintln!("usage: awk '{{print}}' file.txt");
        process::exit(1);
    }

    let program = &args[i];
    i += 1;
    let files: Vec<&str> = args[i..].iter().map(|s| s.as_str()).collect();

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

    if let Some(sep) = field_sep {
        interp.env.fs = sep;
    }

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
