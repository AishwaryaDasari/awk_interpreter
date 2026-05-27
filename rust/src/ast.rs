#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<Rule>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
    Begin(Vec<Stmt>),
    End(Vec<Stmt>),
    BeginFile(Vec<Stmt>),
    EndFile(Vec<Stmt>),
    Pattern(Pattern, Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Expr(Expr),
    Range(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Print(Vec<Expr>),
    Expr(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    DoWhile(Vec<Stmt>, Expr),
    For(Option<Expr>, Option<Expr>, Option<Expr>, Vec<Stmt>),
    ForIn(String, Expr, Vec<Stmt>),
    Break,
    Continue,
    Next,
    Exit(Option<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Match,
    NotMatch,
    Concat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Field(usize),
    FieldExpr(Box<Expr>),
    Ident(String),
    Str(String),
    Num(f64),
    BinOp(Box<Expr>, BinOpKind, Box<Expr>),
    UnaryOp(UnaryOpKind, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Assign(String, Box<Expr>),
    AssignOp(BinOpKind, String, Box<Expr>),
    Incr(String, bool),
    Decr(String, bool),
    Call(String, Vec<Expr>),
    ArrayAccess(String, Box<Expr>),
}
