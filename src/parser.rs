use crate::lexer::Lexer;
use crate::module::*;
use crate::token::{Position, Token, TokenList, TokenType};
use crate::utils::ToSimpleString;

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

  fn current(&self) -> Option<&Token> {
    if self.index < self.token_list.len() {
      Some(&self.token_list[self.index])
    } else {
      None
    }
  }

  fn on_eof(&self) -> bool {
    self.index >= self.token_list.len()
  }

  fn peek(&mut self) -> Option<&Token> {
    if self.index + 1 < self.token_list.len() {
      Some(&self.token_list[self.index + 1])
    } else {
      None
    }
  }

  fn next(&mut self) {
    self.index += 1;
  }

  fn consume(&mut self, expect_type: TokenType) -> bool {
    if self.on_eof() {
      return false;
    }
    let next_token = self.current().unwrap();
    if next_token.ty == expect_type {
      self.next();
      true
    } else {
      false
    }
  }

  pub fn parse(&mut self) -> Box<Module> {
    let mut module = Box::new(Module::new());
    loop {
      match self.parse_stmt() {
        Some(stmt) => {
          println!("{}", stmt);
          module.add_stmt(stmt);
        }
        None => break,
      }
    }
    module
  }

  pub fn parse_stmt(&mut self) -> Option<Box<Stmt>> {
    let expr = self.parse_expr(Precedence::LOWEST)?;
    if !self.consume(TokenType::Semicolon) {
      match self.current() {
        Some(token) => panic!("Expected ';' but {} found...", token),
        None => panic!("Expected ';' but <EOF> found..."),
      };
    }
    if self.on_eof() {
      None
    } else {
      Some(Box::new(Stmt::ExprStmt { expr: expr }))
    }
  }

  fn parse_expr(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
    let mut lhs = self.parse_unary_op()?;
    while !self.on_eof() && precedence < self.current_precedence() {
      lhs = self.parse_binary_op(lhs)?;
    }
    Some(lhs)
  }

  fn parse_number(&mut self) -> Option<Box<Expr>> {
    let token = self.current()?;
    let pos = token.position;
    match token.ty {
      TokenType::Plus => self.make_unary_op(UnaryOpType::Plus),
      TokenType::Minus => self.make_unary_op(UnaryOpType::Minus),
      TokenType::Number(v) => {
        self.next();
        Some(Box::new(Expr::Number {
          value: v,
          position: pos,
        }))
      }
      _ => None,
    }
  }

  fn parse_unary_op(&mut self) -> Option<Box<Expr>> {
    let token = self.current()?;
    match token.ty {
      TokenType::Plus => self.make_unary_op(UnaryOpType::Plus),
      TokenType::Minus => self.make_unary_op(UnaryOpType::Minus),
      TokenType::Number(_) => self.parse_number(),
      TokenType::LParen => self.parse_grouped_expr(),
      _ => None,
    }
  }

  fn parse_binary_op(&mut self, lhs: Box<Expr>) -> Option<Box<Expr>> {
    let token = self.current()?;
    let op = match token.ty {
      TokenType::Plus => BinaryOpType::Add,
      TokenType::Minus => BinaryOpType::Sub,
      TokenType::Aster => BinaryOpType::Mul,
      TokenType::Slash => BinaryOpType::Div,
      _ => return Some(lhs),
    };
    let pos = token.position;
    let precedence = Self::token_precedence(token);
    self.next();

    let rhs = self.parse_expr(precedence)?;
    Some(Box::new(Expr::BinaryOp {
      op: op,
      lhs: lhs,
      rhs: rhs,
      position: pos,
    }))
  }

  fn parse_grouped_expr(&mut self) -> Option<Box<Expr>> {
    if !self.consume(TokenType::LParen) {
      panic!("'(' expected but {} found...", self.current()?);
    }
    let expr = self.parse_expr(Precedence::LOWEST);
    if self.consume(TokenType::RParen) {
      expr
    } else {
      panic!("')' expected but {} found...", self.current()?);
    }
  }

  fn make_unary_op(&mut self, op: UnaryOpType) -> Option<Box<Expr>> {
    let token = self.current()?;
    let pos = token.position;
    self.next();

    let rhs = self.parse_expr(Precedence::PREFIX)?;
    Some(Box::new(Expr::UnaryOp {
      op: op,
      rhs: rhs,
      position: pos,
    }))
  }

  fn current_precedence(&self) -> Precedence {
    Self::token_precedence(self.current().unwrap())
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

#[test]
fn test_parser() {
  test_parse(
    "1 + 2",
    r#"Some(BinaryOp { op: Add, lhs: Number(1), rhs: Number(2) })"#,
  );
  test_parse(
    "-5 + (4 - 20) * 4",
    "Some(BinaryOp { op: Add, lhs: UnaryOp { op: Minus, rhs: Number(5) }, rhs: BinaryOp { op: Mul, lhs: BinaryOp { op: Sub, lhs: Number(4), rhs: Number(20) }, rhs: Number(4) } })"
  );
}

#[cfg(test)]
fn test_parse(input: &str, expected: &str) {
  let mut lexer = Lexer::new(input.chars().collect());
  let token_list = lexer.tokenize();
  let mut parser = Parser::new(token_list);
  assert_eq!(format!("{:?}", parser.parse()), expected);
}
