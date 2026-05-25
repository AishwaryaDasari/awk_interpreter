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

    pub fn parse(&mut self) -> Node {
        let mut stmts = Vec::new();

        if self.curr == Token::LBrace {
            self.advance();
        }

        loop {
            match &self.curr {
                Token::RBrace | Token::EOF => {
                    break;
                }
                Token::Print => {
                    stmts.push(self.parse_print());
                }
                _ => {
                    self.advance();
                }
            }
        }

        Node::Program(stmts)
    }

    fn parse_print(&mut self) -> Stmt {
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
                    exprs.push(self.parse_expr());
                    if self.curr == Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        Stmt::Print(exprs)
    }

    fn parse_expr(&mut self) -> Expr {
        match &self.curr.clone() {
            Token::Dollar => {
                self.advance();
                let inner = self.parse_expr();
                Expr::FieldExpr(Box::new(inner))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Expr::Str(val)
            }
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Expr::Num(val)
            }
            Token::Ident(s) => {
                let name = s.clone();
                self.advance();
                Expr::Ident(name)
            }
            _ => {
                self.advance();
                Expr::Ident(String::new())
            }
        }
    }
}
