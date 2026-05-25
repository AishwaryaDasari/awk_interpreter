#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Field(usize),
    FieldExpr(Box<Expr>),
    Ident(String),
    Str(String),
    Num(f64),
}
