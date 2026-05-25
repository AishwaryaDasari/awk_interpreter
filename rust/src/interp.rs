use crate::ast::{Expr, Node, Pattern, Rule, Stmt};
use crate::builtins::Env;
use crate::value::Value;

pub struct Interp {
    pub env: Env,
    regex_cache: regex::Regex,
}

impl Interp {
    pub fn new() -> Self {
        Interp {
            env: Env::new(),
            regex_cache: regex::Regex::new("").unwrap(),
        }
    }

    pub fn exec_begin(&mut self, node: &Node) -> String {
        let mut out = String::new();
        match node {
            Node::Program(rules) => {
                for rule in rules {
                    if let Rule::Begin(stmts) = rule {
                        self.exec_stmts(stmts, &mut out);
                    }
                }
            }
        }
        out
    }

    pub fn exec_line(&mut self, node: &Node, line: &str) -> String {
        self.env.set_line(line, " ");
        let mut out = String::new();
        match node {
            Node::Program(rules) => {
                for rule in rules {
                    match rule {
                        Rule::Pattern(pattern, stmts) => {
                            if self.matches(pattern, line) {
                                self.exec_stmts(stmts, &mut out);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        out
    }

    pub fn exec_end(&mut self, node: &Node) -> String {
        let mut out = String::new();
        match node {
            Node::Program(rules) => {
                for rule in rules {
                    if let Rule::End(stmts) = rule {
                        self.exec_stmts(stmts, &mut out);
                    }
                }
            }
        }
        out
    }

    fn exec_stmts(&mut self, stmts: &[Stmt], out: &mut String) {
        for stmt in stmts {
            match stmt {
                Stmt::Print(exprs) => {
                    self.exec_print(exprs, out);
                }
            }
        }
    }

    fn exec_print(&self, exprs: &[Expr], out: &mut String) {
        if exprs.is_empty() {
            out.push_str(&self.env.get_field(0).as_str());
            out.push('\n');
            return;
        }
        let mut parts = Vec::new();
        for expr in exprs {
            parts.push(self.eval_expr(expr));
        }
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(&part.as_str());
        }
        out.push('\n');
    }

    fn matches(&mut self, pattern: &Pattern, line: &str) -> bool {
        match pattern {
            Pattern::Regex(re) => {
                if re.is_empty() {
                    return true;
                }
                if let Ok(r) = regex::Regex::new(re) {
                    self.regex_cache = r;
                }
                self.regex_cache.is_match(line)
            }
        }
    }

    fn eval_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Field(n) => self.env.get_field(*n),
            Expr::FieldExpr(inner) => {
                let v = self.eval_expr(inner);
                let n = v.as_f64() as usize;
                self.env.get_field(n)
            }
            Expr::Ident(name) => self.env.get_var(name),
            Expr::Str(s) => Value::Str(s.clone()),
            Expr::Num(n) => Value::Num(*n),
        }
    }
}
