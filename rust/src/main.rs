use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process;

use awk_rust::ast::{Node, Rule};
use awk_rust::interp::{Flow, Interp};
use awk_rust::lexer::Lexer;
use awk_rust::parser::Parser;
use awk_rust::value::Value;

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
    let mut field_sep: Option<String> = None;
    let mut program: Option<String> = None;
    let mut vars: Vec<(String, String)> = Vec::new();

    while i < args.len() {
        let arg = &args[i];
        if !arg.starts_with('-') || arg == "--" {
            if arg == "--" {
                i += 1;
            }
            break;
        }
        match arg.as_str() {
            "-F" => {
                i += 1;
                if i < args.len() {
                    field_sep = Some(args[i].clone());
                    i += 1;
                } else {
                    eprintln!("error: -F requires an argument");
                    process::exit(1);
                }
            }
            "-v" => {
                i += 1;
                if i < args.len() {
                    let eq = args[i].find('=');
                    if let Some(pos) = eq {
                        vars.push((args[i][..pos].to_string(), args[i][pos + 1..].to_string()));
                    } else {
                        eprintln!("error: -v requires var=value format");
                        process::exit(1);
                    }
                    i += 1;
                } else {
                    eprintln!("error: -v requires an argument");
                    process::exit(1);
                }
            }
            "-f" => {
                i += 1;
                if i < args.len() {
                    match fs::read_to_string(&args[i]) {
                        Ok(c) => program = Some(c),
                        Err(e) => {
                            eprintln!("error reading script file: {}", e);
                            process::exit(1);
                        }
                    }
                    i += 1;
                } else {
                    eprintln!("error: -f requires an argument");
                    process::exit(1);
                }
            }
            _ => {
                if arg.starts_with("-F") {
                    field_sep = Some(arg[2..].to_string());
                    i += 1;
                } else if arg.starts_with("-v") {
                    let val = arg[2..].to_string();
                    let eq = val.find('=');
                    if let Some(pos) = eq {
                        vars.push((val[..pos].to_string(), val[pos + 1..].to_string()));
                    } else {
                        eprintln!("error: -v requires var=value format");
                        process::exit(1);
                    }
                    i += 1;
                } else if arg.starts_with("-f") {
                    let fname = arg[2..].to_string();
                    match fs::read_to_string(&fname) {
                        Ok(c) => program = Some(c),
                        Err(e) => {
                            eprintln!("error reading script file: {}", e);
                            process::exit(1);
                        }
                    }
                    i += 1;
                } else {
                    eprintln!("unknown option: {}", arg);
                    process::exit(1);
                }
            }
        }
    }

    let program = if let Some(p) = program {
        p
    } else {
        if i >= args.len() {
            eprintln!("usage: awk '{{print}}' file.txt");
            process::exit(1);
        }
        let p = args[i].clone();
        i += 1;
        p
    };

    let files: Vec<&str> = args[i..].iter().map(|s| s.as_str()).collect();

    let lexer = Lexer::new(&program);
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

    for (name, val) in &vars {
        interp.env.set_var(name, Value::Str(val.clone()));
    }

    let mut exit_requested = false;

    let (begin_out, begin_flow) = interp.exec_begin(&node);
    print!("{}", begin_out);
    if matches!(begin_flow, Flow::Exit) {
        exit_requested = true;
    }

    if !exit_requested {
        if files.is_empty() {
            if needs_input(&node) {
                interp.env.set_filename("");
                interp.env.reset_fnr();
                for line in io::stdin().lock().lines() {
                    if let Ok(l) = line {
                        let (out, flow) = interp.exec_line(&node, &l);
                        print!("{}", out);
                        match flow {
                            Flow::Next => continue,
                            Flow::Exit => break,
                            _ => {}
                        }
                    }
                }
            }
        } else {
            for file in &files {
                if exit_requested {
                    break;
                }

                interp.env.set_filename(file);
                interp.env.reset_fnr();

                let (bf_out, bf_flow) = interp.exec_begin_file(&node);
                print!("{}", bf_out);
                if matches!(bf_flow, Flow::Exit) {
                    break;
                }

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
                    let (out, flow) = interp.exec_line(&node, line);
                    print!("{}", out);
                    match flow {
                        Flow::Next => continue,
                        Flow::Exit => {
                            exit_requested = true;
                            break;
                        }
                        _ => {}
                    }
                }

                if exit_requested {
                    break;
                }

                let (ef_out, ef_flow) = interp.exec_end_file(&node);
                print!("{}", ef_out);
                if matches!(ef_flow, Flow::Exit) {
                    break;
                }
            }
        }
    }

    let (end_out, _) = interp.exec_end(&node);
    print!("{}", end_out);

    if let Some(code) = interp.exit_code {
        process::exit(code as i32);
    }
}
