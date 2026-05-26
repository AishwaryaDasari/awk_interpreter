use crate::ast::{BinOpKind, Expr, Node, Pattern, Rule, Stmt, UnaryOpKind};
use crate::builtins::Env;
use crate::value::Value;

pub struct Interp {
    pub env: Env,
    pub in_range: bool,
    regex_cache: regex::Regex,
}

impl Interp {
    pub fn new() -> Self {
        Interp {
            env: Env::new(),
            in_range: false,
            regex_cache: regex::Regex::new("").unwrap(),
        }
    }

    pub fn exec_begin(&mut self, node: &Node) -> String {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::Begin(stmts) = rule {
                self.exec_stmts(stmts, &mut out);
            }
        }
        out
    }

    pub fn exec_begin_file(&mut self, node: &Node) -> String {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::BeginFile(stmts) = rule {
                self.exec_stmts(stmts, &mut out);
            }
        }
        out
    }

    pub fn exec_end_file(&mut self, node: &Node) -> String {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::EndFile(stmts) = rule {
                self.exec_stmts(stmts, &mut out);
            }
        }
        out
    }

    pub fn exec_end(&mut self, node: &Node) -> String {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::End(stmts) = rule {
                self.exec_stmts(stmts, &mut out);
            }
        }
        out
    }

    pub fn exec_line(&mut self, node: &Node, line: &str) -> String {
        self.env.set_line(line, " ");
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::Pattern(pattern, stmts) = rule {
                if self.matches_pattern(pattern) {
                    self.exec_stmts(stmts, &mut out);
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

    fn exec_print(&mut self, exprs: &[Expr], out: &mut String) {
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

    fn matches_pattern(&mut self, pattern: &Pattern) -> bool {
        match pattern {
            Pattern::Expr(expr) => {
                let v = self.eval_expr(expr);
                self.is_truthy(&v)
            }
            Pattern::Range(start, end) => {
                if self.in_range {
                    let v = self.eval_expr(end);
                    if self.is_truthy(&v) {
                        self.in_range = false;
                    }
                    true
                } else {
                    let v = self.eval_expr(start);
                    if self.is_truthy(&v) {
                        self.in_range = true;
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Num(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
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
            Expr::BinOp(lhs, kind, rhs) => self.eval_binop(lhs, kind, rhs),
            Expr::UnaryOp(kind, expr) => self.eval_unaryop(kind, expr),
            Expr::Ternary(cond, t, f) => {
                let cv = self.eval_expr(cond);
                if self.is_truthy(&cv) {
                    self.eval_expr(t)
                } else {
                    self.eval_expr(f)
                }
            }
        }
    }

    fn eval_binop(&mut self, lhs: &Expr, kind: &BinOpKind, rhs: &Expr) -> Value {
        match kind {
            BinOpKind::Concat => {
                let lv = self.eval_expr(lhs);
                let rv = self.eval_expr(rhs);
                Value::Str(lv.as_str() + &rv.as_str())
            }
            BinOpKind::And => {
                let lv = self.eval_expr(lhs);
                if !self.is_truthy(&lv) {
                    return lv;
                }
                self.eval_expr(rhs)
            }
            BinOpKind::Or => {
                let lv = self.eval_expr(lhs);
                if self.is_truthy(&lv) {
                    return lv;
                }
                self.eval_expr(rhs)
            }
            BinOpKind::Match | BinOpKind::NotMatch => {
                let text = self.eval_expr(lhs).as_str();
                let re_str = self.eval_expr(rhs).as_str();
                let matched = if let Ok(r) = regex::Regex::new(&re_str) {
                    self.regex_cache = r;
                    self.regex_cache.is_match(&text)
                } else {
                    false
                };
                Value::Num(if matched == (*kind == BinOpKind::Match) {
                    1.0
                } else {
                    0.0
                })
            }
            BinOpKind::Eq | BinOpKind::Ne => {
                let lv = self.eval_expr(lhs);
                let rv = self.eval_expr(rhs);
                let equal = match (&lv, &rv) {
                    (Value::Num(a), Value::Num(b)) => (a - b).abs() < f64::EPSILON,
                    (Value::Str(a), Value::Str(b)) => a == b,
                    (Value::Num(_), Value::Str(_)) => lv.as_str() == rv.as_str(),
                    (Value::Str(_), Value::Num(_)) => lv.as_str() == rv.as_str(),
                };
                Value::Num(if equal == (*kind == BinOpKind::Eq) {
                    1.0
                } else {
                    0.0
                })
            }
            BinOpKind::Lt | BinOpKind::Gt | BinOpKind::Le | BinOpKind::Ge => {
                let lv = self.eval_expr(lhs);
                let rv = self.eval_expr(rhs);
                let result = match (&lv, &rv) {
                    (Value::Num(a), Value::Num(b)) => {
                        let cmp = a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal);
                        match kind {
                            BinOpKind::Lt => cmp.is_lt(),
                            BinOpKind::Gt => cmp.is_gt(),
                            BinOpKind::Le => cmp.is_le(),
                            BinOpKind::Ge => cmp.is_ge(),
                            _ => false,
                        }
                    }
                    (Value::Str(a), Value::Str(b)) => {
                        let cmp = a.cmp(b);
                        match kind {
                            BinOpKind::Lt => cmp.is_lt(),
                            BinOpKind::Gt => cmp.is_gt(),
                            BinOpKind::Le => cmp.is_le(),
                            BinOpKind::Ge => cmp.is_ge(),
                            _ => false,
                        }
                    }
                    (Value::Num(a), Value::Str(b)) => {
                        if let Ok(bn) = b.parse::<f64>() {
                            let cmp = a.partial_cmp(&bn).unwrap_or(std::cmp::Ordering::Equal);
                            match kind {
                                BinOpKind::Lt => cmp.is_lt(),
                                BinOpKind::Gt => cmp.is_gt(),
                                BinOpKind::Le => cmp.is_le(),
                                BinOpKind::Ge => cmp.is_ge(),
                                _ => false,
                            }
                        } else {
                            let as_str = &lv.as_str();
                            let cmp = as_str.cmp(b);
                            match kind {
                                BinOpKind::Lt => cmp.is_lt(),
                                BinOpKind::Gt => cmp.is_gt(),
                                BinOpKind::Le => cmp.is_le(),
                                BinOpKind::Ge => cmp.is_ge(),
                                _ => false,
                            }
                        }
                    }
                    (Value::Str(a), Value::Num(b)) => {
                        if let Ok(an) = a.parse::<f64>() {
                            let cmp = an.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal);
                            match kind {
                                BinOpKind::Lt => cmp.is_lt(),
                                BinOpKind::Gt => cmp.is_gt(),
                                BinOpKind::Le => cmp.is_le(),
                                BinOpKind::Ge => cmp.is_ge(),
                                _ => false,
                            }
                        } else {
                            let b_str = &rv.as_str();
                            let cmp = a.cmp(b_str);
                            match kind {
                                BinOpKind::Lt => cmp.is_lt(),
                                BinOpKind::Gt => cmp.is_gt(),
                                BinOpKind::Le => cmp.is_le(),
                                BinOpKind::Ge => cmp.is_ge(),
                                _ => false,
                            }
                        }
                    }
                };
                Value::Num(if result { 1.0 } else { 0.0 })
            }
        }
    }

    fn eval_unaryop(&mut self, kind: &UnaryOpKind, expr: &Expr) -> Value {
        match kind {
            UnaryOpKind::Not => {
                let v = self.eval_expr(expr);
                Value::Num(if self.is_truthy(&v) { 0.0 } else { 1.0 })
            }
        }
    }
}
