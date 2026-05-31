use crate::ast::{BinOpKind, Expr, Node, Pattern, Rule, Stmt, UnaryOpKind};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    curr: Token,
    auto_match_regex: bool,
    allow_in_expr: bool,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curr = lexer.next_token();
        Parser {
            lexer,
            curr,
            auto_match_regex: true,
            allow_in_expr: true,
        }
    }

    fn can_start_expr(&self) -> bool {
        matches!(
            self.curr,
            Token::String(_)
                | Token::Number(_)
                | Token::LParen
                | Token::Dollar
                | Token::Ident(_)
                | Token::Regex(_)
                | Token::Not
                | Token::Plus
                | Token::Minus
                | Token::Incr
                | Token::Decr
        )
    }

    fn advance(&mut self) {
        self.curr = self.lexer.next_token();
    }

    fn err_unexpected(&self) -> String {
        format!("unexpected token: {:?}", self.curr)
    }

    fn expect(&mut self, tok: &Token) -> Result<(), String> {
        if self.curr == *tok {
            self.advance();
            Ok(())
        } else {
            Err(self.err_unexpected())
        }
    }

    // ----- top-level parse -----

    pub fn parse(&mut self) -> Result<Node, String> {
        let mut rules = Vec::new();
        loop {
            match &self.curr {
                Token::EOF => break,
                Token::Newline => {
                    self.advance();
                }
                Token::Begin => {
                    self.advance();
                    rules.push(Rule::Begin(self.parse_block()?));
                }
                Token::End => {
                    self.advance();
                    rules.push(Rule::End(self.parse_block()?));
                }
                Token::BeginFile => {
                    self.advance();
                    rules.push(Rule::BeginFile(self.parse_block()?));
                }
                Token::EndFile => {
                    self.advance();
                    rules.push(Rule::EndFile(self.parse_block()?));
                }
                Token::LBrace => {
                    rules.push(Rule::Pattern(
                        Pattern::Expr(Expr::Num(1.0)),
                        self.parse_block()?,
                    ));
                }
                _ => {
                    let start = self.parse_expr()?;
                    if self.curr == Token::Comma {
                        self.advance();
                        let end = self.parse_expr()?;
                        rules.push(Rule::Pattern(
                            Pattern::Range(Box::new(start), Box::new(end)),
                            self.parse_block()?,
                        ));
                    } else {
                        rules.push(Rule::Pattern(Pattern::Expr(start), self.parse_block()?));
                    }
                }
            }
        }
        Ok(Node::Program(rules))
    }

    // ----- block & statement parsing -----

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(&Token::LBrace)?;
        let mut stmts = Vec::new();
        loop {
            match &self.curr {
                Token::RBrace | Token::EOF => break,
                Token::Semicolon | Token::Newline => {
                    self.advance();
                }
                _ => {
                    stmts.push(self.parse_one_stmt()?);
                }
            }
        }
        self.expect(&Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_one_stmt(&mut self) -> Result<Stmt, String> {
        match &self.curr {
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while_stmt(),
            Token::Do => self.parse_do_while_stmt(),
            Token::For => self.parse_for_stmt(),
            Token::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            Token::Next => {
                self.advance();
                Ok(Stmt::Next)
            }
            Token::Exit => self.parse_exit_stmt(),
            Token::Delete => self.parse_delete_stmt(),
            _ => {
                if self.can_start_expr() {
                    Ok(Stmt::Expr(self.parse_expr()?))
                } else {
                    Err(self.err_unexpected())
                }
            }
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(&Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        while self.curr == Token::Newline {
            self.advance();
        }
        let then_body = self.parse_branch()?;
        while self.curr == Token::Semicolon || self.curr == Token::Newline {
            self.advance();
        }
        let mut else_body = Vec::new();
        if self.curr == Token::Else {
            self.advance();
            else_body = self.parse_branch()?;
        }
        Ok(Stmt::If(cond, then_body, else_body))
    }

    fn parse_branch(&mut self) -> Result<Vec<Stmt>, String> {
        if self.curr == Token::LBrace {
            self.parse_block()
        } else if self.curr == Token::Newline {
            self.advance();
            self.parse_branch()
        } else {
            let stmt = self.parse_one_stmt()?;
            Ok(vec![stmt])
        }
    }

    // ----- loop statements -----

    fn parse_while_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(&Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        let body = self.parse_branch()?;
        Ok(Stmt::While(cond, body))
    }

    fn parse_do_while_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();
        let body = self.parse_branch()?;
        while self.curr == Token::Newline || self.curr == Token::Semicolon {
            self.advance();
        }
        self.expect(&Token::While)?;
        self.expect(&Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        Ok(Stmt::DoWhile(body, cond))
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(&Token::LParen)?;

        let saved = self.allow_in_expr;
        self.allow_in_expr = false;
        let init = if self.curr != Token::Semicolon {
            let expr = self.parse_expr()?;
            if self.curr == Token::In {
                self.advance();
                let array = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                let body = self.parse_branch()?;
                self.allow_in_expr = saved;
                match expr {
                    Expr::Ident(var) => return Ok(Stmt::ForIn(var, array, body)),
                    _ => return Err("for-in requires a variable name".to_string()),
                }
            }
            Some(expr)
        } else {
            None
        };
        self.allow_in_expr = saved;
        self.expect(&Token::Semicolon)?;
        let cond = if self.curr != Token::Semicolon && self.curr != Token::RParen {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(&Token::Semicolon)?;
        let incr = if self.curr != Token::RParen {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(&Token::RParen)?;
        let body = self.parse_branch()?;
        Ok(Stmt::For(init, cond, incr, body))
    }

    fn parse_exit_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();
        let expr = if self.can_start_expr() {
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Stmt::Exit(expr))
    }

    // ----- print statement -----

    fn parse_delete_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();
        match &self.curr {
            Token::Ident(name) => {
                let arr_name = name.clone();
                self.advance();
                if self.curr == Token::LBracket {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(&Token::RBracket)?;
                    Ok(Stmt::Delete(arr_name, Some(Box::new(index))))
                } else {
                    Ok(Stmt::Delete(arr_name, None))
                }
            }
            _ => Err("expected array name after delete".to_string()),
        }
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.advance();
        let mut exprs = Vec::new();
        loop {
            match &self.curr {
                Token::RBrace | Token::EOF | Token::Semicolon | Token::Newline => break,
                Token::Comma => {
                    self.advance();
                }
                _ => {
                    exprs.push(self.parse_expr()?);
                    if self.curr == Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(Stmt::Print(exprs))
    }

    // ----- expression parsing with precedence -----

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_ternary()?;
        match &self.curr {
            Token::Assign => {
                self.advance();
                match expr {
                    Expr::Ident(name) => Ok(Expr::Assign(name, Box::new(self.parse_assignment()?))),
                    Expr::ArrayAccess(name, index) => Ok(Expr::ArrayAssign(
                        name,
                        index,
                        Box::new(self.parse_assignment()?),
                    )),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            Token::PlusAssign
            | Token::MinusAssign
            | Token::StarAssign
            | Token::SlashAssign
            | Token::PercentAssign => {
                let kind = match &self.curr {
                    Token::PlusAssign => BinOpKind::Add,
                    Token::MinusAssign => BinOpKind::Sub,
                    Token::StarAssign => BinOpKind::Mul,
                    Token::SlashAssign => BinOpKind::Div,
                    Token::PercentAssign => BinOpKind::Mod,
                    _ => unreachable!(),
                };
                self.advance();
                match expr {
                    Expr::Ident(name) => Ok(Expr::AssignOp(
                        kind,
                        name,
                        Box::new(self.parse_assignment()?),
                    )),
                    Expr::ArrayAccess(name, index) => Ok(Expr::ArrayAssignOp(
                        kind,
                        name,
                        index,
                        Box::new(self.parse_assignment()?),
                    )),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            _ => Ok(expr),
        }
    }

    fn parse_ternary(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_or()?;
        if self.curr == Token::Question {
            self.advance();
            let true_branch = self.parse_expr()?;
            self.expect(&Token::Colon)?;
            let false_branch = self.parse_expr()?;
            expr = Expr::Ternary(
                Box::new(expr),
                Box::new(true_branch),
                Box::new(false_branch),
            );
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and()?;
        while self.curr == Token::Or {
            self.advance();
            let rhs = self.parse_and()?;
            expr = Expr::BinOp(Box::new(expr), BinOpKind::Or, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_membership()?;
        while self.curr == Token::And {
            self.advance();
            let rhs = self.parse_membership()?;
            expr = Expr::BinOp(Box::new(expr), BinOpKind::And, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_membership(&mut self) -> Result<Expr, String> {
        let expr = self.parse_compare()?;
        if self.allow_in_expr && self.curr == Token::In {
            self.advance();
            match &self.curr {
                Token::Ident(name) => {
                    let arr_name = name.clone();
                    self.advance();
                    Ok(Expr::In(Box::new(expr), arr_name))
                }
                _ => Err("expected array name after 'in'".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_compare(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_match()?;
        let kind = match &self.curr {
            Token::Eq => BinOpKind::Eq,
            Token::Ne => BinOpKind::Ne,
            Token::Lt => BinOpKind::Lt,
            Token::Gt => BinOpKind::Gt,
            Token::Le => BinOpKind::Le,
            Token::Ge => BinOpKind::Ge,
            _ => return Ok(expr),
        };
        self.advance();
        let rhs = self.parse_match()?;
        expr = Expr::BinOp(Box::new(expr), kind, Box::new(rhs));
        Ok(expr)
    }

    fn parse_match(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_concat()?;
        while self.curr == Token::Tilde || self.curr == Token::BangTilde {
            let kind = if self.curr == Token::Tilde {
                BinOpKind::Match
            } else {
                BinOpKind::NotMatch
            };
            self.advance();
            let saved = self.auto_match_regex;
            self.auto_match_regex = false;
            let rhs = self.parse_concat()?;
            self.auto_match_regex = saved;
            expr = Expr::BinOp(Box::new(expr), kind, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_concat(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_arith()?;
        while self.can_start_expr() {
            let rhs = self.parse_arith()?;
            expr = Expr::BinOp(Box::new(expr), BinOpKind::Concat, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_arith(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;
        while self.curr == Token::Plus || self.curr == Token::Minus {
            let kind = if self.curr == Token::Plus {
                BinOpKind::Add
            } else {
                BinOpKind::Sub
            };
            self.advance();
            let rhs = self.parse_term()?;
            expr = Expr::BinOp(Box::new(expr), kind, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        while self.curr == Token::Star || self.curr == Token::Slash || self.curr == Token::Percent {
            let kind = if self.curr == Token::Star {
                BinOpKind::Mul
            } else if self.curr == Token::Slash {
                BinOpKind::Div
            } else {
                BinOpKind::Mod
            };
            self.advance();
            let rhs = self.parse_unary()?;
            expr = Expr::BinOp(Box::new(expr), kind, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match &self.curr {
            Token::Not => {
                self.advance();
                Ok(Expr::UnaryOp(
                    UnaryOpKind::Not,
                    Box::new(self.parse_unary()?),
                ))
            }
            Token::Minus => {
                self.advance();
                Ok(Expr::UnaryOp(
                    UnaryOpKind::Neg,
                    Box::new(self.parse_unary()?),
                ))
            }
            Token::Plus => {
                self.advance();
                self.parse_unary()
            }
            Token::Incr => {
                self.advance();
                let expr = self.parse_unary()?;
                match expr {
                    Expr::Ident(name) => Ok(Expr::Incr(name, false)),
                    Expr::ArrayAccess(name, index) => Ok(Expr::ArrayIncr(name, index, false)),
                    _ => Err("invalid increment target".to_string()),
                }
            }
            Token::Decr => {
                self.advance();
                let expr = self.parse_unary()?;
                match expr {
                    Expr::Ident(name) => Ok(Expr::Decr(name, false)),
                    Expr::ArrayAccess(name, index) => Ok(Expr::ArrayDecr(name, index, false)),
                    _ => Err("invalid decrement target".to_string()),
                }
            }
            _ => {
                let expr = self.parse_primary()?;
                match &self.curr {
                    Token::Incr => {
                        self.advance();
                        match expr {
                            Expr::Ident(name) => Ok(Expr::Incr(name, true)),
                            Expr::ArrayAccess(name, index) => {
                                Ok(Expr::ArrayIncr(name, index, true))
                            }
                            _ => Err("invalid increment target".to_string()),
                        }
                    }
                    Token::Decr => {
                        self.advance();
                        match expr {
                            Expr::Ident(name) => Ok(Expr::Decr(name, true)),
                            Expr::ArrayAccess(name, index) => {
                                Ok(Expr::ArrayDecr(name, index, true))
                            }
                            _ => Err("invalid decrement target".to_string()),
                        }
                    }
                    _ => Ok(expr),
                }
            }
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>, String> {
        self.advance();
        let mut args = Vec::new();
        if self.curr != Token::RParen {
            let saved = self.auto_match_regex;
            self.auto_match_regex = false;
            args.push(self.parse_expr()?);
            while self.curr == Token::Comma {
                self.advance();
                args.push(self.parse_expr()?);
            }
            self.auto_match_regex = saved;
        }
        self.expect(&Token::RParen)?;
        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let expr = self.parse_primary_atom()?;
        self.parse_bracket_suffix(expr)
    }

    fn parse_primary_atom(&mut self) -> Result<Expr, String> {
        match &self.curr.clone() {
            Token::Dollar => {
                self.advance();
                let inner = self.parse_primary()?;
                Ok(Expr::FieldExpr(Box::new(inner)))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::Str(val))
            }
            Token::Regex(re) => {
                let val = re.clone();
                self.advance();
                if self.auto_match_regex {
                    Ok(Expr::BinOp(
                        Box::new(Expr::Field(0)),
                        BinOpKind::Match,
                        Box::new(Expr::Str(val)),
                    ))
                } else {
                    Ok(Expr::Str(val))
                }
            }
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Num(val))
            }
            Token::Ident(s) => {
                let name = s.clone();
                self.advance();
                if self.curr == Token::LParen {
                    Ok(Expr::Call(name, self.parse_call_args()?))
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Token::Invalid(c) => {
                let ch = *c;
                self.advance();
                Err(format!("invalid character: '{}'", ch))
            }
            _ => Err(self.err_unexpected()),
        }
    }

    fn parse_bracket_suffix(&mut self, mut expr: Expr) -> Result<Expr, String> {
        if self.curr == Token::LBracket {
            let name = match expr {
                Expr::Ident(n) => n,
                _ => return Err("array access requires a variable name".to_string()),
            };
            self.advance();
            let index = self.parse_expr()?;
            self.expect(&Token::RBracket)?;
            expr = Expr::ArrayAccess(name, Box::new(index));
        }
        Ok(expr)
    }
}
