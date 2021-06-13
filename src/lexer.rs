use crate::token::{Position, Token, TokenList, TokenType};

pub struct Lexer {
  input: Vec<char>,
  position: usize,
}

fn is_number(c: char) -> bool {
  c.is_ascii_digit()
}

fn is_alpha(c: char) -> bool {
  c.is_alphabetic()
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

  fn consume(&mut self, expect: &char) -> bool {
    match self.current() {
      Some(next_char) => {
        if next_char == expect {
          self.next();
          true
        } else {
          false
        }
      }
      None => false,
    }
  }

  fn skip_space(&mut self) {
    while self.current().is_some() && self.current().unwrap().is_whitespace() {
      self.next();
    }
  }

  pub fn tokenize(&mut self) -> TokenList {
    let mut tokens = TokenList::new();

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
    } else if is_alpha(*cur) {
      Some(self.make_id())
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
        '{' => {
          self.next();
          Some(Token {
            ty: TokenType::LBrace,
            position: Position::new(pos, pos + 1),
          })
        }
        '}' => {
          self.next();
          Some(Token {
            ty: TokenType::RBrace,
            position: Position::new(pos, pos + 1),
          })
        }
        '=' => {
          self.next();
          if self.consume(&'=') {
            Some(Token {
              ty: TokenType::Eq,
              position: Position::new(pos, pos + 2),
            })
          } else {
            Some(Token {
              ty: TokenType::Assign,
              position: Position::new(pos, pos + 1),
            })
          }
        }
        '!' => {
          self.next();
          if self.consume(&'=') {
            Some(Token {
              ty: TokenType::Ne,
              position: Position::new(pos, pos + 2),
            })
          } else {
            Some(Token {
              ty: TokenType::Not,
              position: Position::new(pos, pos + 1),
            })
          }
        }
        '<' => {
          self.next();
          if self.consume(&'=') {
            Some(Token {
              ty: TokenType::Le,
              position: Position::new(pos, pos + 2),
            })
          } else {
            Some(Token {
              ty: TokenType::Lt,
              position: Position::new(pos, pos + 1),
            })
          }
        }
        '>' => {
          self.next();
          if self.consume(&'=') {
            Some(Token {
              ty: TokenType::Ge,
              position: Position::new(pos, pos + 2),
            })
          } else {
            Some(Token {
              ty: TokenType::Gt,
              position: Position::new(pos, pos + 1),
            })
          }
        }
        ';' => {
          self.next();
          Some(Token {
            ty: TokenType::Semicolon,
            position: Position::new(pos, pos + 1),
          })
        }
        ',' => {
          self.next();
          Some(Token {
            ty: TokenType::Comma,
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

  fn make_id(&mut self) -> Token {
    use std::iter::FromIterator;
    let cur = self.current().unwrap();
    let pos = self.pos();
    let mut id = vec![*cur];

    while let Some(next_char) = self.peek() {
      if next_char.is_alphanumeric() {
        id.push(*next_char);
        self.next();
      } else {
        break;
      }
    }

    self.next();
    let pos_end = self.position;

    let id_str = String::from_iter(id);
    match self.make_reserved_word(&id_str[..], pos, pos_end) {
      Some(token) => token,
      None => Token::id(id_str, pos, pos_end),
    }
  }

  fn make_reserved_word(&mut self, id_str: &str, pos: usize, pos_end: usize) -> Option<Token> {
    match id_str {
      "if" => Some(Token {
        ty: TokenType::If,
        position: Position::new(pos, pos_end),
      }),
      "else" => Some(Token {
        ty: TokenType::Else,
        position: Position::new(pos, pos_end),
      }),
      "for" => Some(Token {
        ty: TokenType::For,
        position: Position::new(pos, pos_end),
      }),
      "while" => Some(Token {
        ty: TokenType::While,
        position: Position::new(pos, pos_end),
      }),
      "return" => Some(Token {
        ty: TokenType::Return,
        position: Position::new(pos, pos_end),
      }),
      _ => None,
    }
  }
}

#[test]
fn test_lexer() {
  test_tokenize("+", r#"TokenList[Token('+', @[0,1])]"#);
  test_tokenize("-", r#"TokenList[Token('-', @[0,1])]"#);
  test_tokenize("*", r#"TokenList[Token('*', @[0,1])]"#);
  test_tokenize("/", r#"TokenList[Token('/', @[0,1])]"#);
  test_tokenize(";", r#"TokenList[Token(';', @[0,1])]"#);
  test_tokenize("=", r#"TokenList[Token('=', @[0,1])]"#);
  test_tokenize("==", r#"TokenList[Token('==', @[0,2])]"#);
  test_tokenize("!", r#"TokenList[Token('!', @[0,1])]"#);
  test_tokenize("!=", r#"TokenList[Token('!=', @[0,2])]"#);
  test_tokenize(">", r#"TokenList[Token('>', @[0,1])]"#);
  test_tokenize(">=", r#"TokenList[Token('>=', @[0,2])]"#);
  test_tokenize("<", r#"TokenList[Token('<', @[0,1])]"#);
  test_tokenize("<=", r#"TokenList[Token('<=', @[0,2])]"#);

  test_tokenize(
    ">==>",
    r#"TokenList[Token('>=', @[0,2]), Token('=', @[2,3]), Token('>', @[3,4])]"#,
  );

  test_tokenize(
    "1 + 2",
    r#"TokenList[Token(Num(1), @[0,1]), Token('+', @[2,3]), Token(Num(2), @[4,5])]"#,
  );
  test_tokenize(
    "-5 + (4 - 20) * 4",
    r#"TokenList[Token('-', @[0,1]), Token(Num(5), @[1,2]), Token('+', @[3,4]), Token('(', @[5,6]), Token(Num(4), @[6,7]), Token('-', @[8,9]), Token(Num(20), @[10,12]), Token(')', @[12,13]), Token('*', @[14,15]), Token(Num(4), @[16,17])]"#,
  );
}

#[cfg(test)]
fn test_tokenize(input: &str, expected: &str) {
  let mut lexer = Lexer::new(input.chars().collect());
  assert_eq!(format!("{}", lexer.tokenize()), expected);
}
