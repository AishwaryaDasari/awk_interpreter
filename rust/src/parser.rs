use crate::ast::{Expr, Node, Stmt};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    curr: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curr = lexer.next_token();
        Parser { lexer, curr }
    }

    fn advance(&mut self) {
        self.curr = self.lexer.next_token();
    }

    fn err_unexpected(&self) -> String {
        format!("unexpected token: {:?}", self.curr)
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let mut stmts = Vec::new();

        if self.curr == Token::LBrace {
            self.advance();
        } else {
            return Err(self.err_unexpected());
        }

        loop {
            match &self.curr {
                Token::RBrace | Token::EOF => {
                    break;
                }
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

        if self.curr != Token::RBrace {
            return Err(self.err_unexpected());
        }
        self.advance();

        Ok(Node::Program(stmts))
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.advance();
        let mut exprs = Vec::new();

        if self.curr == Token::Comma {
            self.advance();
        }

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

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match &self.curr.clone() {
            Token::Dollar => {
                self.advance();
                let inner = self.parse_expr()?;
                Ok(Expr::FieldExpr(Box::new(inner)))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::Str(val))
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
            Token::Invalid(c) => {
                let ch = *c;
                self.advance();
                Err(format!("invalid character: '{}'", ch))
            }
            _ => Err(self.err_unexpected()),
        }
    }
}
