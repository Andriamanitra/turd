use std::iter::Peekable;

pub struct Parser {
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub enum ParseError {
    Invalid(String),
}

type Result<T> = std::result::Result<T, ParseError>;

impl Parser {
    fn new() -> Self {
        Parser {
            line: 1,
            col: 1,
        }
    }

    pub fn parse(code: &str) -> Result<Expr> {
        let mut it = code.chars().peekable();
        let mut parser = Self::new();
        let expr = parser.parse_expr(&mut it);
        match it.next() {
            Some(c) => Err(ParseError::Invalid(format!("Unexpected character {c:?} after expression"))),
            None => expr,
        }
    }

    fn next<It: Iterator<Item = char>>(&mut self, it: &mut Peekable<It>) -> Option<char> {
        let c = it.next();
        match c {
            Some('\n') => {
                self.line += 1;
                self.col = 1;
            }
            Some(_) => {
                self.col += 1;
            }
            _ => {}
        }
        c
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError::Invalid(format!("ERROR: {} on line {} column {}", msg, self.line, self.col))
    }

    fn parse_expr<It: Iterator<Item = char>>(&mut self, it: &mut Peekable<It>) -> Result<Expr> {
        self.skip_whitespace(it);
        match it.peek() {
            None => Ok(Expr::Noop),
            Some('(') => self.parse_list(it),
            Some('"') => self.parse_string_literal(it),
            Some(c) if c.is_ascii_alphabetic() => self.parse_identifier(it),
            Some(c) => Err(self.error(&format!("unexpected char {c:?} in expression")))
        }
    }

    fn parse_identifier<It: Iterator<Item = char>>(&mut self, it: &mut Peekable<It>) -> Result<Expr> {
        match self.next(it) {
            Some(c) if c.is_ascii_alphabetic() => {
                let mut ident = c.to_string();
                loop {
                    match it.peek() {
                        Some(c) if c.is_ascii_alphabetic() => {
                            let c = self.next(it).unwrap();
                            ident.push(c);
                        }
                        _ => return Ok(Expr::Identifier(ident)),
                    }
                }
            }
            Some(c) => Err(self.error(&format!("Identifier can't start with {c:?}"))),
            None => Err(self.error("no identifier"))
        }
    }

    fn parse_string_literal<It: Iterator<Item = char>>(&mut self, it: &mut Peekable<It>) -> Result<Expr> {
        match self.next(it) {
            Some('"') => {
                let mut string = String::new();
                loop {
                    match self.next(it) {
                        Some('"') => return Ok(Expr::StringLiteral(string)),
                        Some(c) => string.push(c),
                        None => return Err(self.error("Unterminated string literal")),
                    }
                }
            }
            _ => panic!("string literal should start with a double quote")
        }
    }
    
    fn parse_list<It: Iterator<Item = char>>(&mut self, it: &mut Peekable<It>) -> Result<Expr> {
        if let Some('(') = it.next() {
            self.col += 1;
            let mut contents = vec![];
            loop {
                self.skip_whitespace(it);
                match it.peek() {
                    Some(')') => {
                        self.next(it);
                        return Ok(Expr::List(contents))
                    },
                    None => return Err(self.error("Unterminated list")),
                    _ => {}
                }
                let inner_expr = self.parse_expr(it)?;
                contents.push(inner_expr);
            }
        } else {
            Err(ParseError::Invalid("list".into()))
        }
    }
    
    fn skip_whitespace<It: Iterator<Item = char>>(&mut self, it: &mut Peekable<It>) {
        while let Some(&c) = it.peek() {
            if c.is_ascii_whitespace() {
                self.next(it);
            } else {
                break
            }
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Noop,
    List(Vec<Expr>),
    Identifier(String),
    StringLiteral(String),
}
