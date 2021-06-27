use crate::utils::ToSimpleString;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
pub enum TokenType {
  Id(String),  // ([a-z|A-Z|_])([a-z|A-Z|_|0-9])*
  Number(u64), // [0-9][0-9]*
  If,          // 'if'
  Else,        // 'else'
  For,         // 'for'
  While,       // 'while'
  Return,      // 'return'
  Plus,        // '+'
  Minus,       // '-'
  Aster,       // '*'
  Slash,       // '/'
  LParen,      // '('
  RParen,      // ')'
  LBrace,      // '{'
  RBrace,      // '}'
  Assign,      // '='
  Inc,         // '+='
  Dec,         // '-='
  Not,         // '!'
  Eq,          // '=='
  Ne,          // '!='
  Lt,          // '<'
  Gt,          // '>'
  Le,          // '<='
  Ge,          // '>='
  Semicolon,   // ';'
  Comma,       // ','
  Eof,
}

impl ToSimpleString for TokenType {
  fn to_simple_string(&self) -> String {
    match self {
      TokenType::Id(x) => format!("Id({})", x),
      TokenType::Number(x) => format!("Num({})", x),
      TokenType::If => format!("If"),
      TokenType::Else => format!("Else"),
      TokenType::For => format!("For"),
      TokenType::While => format!("While"),
      TokenType::Return => format!("Return"),
      TokenType::Plus => String::from("'+'"),
      TokenType::Minus => String::from("'-'"),
      TokenType::Aster => String::from("'*'"),
      TokenType::Slash => String::from("'/'"),
      TokenType::LParen => String::from("'('"),
      TokenType::RParen => String::from("')'"),
      TokenType::LBrace => String::from("'{'"),
      TokenType::RBrace => String::from("'}'"),
      TokenType::Assign => String::from("'='"),
      TokenType::Inc => String::from("'+='"),
      TokenType::Dec => String::from("'-='"),
      TokenType::Not => String::from("'!'"),
      TokenType::Eq => String::from("'=='"),
      TokenType::Ne => String::from("'!='"),
      TokenType::Lt => String::from("'<'"),
      TokenType::Le => String::from("'<='"),
      TokenType::Gt => String::from("'>'"),
      TokenType::Ge => String::from("'>='"),
      TokenType::Semicolon => String::from("';'"),
      TokenType::Comma => String::from("','"),
      TokenType::Eof => String::from("<EOF>"),
    }
  }
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

// impl fmt::Display for TokenType {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}", self.to_pretty_string())
//   }
// }

#[derive(Debug, Clone, Copy)]
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

impl ToSimpleString for Position {
  fn to_simple_string(&self) -> String {
    format!("@[{},{}]", self.start, self.end)
  }
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

#[derive(Debug)]
pub struct Token {
  pub ty: TokenType,
  pub position: Position,
}

impl ToSimpleString for Token {
  fn to_simple_string(&self) -> String {
    format!("Token({}, {})", self.ty, self.position)
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

impl Token {
  pub fn num(n: u64, start: usize, end: usize) -> Self {
    Self {
      ty: TokenType::Number(n),
      position: Position::new(start, end),
    }
  }

  pub fn id(s: String, start: usize, end: usize) -> Self {
    Self {
      ty: TokenType::Id(s),
      position: Position::new(start, end),
    }
  }

  pub fn eof(start: usize) -> Self {
    Self {
      ty: TokenType::Eof,
      position: Position::new(start, start),
    }
  }

  pub fn get_id_string(&self) -> String {
    match &self.ty {
      TokenType::Id(name) => name.clone(),
      _ => panic!("Expect ID token but {} found...", self),
    }
  }
}

pub struct TokenList {
  pub tokens: Vec<Token>,
}

impl TokenList {
  pub fn new() -> Self {
    TokenList { tokens: Vec::new() }
  }

  pub fn len(&self) -> usize {
    self.tokens.len()
  }

  pub fn push(&mut self, token: Token) {
    self.tokens.push(token);
  }
}

impl ToSimpleString for TokenList {
  fn to_simple_string(&self) -> String {
    let mut s = String::from("TokenList[");
    for (i, token) in self.tokens.iter().enumerate() {
      if i != 0 {
        s.push_str(", ");
      }
      s.push_str(&format!("{}", token));
    }
    s.push_str("]");
    s
  }
}

impl fmt::Display for TokenList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

impl Index<usize> for TokenList {
  type Output = Token;
  fn index<'a>(&'a self, i: usize) -> &'a Token {
    &self.tokens[i]
  }
}

impl IndexMut<usize> for TokenList {
  fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Token {
    &mut self.tokens[i]
  }
}
