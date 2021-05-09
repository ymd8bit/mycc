#[derive(Debug, Clone, Copy)]
pub enum TokenType {
  Number(u64), // [0-9][0-9]*
  Plus,        // '+'
  Minus,       // '-'
  Aster,       // '*'
  Slash,       // '/'
  LParen,      // '('
  RParen,      // ')'
  Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
  start: usize,
  end: usize,
}

impl Position {
  fn new(start: usize, end: usize) -> Self {
    Self {
      start: start,
      end: end,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
  pub ty: TokenType,
  position: Position,
}

impl Token {
  fn num(n: u64, start: usize, end: usize) -> Self {
    Self {
      ty: TokenType::Number(n),
      position: Position::new(start, end),
    }
  }

  fn eof(start: usize) -> Self {
    Self {
      ty: TokenType::Eof,
      position: Position::new(start, start),
    }
  }
}

pub struct Lexer {
  input: Vec<char>,
  position: usize,
}

fn is_number(c: char) -> bool {
  c.is_ascii_digit()
}

impl Lexer {
  pub fn new(input: Vec<char>) -> Lexer {
    Lexer {
      input: input,
      position: 0,
    }
  }

  fn current(&self) -> Option<&char> {
    self.input.get(self.position)
  }

  fn pos(&self) -> usize {
    self.position
  }

  fn peek(&self) -> Option<&char> {
    self.input.get(self.position + 1)
  }

  fn next(&mut self) {
    self.position += 1;
  }

  fn skip_space(&mut self) {
    while self.current().is_some() && self.current().unwrap().is_whitespace() {
      self.next();
    }
  }

  pub fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();

    while self.current().is_some() {
      self.skip_space();

      if self.current().is_none() {
        return tokens;
      }

      match self.make_token() {
        Some(tok) => tokens.push(tok),
        None => tokens.push(Token::eof(self.position)),
      }
    }

    tokens
  }

  fn make_token(&mut self) -> Option<Token> {
    let cur = self.current().unwrap();
    let pos = self.pos();

    if is_number(*cur) {
      Some(self.make_number())
    } else {
      match *cur {
        '+' => {
          self.next();
          Some(Token {
            ty: TokenType::Plus,
            position: Position::new(pos, pos + 1),
          })
        }
        '-' => {
          self.next();
          Some(Token {
            ty: TokenType::Minus,
            position: Position::new(pos, pos + 1),
          })
        }
        '*' => {
          self.next();
          Some(Token {
            ty: TokenType::Aster,
            position: Position::new(pos, pos + 1),
          })
        }
        '/' => {
          self.next();
          Some(Token {
            ty: TokenType::Slash,
            position: Position::new(pos, pos + 1),
          })
        }
        '(' => {
          self.next();
          Some(Token {
            ty: TokenType::LParen,
            position: Position::new(pos, pos + 1),
          })
        }
        ')' => {
          self.next();
          Some(Token {
            ty: TokenType::RParen,
            position: Position::new(pos, pos + 1),
          })
        }
        _ => panic!("Unknown char '{}' found...", *cur),
      }
    }
  }

  fn make_number(&mut self) -> Token {
    use std::iter::FromIterator;
    let cur = self.current().unwrap();
    let pos = self.pos();
    let mut num = vec![*cur];

    while let Some(next_char) = self.peek() {
      if is_number(*next_char) {
        num.push(*next_char);
        self.next();
      } else {
        break;
      }
    }
    match String::from_iter(num).parse::<u64>() {
      Ok(n) => {
        self.next();
        return Token::num(n, pos, self.position);
      }
      Err(_) => panic!("Invalid number found..."),
    }
  }
}
