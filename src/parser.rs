use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum UnaryOpType {
  Minus,
  Plus,
}

#[derive(Debug)]
pub enum BinaryOpType {
  Add,
  Sub,
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
  tokens: Vec<Token>,
  index: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens: tokens,
      index: 0,
    }
  }

  pub fn parse(&mut self) -> Option<Box<Expr>> {
    self.parse_expr(Precedence::LOWEST)
  }

  fn next(&mut self) -> Option<usize> {
    if self.tokens.len() > self.index + 1 {
      self.index += 1;
      Some(self.index)
    } else {
      None
    }
  }

  fn current(&mut self) -> Token {
    self.tokens[self.index]
  }

  fn peek(&mut self) -> Option<Token> {
    if self.tokens.len() > self.index + 1 {
      Some(self.tokens[self.index + 1])
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

  fn parse_unary_op(&mut self) -> Option<Box<Expr>> {
    match self.current().ty {
      TokenType::Plus => self.make_unary(UnaryOpType::Plus),
      TokenType::Minus => self.make_unary(UnaryOpType::Minus),
      TokenType::Number(_) => self.parse_number(),
      _ => None,
    }
  }

  fn parse_number(&mut self) -> Option<Box<Expr>> {
    match self.current().ty {
      TokenType::Number(v) => Some(Box::new(Expr::Number(v))),
      _ => None,
    }
  }

  fn parse_binary_op(&mut self, lhs: Box<Expr>) -> Option<Box<Expr>> {
    let op = match self.current().ty {
      TokenType::Plus => BinaryOpType::Add,
      TokenType::Minus => BinaryOpType::Sub,
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

  fn make_unary(&mut self, op: UnaryOpType) -> Option<Box<Expr>> {
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

  fn token_precedence(token: Token) -> Precedence {
    match token.ty {
      TokenType::Plus | TokenType::Minus => Precedence::SUM,
      _ => Precedence::LOWEST,
    }
  }
}
