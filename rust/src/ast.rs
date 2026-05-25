#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<Rule>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
    Begin(Vec<Stmt>),
    End(Vec<Stmt>),
    Pattern(Pattern, Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Regex(String),
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
