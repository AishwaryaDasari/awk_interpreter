use crate::ast::{Expr, Node, Stmt};
use crate::builtins::Env;
use crate::value::Value;

pub struct Interp {
    pub env: Env,
}

impl Interp {
    pub fn new() -> Self {
        Interp {
            env: Env::new(),
        }
    }

    pub fn exec(&mut self, node: &Node, line: &str) -> String {
        self.env.set_line(line, " ");
        match node {
            Node::Program(stmts) => {
                let mut out = String::new();
                for stmt in stmts {
                    match stmt {
                        Stmt::Print(exprs) => {
                            self.exec_print(exprs, &mut out);
                        }
                    }
                }
                out
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
