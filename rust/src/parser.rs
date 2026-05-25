use crate::ast::{Expr, Node, Pattern, Rule, Stmt};
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
        let mut rules = Vec::new();

        loop {
            match &self.curr {
                Token::EOF => break,
                Token::Begin => {
                    self.advance();
                    let stmts = self.parse_block()?;
                    rules.push(Rule::Begin(stmts));
                }
                Token::End => {
                    self.advance();
                    let stmts = self.parse_block()?;
                    rules.push(Rule::End(stmts));
                }
                Token::Regex(re) => {
                    let pattern = Pattern::Regex(re.clone());
                    self.advance();
                    let stmts = self.parse_block()?;
                    rules.push(Rule::Pattern(pattern, stmts));
                }
                Token::LBrace => {
                    let stmts = self.parse_block()?;
                    rules.push(Rule::Pattern(Pattern::Regex("".to_string()), stmts));
                }
                _ => return Err(self.err_unexpected()),
            }
        }

        Ok(Node::Program(rules))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        if self.curr == Token::LBrace {
            self.advance();
        } else {
            return Err(self.err_unexpected());
        }

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
                _ => return Err(self.err_unexpected()),
            }
        }

        if self.curr != Token::RBrace {
            return Err(self.err_unexpected());
        }
        self.advance();

        Ok(stmts)
    }

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
