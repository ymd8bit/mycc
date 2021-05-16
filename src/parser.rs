use crate::token::{Token, TokenList, TokenType};

#[derive(Debug)]
pub enum UnaryOpType {
  Minus,
  Plus,
}

#[derive(Debug)]
pub enum BinaryOpType {
  Add,
  Sub,
  Mul,
  Div,
  // Assign,
  // Lt,
  // Ne,
  // Gt,
  // Le,
  // Ge,
}

#[derive(Debug)]
pub enum Expr {
  Number(u64),
  UnaryOp {
    op: UnaryOpType,
    rhs: Box<Expr>,
  },
  BinaryOp {
    op: BinaryOpType,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
  LOWEST = 0x0,
  SUM = 0x1,
  PRODUCT = 0x2,
  PREFIX = 0x3,
}

pub struct Parser {
  token_list: TokenList,
  index: usize,
}

impl Parser {
  pub fn new(token_list: TokenList) -> Parser {
    Parser {
      token_list: token_list,
      index: 0,
    }
  }

  pub fn parse(&mut self) -> Option<Box<Expr>> {
    self.parse_expr(Precedence::LOWEST)
  }

  fn next(&mut self) -> Option<usize> {
    if self.token_list.tokens.len() > self.index + 1 {
      self.index += 1;
      Some(self.index)
    } else {
      None
    }
  }

  fn current(&mut self) -> &Token {
    &self.token_list.tokens[self.index]
  }

  fn peek(&mut self) -> Option<&Token> {
    if self.token_list.tokens.len() > self.index + 1 {
      Some(&self.token_list.tokens[self.index + 1])
    } else {
      None
    }
  }

  fn parse_expr(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
    let mut lhs = self.parse_unary_op()?;
    while self.peek().is_some() && precedence < self.peek_precedence() {
      self.next()?;
      lhs = self.parse_binary_op(lhs)?;
    }
    Some(lhs)
  }

  fn parse_number(&mut self) -> Option<Box<Expr>> {
    match self.current().ty {
      TokenType::Plus => self.make_unary_op(UnaryOpType::Plus),
      TokenType::Minus => self.make_unary_op(UnaryOpType::Minus),
      TokenType::Number(v) => Some(Box::new(Expr::Number(v))),
      _ => None,
    }
  }

  fn parse_unary_op(&mut self) -> Option<Box<Expr>> {
    match self.current().ty {
      TokenType::Plus => self.make_unary_op(UnaryOpType::Plus),
      TokenType::Minus => self.make_unary_op(UnaryOpType::Minus),
      TokenType::Number(_) => self.parse_number(),
      TokenType::LParen => self.parse_grouped_expr(),
      _ => None,
    }
  }

  fn parse_binary_op(&mut self, lhs: Box<Expr>) -> Option<Box<Expr>> {
    let op = match self.current().ty {
      TokenType::Plus => BinaryOpType::Add,
      TokenType::Minus => BinaryOpType::Sub,
      TokenType::Aster => BinaryOpType::Mul,
      TokenType::Slash => BinaryOpType::Div,
      _ => return Some(lhs),
    };
    let precedence = Self::token_precedence(self.current());
    self.next()?;
    let rhs = self.parse_expr(precedence)?;
    Some(Box::new(Expr::BinaryOp {
      op: op,
      lhs: lhs,
      rhs: rhs,
    }))
  }

  fn parse_grouped_expr(&mut self) -> Option<Box<Expr>> {
    self.next()?; // consume '('
    let expr = self.parse_expr(Precedence::LOWEST);
    if self.peek().is_none() {
      panic!("Expected ')' not found..."); // the case of "(<EOF>"
    }
    let next_token = self.peek().unwrap();
    match next_token.ty {
      TokenType::RParen => {
        self.next()?;
        expr
      }
      _ => panic!("Expected ')' not found..."),
    }
  }

  fn make_unary_op(&mut self, op: UnaryOpType) -> Option<Box<Expr>> {
    self.next()?;
    let rhs = self.parse_expr(Precedence::PREFIX)?;
    Some(Box::new(Expr::UnaryOp { op: op, rhs: rhs }))
  }

  fn peek_precedence(&mut self) -> Precedence {
    match self.peek() {
      Some(token) => Self::token_precedence(token),
      None => Precedence::LOWEST,
    }
  }

  fn token_precedence(token: &Token) -> Precedence {
    match token.ty {
      TokenType::Plus | TokenType::Minus => Precedence::SUM,
      TokenType::Aster | TokenType::Slash => Precedence::PRODUCT,
      _ => Precedence::LOWEST,
    }
  }
}

// #[test]
// fn test_parser() {
//   test_parse(
//     "1 + 2",
//     r#"Some(BinaryOp { op: Add, lhs: Number(1), rhs: Number(2) })"#,
//   );
//   test_parse(
//     "-5 + (4 - 20) * 4",
//     "Some(BinaryOp { op: Add, lhs: UnaryOp { op: Minus, rhs: Number(5) }, rhs: BinaryOp { op: Mul, lhs: BinaryOp { op: Sub, lhs: Number(4), rhs: Number(20) }, rhs: Number(4) } })"
//   );
// }

// #[cfg(test)]
// fn test_parse(input: &str, expected: &str) {
//   let mut lexer = Lexer::new(input.chars().collect());
//   let token_list = lexer.tokenize();
//   let mut parser = Parser::new(tokens);
//   assert_eq!(format!("{:?}", parser.parse()), expected);
// }
