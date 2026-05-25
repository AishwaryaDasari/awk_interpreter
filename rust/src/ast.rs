#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Print(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Field(usize),
    FieldExpr(Box<Expr>),
    Ident(String),
    Str(String),
    Num(f64),
}
