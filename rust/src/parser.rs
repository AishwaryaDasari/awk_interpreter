use crate::ast::{BinOpKind, Expr, Node, Pattern, Rule, Stmt, UnaryOpKind};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    curr: Token,
    auto_match_regex: bool,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curr = lexer.next_token();
        Parser {
            lexer,
            curr,
            auto_match_regex: true,
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

    // ----- block parsing -----

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(&Token::LBrace)?;
        let mut stmts = Vec::new();
        loop {
            match &self.curr {
                Token::RBrace | Token::EOF => break,
                Token::Semicolon => {
                    self.advance();
                }
                Token::Print => {
                    stmts.push(self.parse_print()?);
                }
                _ => {
                    return Err(self.err_unexpected());
                }
            }
        }
        self.expect(&Token::RBrace)?;
        Ok(stmts)
    }

    // ----- print statement -----

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.advance();
        let mut exprs = Vec::new();
        loop {
            match &self.curr {
                Token::RBrace | Token::EOF | Token::Semicolon => break,
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
        self.parse_ternary()
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
        let mut expr = self.parse_compare()?;
        while self.curr == Token::And {
            self.advance();
            let rhs = self.parse_compare()?;
            expr = Expr::BinOp(Box::new(expr), BinOpKind::And, Box::new(rhs));
        }
        Ok(expr)
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
        let mut expr = self.parse_unary()?;
        while self.can_start_expr() {
            let rhs = self.parse_unary()?;
            expr = Expr::BinOp(Box::new(expr), BinOpKind::Concat, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.curr == Token::Not {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expr::UnaryOp(UnaryOpKind::Not, Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
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
                Ok(Expr::Ident(name))
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
}
