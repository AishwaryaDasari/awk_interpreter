#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    Begin,
    End,
    BeginFile,
    EndFile,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Dollar,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Incr,
    Decr,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    Tilde,
    BangTilde,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Not,
    Question,
    Colon,
    String(String),
    Number(f64),
    Regex(String),
    Ident(String),
    Invalid(char),
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    prev_was_operand: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            prev_was_operand: false,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::EOF;
        }
        let ch = self.input[self.pos];
        let tok = match ch {
            '{' => {
                self.pos += 1;
                Token::LBrace
            }
            '}' => {
                self.pos += 1;
                Token::RBrace
            }
            '(' => {
                self.pos += 1;
                Token::LParen
            }
            ')' => {
                self.pos += 1;
                Token::RParen
            }
            '$' => {
                self.pos += 1;
                Token::Dollar
            }
            ',' => {
                self.pos += 1;
                Token::Comma
            }
            ';' => {
                self.pos += 1;
                Token::Semicolon
            }
            '?' => {
                self.pos += 1;
                Token::Question
            }
            ':' => {
                self.pos += 1;
                Token::Colon
            }
            '~' => {
                self.pos += 1;
                Token::Tilde
            }
            '+' => {
                self.pos += 1;
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        '+' => {
                            self.pos += 1;
                            Token::Incr
                        }
                        '=' => {
                            self.pos += 1;
                            Token::PlusAssign
                        }
                        _ => Token::Plus,
                    }
                } else {
                    Token::Plus
                }
            }
            '-' => {
                self.pos += 1;
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        '-' => {
                            self.pos += 1;
                            Token::Decr
                        }
                        '=' => {
                            self.pos += 1;
                            Token::MinusAssign
                        }
                        _ => Token::Minus,
                    }
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1;
                    Token::StarAssign
                } else {
                    Token::Star
                }
            }
            '/' => {
                if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '=' {
                    self.pos += 2;
                    Token::SlashAssign
                } else if !self.prev_was_operand {
                    self.read_regex()
                } else {
                    self.pos += 1;
                    Token::Slash
                }
            }
            '%' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1;
                    Token::PercentAssign
                } else {
                    Token::Percent
                }
            }
            '!' => {
                self.pos += 1;
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        '=' => {
                            self.pos += 1;
                            Token::Ne
                        }
                        '~' => {
                            self.pos += 1;
                            Token::BangTilde
                        }
                        _ => Token::Not,
                    }
                } else {
                    Token::Not
                }
            }
            '=' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1;
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '>' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1;
                    Token::Ge
                } else {
                    Token::Gt
                }
            }
            '<' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1;
                    Token::Le
                } else {
                    Token::Lt
                }
            }
            '&' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '&' {
                    self.pos += 1;
                    Token::And
                } else {
                    Token::Invalid('&')
                }
            }
            '|' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '|' {
                    self.pos += 1;
                    Token::Or
                } else {
                    Token::Invalid('|')
                }
            }
            '"' => self.read_string(),
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.read_ident(),
            _ => {
                self.pos += 1;
                Token::Invalid(ch)
            }
        };
        match &tok {
            Token::Number(_)
            | Token::String(_)
            | Token::Ident(_)
            | Token::RParen
            | Token::Regex(_)
            | Token::Incr
            | Token::Decr => self.prev_was_operand = true,
            _ => self.prev_was_operand = false,
        }
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn read_regex(&mut self) -> Token {
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '/' {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }
        if self.pos < self.input.len() {
            self.pos += 1;
        }
        Token::Regex(s)
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }
        if self.pos < self.input.len() {
            self.pos += 1;
        }
        Token::String(s)
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos < self.input.len() && self.input[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        let s: String = self.input[start..self.pos].iter().collect();
        let n: f64 = s.parse().unwrap_or(0.0);
        Token::Number(n)
    }

    fn read_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_')
        {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        match s.as_str() {
            "print" => Token::Print,
            "BEGIN" => Token::Begin,
            "END" => Token::End,
            "BEGINFILE" => Token::BeginFile,
            "ENDFILE" => Token::EndFile,
            _ => Token::Ident(s),
        }
    }
}
