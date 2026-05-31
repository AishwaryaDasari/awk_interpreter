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
                Stmt::ForIn(var, array_expr, body) => self.exec_for_in(var, array_expr, body, out),
                Stmt::Delete(name, index) => {
                    self.exec_delete(name, index);
                    Flow::None
                }
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

    fn exec_for_in(
        &mut self,
        var: &str,
        array_expr: &Expr,
        body: &[Stmt],
        out: &mut String,
    ) -> Flow {
        let arr_name = match array_expr {
            Expr::Ident(name) => name.clone(),
            _ => return Flow::None,
        };
        let keys = self.env.get_array_keys(&arr_name);
        for key in keys {
            self.env.set_var(var, Value::Str(key));
            let flow = self.exec_stmts(body, out);
            match flow {
                Flow::None | Flow::Continue => {}
                Flow::Break => break,
                Flow::Next => return Flow::Next,
                Flow::Exit => return Flow::Exit,
            }
        }
        Flow::None
    }

    fn exec_delete(&mut self, name: &str, index: &Option<Box<Expr>>) {
        match index {
            Some(idx_expr) => {
                let idx = self.eval_expr(idx_expr).as_str();
                self.env.delete_array_entry(name, &idx);
            }
            None => {
                self.env.delete_array(name);
            }
        }
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
            Expr::Call(name, args) => self.eval_call(name, args),
            Expr::ArrayAccess(name, index) => {
                let idx = self.eval_expr(index).as_str();
                self.env.get_array(name, &idx)
            }
            Expr::ArrayAssign(name, index, value) => {
                let idx = self.eval_expr(index).as_str();
                let val = self.eval_expr(value);
                self.env.set_array(name, &idx, val.clone());
                val
            }
            Expr::ArrayAssignOp(kind, name, index, value) => {
                let idx = self.eval_expr(index).as_str();
                let lhs = self.env.get_array(name, &idx);
                let rhs = self.eval_expr(value);
                let result = match kind {
                    BinOpKind::Add => Value::Num(lhs.as_f64() + rhs.as_f64()),
                    BinOpKind::Sub => Value::Num(lhs.as_f64() - rhs.as_f64()),
                    BinOpKind::Mul => Value::Num(lhs.as_f64() * rhs.as_f64()),
                    BinOpKind::Div => Value::Num(lhs.as_f64() / rhs.as_f64()),
                    BinOpKind::Mod => Value::Num(lhs.as_f64() % rhs.as_f64()),
                    _ => unreachable!(),
                };
                self.env.set_array(name, &idx, result.clone());
                result
            }
            Expr::ArrayIncr(name, index, post) => {
                let idx = self.eval_expr(index).as_str();
                let val = self.env.get_array(name, &idx);
                let new_val = Value::Num(val.as_f64() + 1.0);
                self.env.set_array(name, &idx, new_val.clone());
                if *post {
                    val
                } else {
                    new_val
                }
            }
            Expr::ArrayDecr(name, index, post) => {
                let idx = self.eval_expr(index).as_str();
                let val = self.env.get_array(name, &idx);
                let new_val = Value::Num(val.as_f64() - 1.0);
                self.env.set_array(name, &idx, new_val.clone());
                if *post {
                    val
                } else {
                    new_val
                }
            }
            Expr::In(key, arr_name) => {
                let k = self.eval_expr(key).as_str();
                Value::Num(if self.env.has_array_key(arr_name, &k) {
                    1.0
                } else {
                    0.0
                })
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

    // ---- built-in functions ----

    fn eval_call(&mut self, name: &str, args: &[Expr]) -> Value {
        match name {
            "length" => self.call_length(args),
            "substr" => self.call_substr(args),
            "toupper" | "uc" => self.call_toupper(args),
            "tolower" | "lc" => self.call_tolower(args),
            "index" => self.call_index(args),
            "split" => self.call_split(args),
            "gsub" => self.call_gsub(args),
            "sub" => self.call_sub(args),
            "gensub" => self.call_gensub(args),
            "match" => self.call_match(args),
            "sprintf" => self.call_sprintf(args),
            _ => Value::Num(0.0),
        }
    }

    fn call_length(&mut self, args: &[Expr]) -> Value {
        let s = if args.is_empty() {
            self.env.get_field(0).as_str()
        } else {
            self.eval_expr(&args[0]).as_str()
        };
        Value::Num(s.len() as f64)
    }

    fn call_substr(&mut self, args: &[Expr]) -> Value {
        let s = self.eval_expr(&args[0]).as_str();
        let start = self.eval_expr(&args[1]).as_f64() as usize;
        let start_idx = if start > 0 { start - 1 } else { 0 };
        let start_idx = start_idx.min(s.len());
        if args.len() >= 3 {
            let len = self.eval_expr(&args[2]).as_f64() as usize;
            let end = std::cmp::min(start_idx + len, s.len());
            Value::Str(s[start_idx..end].to_string())
        } else {
            Value::Str(s[start_idx..].to_string())
        }
    }

    fn call_toupper(&mut self, args: &[Expr]) -> Value {
        let s = self.eval_expr(&args[0]).as_str();
        Value::Str(s.to_uppercase())
    }

    fn call_tolower(&mut self, args: &[Expr]) -> Value {
        let s = self.eval_expr(&args[0]).as_str();
        Value::Str(s.to_lowercase())
    }

    fn call_index(&mut self, args: &[Expr]) -> Value {
        let s = self.eval_expr(&args[0]).as_str();
        let sub = self.eval_expr(&args[1]).as_str();
        match s.find(&sub) {
            Some(pos) => Value::Num((pos + 1) as f64),
            None => Value::Num(0.0),
        }
    }

    fn call_split(&mut self, args: &[Expr]) -> Value {
        let s = self.eval_expr(&args[0]).as_str();
        if s.is_empty() {
            return Value::Num(0.0);
        }
        let arr_name = match &args[1] {
            Expr::Ident(name) => name.clone(),
            _ => return Value::Num(0.0),
        };
        let sep = if args.len() >= 3 {
            self.eval_expr(&args[2]).as_str()
        } else {
            self.env.fs.clone()
        };
        let parts: Vec<String> = if sep == " " {
            s.split_whitespace().map(String::from).collect()
        } else if sep.is_empty() {
            s.chars().map(String::from).collect()
        } else {
            s.split(&sep).map(String::from).collect()
        };
        let mut arr = std::collections::HashMap::new();
        for (i, part) in parts.iter().enumerate() {
            arr.insert((i + 1).to_string(), Value::Str(part.clone()));
        }
        self.env.arrays.insert(arr_name, arr);
        Value::Num(parts.len() as f64)
    }

    fn call_gsub(&mut self, args: &[Expr]) -> Value {
        let re_str = self.eval_expr(&args[0]).as_str();
        let repl = self.eval_expr(&args[1]).as_str();

        let text_owned: String;
        let should_set_field: bool;
        let should_set_var: bool;
        let field_n: usize;
        let var_name: String;

        if args.len() >= 3 {
            match &args[2] {
                Expr::Field(n) => {
                    text_owned = self.env.get_field(*n).as_str();
                    should_set_field = true;
                    should_set_var = false;
                    field_n = *n;
                    var_name = String::new();
                }
                Expr::FieldExpr(inner) => {
                    let n = self.eval_expr(inner).as_f64() as usize;
                    text_owned = self.env.get_field(n).as_str();
                    should_set_field = true;
                    should_set_var = false;
                    field_n = n;
                    var_name = String::new();
                }
                Expr::Ident(name) => {
                    text_owned = self.env.get_var(name).as_str();
                    should_set_field = false;
                    should_set_var = true;
                    field_n = 0;
                    var_name = name.clone();
                }
                _ => {
                    text_owned = self.eval_expr(&args[2]).as_str();
                    should_set_field = false;
                    should_set_var = false;
                    field_n = 0;
                    var_name = String::new();
                }
            }
        } else {
            text_owned = self.env.get_field(0).as_str();
            should_set_field = true;
            should_set_var = false;
            field_n = 0;
            var_name = String::new();
        };

        let re = regex::Regex::new(&re_str).unwrap_or_else(|_| regex::Regex::new("").unwrap());

        let mut count = 0;
        let mut result = String::new();
        let mut last_end = 0;

        for m in re.find_iter(&text_owned) {
            count += 1;
            result.push_str(&text_owned[last_end..m.start()]);
            let expanded = repl.replace("&", m.as_str());
            result.push_str(&expanded);
            last_end = m.end();
        }
        result.push_str(&text_owned[last_end..]);

        if should_set_field {
            self.env.set_field(field_n, Value::Str(result));
        } else if should_set_var {
            self.env.set_var(&var_name, Value::Str(result));
        }

        Value::Num(count as f64)
    }

    fn call_sub(&mut self, args: &[Expr]) -> Value {
        let re_str = self.eval_expr(&args[0]).as_str();
        let repl = self.eval_expr(&args[1]).as_str();

        let text_owned: String;
        let should_set_field: bool;
        let should_set_var: bool;
        let field_n: usize;
        let var_name: String;

        if args.len() >= 3 {
            match &args[2] {
                Expr::Field(n) => {
                    text_owned = self.env.get_field(*n).as_str();
                    should_set_field = true;
                    should_set_var = false;
                    field_n = *n;
                    var_name = String::new();
                }
                Expr::FieldExpr(inner) => {
                    let n = self.eval_expr(inner).as_f64() as usize;
                    text_owned = self.env.get_field(n).as_str();
                    should_set_field = true;
                    should_set_var = false;
                    field_n = n;
                    var_name = String::new();
                }
                Expr::Ident(name) => {
                    text_owned = self.env.get_var(name).as_str();
                    should_set_field = false;
                    should_set_var = true;
                    field_n = 0;
                    var_name = name.clone();
                }
                _ => {
                    text_owned = self.eval_expr(&args[2]).as_str();
                    should_set_field = false;
                    should_set_var = false;
                    field_n = 0;
                    var_name = String::new();
                }
            }
        } else {
            text_owned = self.env.get_field(0).as_str();
            should_set_field = true;
            should_set_var = false;
            field_n = 0;
            var_name = String::new();
        };

        let re = regex::Regex::new(&re_str).unwrap_or_else(|_| regex::Regex::new("").unwrap());

        if let Some(m) = re.find(&text_owned) {
            let mut result = String::new();
            result.push_str(&text_owned[..m.start()]);
            let expanded = repl.replace("&", m.as_str());
            result.push_str(&expanded);
            result.push_str(&text_owned[m.end()..]);

            if should_set_field {
                self.env.set_field(field_n, Value::Str(result));
            } else if should_set_var {
                self.env.set_var(&var_name, Value::Str(result));
            }
            Value::Num(1.0)
        } else {
            Value::Num(0.0)
        }
    }

    fn call_gensub(&mut self, args: &[Expr]) -> Value {
        let re_str = self.eval_expr(&args[0]).as_str();
        let repl = self.eval_expr(&args[1]).as_str();
        let how = self.eval_expr(&args[2]).as_str();
        let text = if args.len() >= 4 {
            self.eval_expr(&args[3]).as_str()
        } else {
            self.env.get_field(0).as_str()
        };

        let re = regex::Regex::new(&re_str).unwrap_or_else(|_| regex::Regex::new("").unwrap());

        let result = if how == "g" {
            let mut out = String::new();
            let mut last = 0;
            for m in re.find_iter(&text) {
                out.push_str(&text[last..m.start()]);
                out.push_str(&repl.replace("&", m.as_str()));
                last = m.end();
            }
            out.push_str(&text[last..]);
            out
        } else {
            let n = how.parse::<usize>().unwrap_or(1);
            let mut count = 0;
            let mut out = String::new();
            let mut last = 0;
            for m in re.find_iter(&text) {
                count += 1;
                if count == n {
                    out.push_str(&text[last..m.start()]);
                    out.push_str(&repl.replace("&", m.as_str()));
                    last = m.end();
                    break;
                }
            }
            out.push_str(&text[last..]);
            out
        };

        Value::Str(result)
    }

    fn call_match(&mut self, args: &[Expr]) -> Value {
        let s = self.eval_expr(&args[0]).as_str();
        let re_str = self.eval_expr(&args[1]).as_str();

        let re = regex::Regex::new(&re_str).unwrap_or_else(|_| regex::Regex::new("").unwrap());

        if let Some(m) = re.find(&s) {
            let pos = m.start() + 1;
            let len = m.end() - m.start();
            self.env.rstart = pos;
            self.env.rlength = len as isize;
            Value::Num(pos as f64)
        } else {
            self.env.rstart = 0;
            self.env.rlength = -1;
            Value::Num(0.0)
        }
    }

    fn call_sprintf(&mut self, args: &[Expr]) -> Value {
        let fmt = self.eval_expr(&args[0]).as_str();
        let mut result = String::new();
        let mut arg_idx = 1;
        let mut chars = fmt.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '%' {
                let mut width_str = String::new();
                let mut precision: Option<String> = None;
                let mut left_justify = false;
                let mut leading_zero = false;

                loop {
                    match chars.peek() {
                        Some('0') => {
                            leading_zero = true;
                            width_str.push(chars.next().unwrap());
                        }
                        Some('-') => {
                            left_justify = true;
                            chars.next();
                        }
                        Some('+') | Some(' ') => {
                            width_str.push(chars.next().unwrap());
                        }
                        _ => break,
                    }
                }
                while let Some(c) = chars.peek() {
                    if c.is_ascii_digit() {
                        width_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some('.') = chars.peek() {
                    chars.next();
                    let mut prec = String::new();
                    while let Some(c) = chars.peek() {
                        if c.is_ascii_digit() {
                            prec.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    precision = Some(prec);
                }

                let w = width_str.parse::<usize>().unwrap_or(0);

                match chars.next() {
                    Some('s') => {
                        if arg_idx < args.len() {
                            let s = self.eval_expr(&args[arg_idx]).as_str();
                            if left_justify {
                                result.push_str(&s);
                                if w > s.len() {
                                    result.push_str(&" ".repeat(w - s.len()));
                                }
                            } else {
                                if w > s.len() {
                                    result.push_str(&" ".repeat(w - s.len()));
                                }
                                result.push_str(&s);
                            }
                            arg_idx += 1;
                        }
                    }
                    Some('d') => {
                        if arg_idx < args.len() {
                            let v = self.eval_expr(&args[arg_idx]).as_f64() as i64;
                            let s = format!("{}", v);
                            if left_justify {
                                result.push_str(&s);
                                if w > s.len() {
                                    result.push_str(&" ".repeat(w - s.len()));
                                }
                            } else {
                                if w > s.len() {
                                    let pad = if leading_zero { "0" } else { " " };
                                    result.push_str(&pad.repeat(w - s.len()));
                                }
                                result.push_str(&s);
                            }
                            arg_idx += 1;
                        }
                    }
                    Some('f') => {
                        if arg_idx < args.len() {
                            let v = self.eval_expr(&args[arg_idx]).as_f64();
                            let s = if let Some(ref prec) = precision {
                                let p = prec.parse::<usize>().unwrap_or(6);
                                format!("{:.1$}", v, p)
                            } else {
                                format!("{}", v)
                            };
                            if left_justify {
                                result.push_str(&s);
                                if w > s.len() {
                                    result.push_str(&" ".repeat(w - s.len()));
                                }
                            } else {
                                if w > s.len() {
                                    result.push_str(&" ".repeat(w - s.len()));
                                }
                                result.push_str(&s);
                            }
                            arg_idx += 1;
                        }
                    }
                    Some('%') => result.push('%'),
                    Some(c) => {
                        result.push('%');
                        result.push(c);
                    }
                    None => result.push('%'),
                }
            } else {
                result.push(ch);
            }
        }

        Value::Str(result)
    }
}
