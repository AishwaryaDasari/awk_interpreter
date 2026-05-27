use crate::ast::{BinOpKind, Expr, Node, Pattern, Rule, Stmt, UnaryOpKind};
use crate::builtins::Env;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    None,
    Break,
    Continue,
    Next,
    Exit,
}

pub struct Interp {
    pub env: Env,
    pub in_range: bool,
    pub exit_code: Option<f64>,
    regex_cache: regex::Regex,
}

impl Interp {
    pub fn new() -> Self {
        Interp {
            env: Env::new(),
            in_range: false,
            exit_code: None,
            regex_cache: regex::Regex::new("").unwrap(),
        }
    }

    pub fn exec_begin(&mut self, node: &Node) -> (String, Flow) {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::Begin(stmts) = rule {
                let flow = self.exec_stmts(stmts, &mut out);
                match flow {
                    Flow::None | Flow::Break | Flow::Continue | Flow::Next => {}
                    Flow::Exit => return (out, flow),
                }
            }
        }
        (out, Flow::None)
    }

    pub fn exec_begin_file(&mut self, node: &Node) -> (String, Flow) {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::BeginFile(stmts) = rule {
                let flow = self.exec_stmts(stmts, &mut out);
                match flow {
                    Flow::None | Flow::Break | Flow::Continue | Flow::Next => {}
                    Flow::Exit => return (out, flow),
                }
            }
        }
        (out, Flow::None)
    }

    pub fn exec_end_file(&mut self, node: &Node) -> (String, Flow) {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::EndFile(stmts) = rule {
                let flow = self.exec_stmts(stmts, &mut out);
                match flow {
                    Flow::None | Flow::Break | Flow::Continue | Flow::Next => {}
                    Flow::Exit => return (out, flow),
                }
            }
        }
        (out, Flow::None)
    }

    pub fn exec_end(&mut self, node: &Node) -> (String, Flow) {
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::End(stmts) = rule {
                let flow = self.exec_stmts(stmts, &mut out);
                match flow {
                    Flow::None | Flow::Break | Flow::Continue | Flow::Next => {}
                    Flow::Exit => return (out, flow),
                }
            }
        }
        (out, Flow::None)
    }

    pub fn exec_line(&mut self, node: &Node, line: &str) -> (String, Flow) {
        self.env.set_line(line);
        let mut out = String::new();
        let Node::Program(rules) = node;
        for rule in rules {
            if let Rule::Pattern(pattern, stmts) = rule {
                if self.matches_pattern(pattern) {
                    let flow = self.exec_stmts(stmts, &mut out);
                    match flow {
                        Flow::None | Flow::Break | Flow::Continue => {}
                        Flow::Next => return (out, Flow::Next),
                        Flow::Exit => return (out, Flow::Exit),
                    }
                }
            }
        }
        (out, Flow::None)
    }

    fn exec_stmts(&mut self, stmts: &[Stmt], out: &mut String) -> Flow {
        for stmt in stmts {
            let flow = match stmt {
                Stmt::Print(exprs) => {
                    self.exec_print(exprs, out);
                    Flow::None
                }
                Stmt::Expr(expr) => {
                    self.eval_expr(expr);
                    Flow::None
                }
                Stmt::If(cond, then_body, else_body) => {
                    self.exec_if(cond, then_body, else_body, out)
                }
                Stmt::While(cond, body) => self.exec_while(cond, body, out),
                Stmt::DoWhile(body, cond) => self.exec_do_while(body, cond, out),
                Stmt::For(init, cond, incr, body) => self.exec_for(init, cond, incr, body, out),
                Stmt::ForIn(_, _, _) => Flow::None,
                Stmt::Break => Flow::Break,
                Stmt::Continue => Flow::Continue,
                Stmt::Next => Flow::Next,
                Stmt::Exit(expr) => self.exec_exit(expr),
            };
            if !matches!(flow, Flow::None) {
                return flow;
            }
        }
        Flow::None
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

    fn exec_if(
        &mut self,
        cond: &Expr,
        then_body: &[Stmt],
        else_body: &[Stmt],
        out: &mut String,
    ) -> Flow {
        let cv = self.eval_expr(cond);
        if self.is_truthy(&cv) {
            self.exec_stmts(then_body, out)
        } else if !else_body.is_empty() {
            self.exec_stmts(else_body, out)
        } else {
            Flow::None
        }
    }

    fn exec_while(&mut self, cond: &Expr, body: &[Stmt], out: &mut String) -> Flow {
        loop {
            let cv = self.eval_expr(cond);
            if !self.is_truthy(&cv) {
                return Flow::None;
            }
            let flow = self.exec_stmts(body, out);
            match flow {
                Flow::None | Flow::Continue => {}
                Flow::Break => return Flow::None,
                Flow::Next => return Flow::Next,
                Flow::Exit => return Flow::Exit,
            }
        }
    }

    fn exec_do_while(&mut self, body: &[Stmt], cond: &Expr, out: &mut String) -> Flow {
        loop {
            let flow = self.exec_stmts(body, out);
            match flow {
                Flow::None | Flow::Continue => {
                    let cv = self.eval_expr(cond);
                    if !self.is_truthy(&cv) {
                        return Flow::None;
                    }
                }
                Flow::Break => return Flow::None,
                Flow::Next => return Flow::Next,
                Flow::Exit => return Flow::Exit,
            }
        }
    }

    fn exec_for(
        &mut self,
        init: &Option<Expr>,
        cond: &Option<Expr>,
        incr: &Option<Expr>,
        body: &[Stmt],
        out: &mut String,
    ) -> Flow {
        if let Some(init_expr) = init {
            self.eval_expr(init_expr);
        }
        loop {
            if let Some(cond_expr) = cond {
                let cv = self.eval_expr(cond_expr);
                if !self.is_truthy(&cv) {
                    return Flow::None;
                }
            }
            let flow = self.exec_stmts(body, out);
            match flow {
                Flow::None | Flow::Continue => {
                    if let Some(incr_expr) = incr {
                        self.eval_expr(incr_expr);
                    }
                }
                Flow::Break => return Flow::None,
                Flow::Next => return Flow::Next,
                Flow::Exit => return Flow::Exit,
            }
        }
    }

    fn exec_exit(&mut self, expr: &Option<Expr>) -> Flow {
        if let Some(e) = expr {
            let v = self.eval_expr(e);
            self.exit_code = Some(v.as_f64());
        } else if self.exit_code.is_none() {
            self.exit_code = Some(0.0);
        }
        Flow::Exit
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
            Expr::Assign(name, value) => {
                let val = self.eval_expr(value);
                self.env.set_var(name, val.clone());
                val
            }
            Expr::AssignOp(kind, name, value) => {
                let lhs = self.env.get_var(name);
                let rhs = self.eval_expr(value);
                let result = match kind {
                    BinOpKind::Add => Value::Num(lhs.as_f64() + rhs.as_f64()),
                    BinOpKind::Sub => Value::Num(lhs.as_f64() - rhs.as_f64()),
                    BinOpKind::Mul => Value::Num(lhs.as_f64() * rhs.as_f64()),
                    BinOpKind::Div => Value::Num(lhs.as_f64() / rhs.as_f64()),
                    BinOpKind::Mod => Value::Num(lhs.as_f64() % rhs.as_f64()),
                    _ => unreachable!(),
                };
                self.env.set_var(name, result.clone());
                result
            }
            Expr::Incr(name, post) => {
                let val = self.env.get_var(name);
                let new_val = Value::Num(val.as_f64() + 1.0);
                self.env.set_var(name, new_val.clone());
                if *post {
                    val
                } else {
                    new_val
                }
            }
            Expr::Decr(name, post) => {
                let val = self.env.get_var(name);
                let new_val = Value::Num(val.as_f64() - 1.0);
                self.env.set_var(name, new_val.clone());
                if *post {
                    val
                } else {
                    new_val
                }
            }
        }
    }

    fn eval_binop(&mut self, lhs: &Expr, kind: &BinOpKind, rhs: &Expr) -> Value {
        match kind {
            BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Mod => {
                let lv = self.eval_expr(lhs);
                let rv = self.eval_expr(rhs);
                Value::Num(match kind {
                    BinOpKind::Add => lv.as_f64() + rv.as_f64(),
                    BinOpKind::Sub => lv.as_f64() - rv.as_f64(),
                    BinOpKind::Mul => lv.as_f64() * rv.as_f64(),
                    BinOpKind::Div => lv.as_f64() / rv.as_f64(),
                    BinOpKind::Mod => lv.as_f64() % rv.as_f64(),
                    _ => unreachable!(),
                })
            }
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
            UnaryOpKind::Neg => {
                let v = self.eval_expr(expr);
                Value::Num(-v.as_f64())
            }
            UnaryOpKind::Not => {
                let v = self.eval_expr(expr);
                Value::Num(if self.is_truthy(&v) { 0.0 } else { 1.0 })
            }
        }
    }
}
