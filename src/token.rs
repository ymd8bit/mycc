use std::fmt;

#[derive(Debug)]
pub enum TokenType {
  Number(u64), // [0-9][0-9]*
  Plus,        // '+'
  Minus,       // '-'
  Aster,       // '*'
  Slash,       // '/'
  LParen,      // '('
  RParen,      // ')'
  Assign,      // '='
  Not,         // '!'
  Eq,          // '=='
  Ne,          // '!='
  Lt,          // '<'
  Gt,          // '>'
  Le,          // '<='
  Ge,          // '>='
  Semicolon,
  Eof,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      TokenType::Number(x) => write!(f, "Num({})", x),
      TokenType::Plus => write!(f, "'+'"),
      TokenType::Minus => write!(f, "'-'"),
      TokenType::Aster => write!(f, "'*'"),
      TokenType::Slash => write!(f, "'/'"),
      TokenType::LParen => write!(f, "'('"),
      TokenType::RParen => write!(f, "')'"),
      TokenType::Assign => write!(f, "'='"),
      TokenType::Not => write!(f, "'!'"),
      TokenType::Eq => write!(f, "'=='"),
      TokenType::Ne => write!(f, "'!='"),
      TokenType::Lt => write!(f, "'<'"),
      TokenType::Le => write!(f, "'<='"),
      TokenType::Gt => write!(f, "'>'"),
      TokenType::Ge => write!(f, "'>='"),
      TokenType::Semicolon => write!(f, "';'"),
      TokenType::Eof => write!(f, "<EOF>"),
    }
  }
}

#[derive(Debug)]
pub struct Position {
  start: usize,
  end: usize,
}

impl Position {
  pub fn new(start: usize, end: usize) -> Self {
    Self {
      start: start,
      end: end,
    }
  }
}

#[derive(Debug)]
pub struct Token {
  pub ty: TokenType,
  pub position: Position,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let start = self.position.start;
    let end = self.position.end;
    write!(f, "Token({}, ({}, {}))", self.ty, start, end)
  }
}

impl Token {
  pub fn num(n: u64, start: usize, end: usize) -> Self {
    Self {
      ty: TokenType::Number(n),
      position: Position::new(start, end),
    }
  }

  pub fn eof(start: usize) -> Self {
    Self {
      ty: TokenType::Eof,
      position: Position::new(start, start),
    }
  }
}

pub struct TokenList {
  pub tokens: Vec<Token>,
}

impl fmt::Display for TokenList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TokenList[").expect("unable to write");
    for (i, token) in self.tokens.iter().enumerate() {
      if i != 0 {
        write!(f, ", ").expect("unable to write");
      }
      write!(f, "{}", token).expect("unable to write");
    }
    write!(f, "]")
  }
}

impl TokenList {
  pub fn new() -> Self {
    TokenList { tokens: Vec::new() }
  }

  pub fn push(&mut self, token: Token) {
    self.tokens.push(token);
  }
}
