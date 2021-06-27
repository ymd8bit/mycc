use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Position, Token, TokenList, TokenType};

#[derive(PartialOrd, PartialEq)]
enum Precedence {
  LOWEST = 0x0,
  ASSIGN = 0x1,
  COMPARE = 0x2,
  SUM = 0x3,
  PRODUCT = 0x4,
  PREFIX = 0x5,
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

  fn current_or_panic(&self) -> &Token {
    if self.index < self.token_list.len() {
      &self.token_list[self.index]
    } else {
      panic!("Unexpectedly reached to <EOF>...");
    }
  }

  fn peek(&self) -> Option<&Token> {
    if self.index < self.token_list.len() {
      Some(&self.token_list[self.index + 1])
    } else {
      None
    }
  }

  fn peek_is(&self, expect_type: TokenType) -> bool {
    if self.peek().is_none() {
      return false;
    }
    let peek = self.peek().unwrap();
    peek.ty == expect_type
  }

  fn on_eof(&self) -> bool {
    self.index >= self.token_list.len()
  }

  fn next(&mut self) {
    self.index += 1;
  }

  fn consume(&mut self, expect_type: TokenType) -> Option<&Token> {
    if self.on_eof() {
      return None;
    }
    let current_token = &self.token_list[self.index];
    if current_token.ty == expect_type {
      self.index += 1;
      Some(current_token)
    } else {
      None
    }
  }

  fn consume_or_panic(&mut self, expect_type: TokenType) -> &Token {
    if self.on_eof() {
      panic!("Unexpectly reached to <EOF>...");
    }
    let current_token = &self.token_list[self.index];
    if current_token.ty == expect_type {
      self.index += 1;
      current_token
    } else {
      panic!("Expect {} but {} found...", expect_type, current_token.ty);
    }
  }

  pub fn parse(&mut self) -> Box<Module> {
    let mut module = Box::new(Module::new());
    while !self.on_eof() {
      match self.parse_decl() {
        Some(stmt) => {
          module.add_stmt(stmt);
        }
        None => break,
      }
    }
    module
  }

  pub fn parse_decl(&mut self) -> Option<Box<Stmt>> {
    let token = self.current()?;
    println!("{}", token);
    match token.ty {
      TokenType::Id(_) => {
        if self.peek_is(TokenType::LParen) {
          return Some(self.parse_fn());
        } else {
          panic!("var decl is not supported now...");
        }
      }
      _ => panic!("Expected decl but {} found...", token.ty),
    }
  }

  pub fn parse_fn(&mut self) -> Box<Stmt> {
    let (name, args) = self.parse_prototype();
    let block = self.parse_stmt_block();

    Box::new(Stmt::FnStmt {
      name: name,
      args: args,
      body: block,
    })
  }

  pub fn parse_prototype(&mut self) -> (String, ArgList) {
    let token = self.current_or_panic();
    let fn_name = token.get_id_string();
    self.next();
    let args = self.parse_fn_args(); // consume '(' first_arg (, arg)* ')'
    (fn_name, args)
  }

  pub fn parse_fn_args(&mut self) -> ArgList {
    self
      .consume(TokenType::LParen)
      .expect("Expect ( but not found...");

    // No argment patern: "{fn_name}()"
    if self.consume(TokenType::RParen).is_some() {
      return ArgList::new();
    }

    let mut args = ArgList::new();
    let first_arg = self.parse_arg();
    args.push(first_arg);

    // with argments patern: "{fn_name}(first_arg (, arg)*)"
    while self.consume(TokenType::RParen).is_none() {
      self.consume_or_panic(TokenType::Comma);
      let arg = self.parse_arg();
      args.push(arg);
    }

    args
  }

  pub fn parse_arg(&mut self) -> Arg {
    let arg = self.current_or_panic();
    let arg_name = arg.get_id_string();
    self.next();
    Arg { name: arg_name }
  }

  pub fn parse_stmt_block(&mut self) -> Vec<Box<Stmt>> {
    self.consume_or_panic(TokenType::LBrace);
    let mut stmt_block = Vec::new();
    while self.consume(TokenType::RBrace).is_none() {
      match self.parse_stmt() {
        Some(stmt) => {
          stmt_block.push(stmt);
        }
        None => {
          break;
        }
      }
    }
    stmt_block
  }

  pub fn parse_stmt(&mut self) -> Option<Box<Stmt>> {
    let token = self.current()?;
    let stmt = match token.ty {
      TokenType::If => self.parse_if_stmt(),
      TokenType::For => self.parse_for_stmt(),
      TokenType::Return => self.parse_return_stmt(),
      _ => {
        let expr = self.parse_expr(Precedence::LOWEST)?;
        self.consume_or_panic(TokenType::Semicolon);
        Stmt::ExprStmt { expr: expr }
      }
    };
    Some(Box::new(stmt))
  }

  fn parse_if_stmt(&mut self) -> Stmt {
    self.next();
    self.consume_or_panic(TokenType::LParen);
    let expr = self
      .parse_expr(Precedence::LOWEST)
      .expect("'if' must have the condition...");
    self.consume_or_panic(TokenType::RParen);
    let true_stmt_block = self.parse_stmt_block();
    if true_stmt_block.len() == 0 {
      panic!("'if' must have at least 1 statement...")
    }

    let false_stmt_block = match self.consume(TokenType::Else) {
      Some(_) => {
        let false_stmt_block = self.parse_stmt_block();
        if false_stmt_block.len() == 0 {
          panic!("'else' must have at least 1 statement...")
        }
        Some(false_stmt_block)
      }
      None => None,
    };
    Stmt::IfStmt {
      cond: expr,
      true_body: true_stmt_block,
      false_body: false_stmt_block,
    }
  }

  fn parse_for_stmt(&mut self) -> Stmt {
    self.next();
    panic!("not support 'for yet...");
    self.consume_or_panic(TokenType::LParen);
    let prologue = self.parse_expr(Precedence::LOWEST);
    let condition = self.parse_expr(Precedence::LOWEST);
    let epilogue = self.parse_expr(Precedence::LOWEST);
    self.consume_or_panic(TokenType::RParen);
    // let true_stmt_block = self.parse_stmt_block();
    // if true_stmt_block.len() == 0 {
    //   panic!("'if' must have at least 1 statement...")
    // }

    // let false_stmt_block = match self.consume(TokenType::Else) {
    //   Some(_) => {
    //     let false_stmt_block = self.parse_stmt_block();
    //     if false_stmt_block.len() == 0 {
    //       panic!("'else' must have at least 1 statement...")
    //     }
    //     Some(false_stmt_block)
    //   }
    //   None => None,
    // };
    // Stmt::IfStmt {
    //   cond: expr,
    //   true_body: true_stmt_block,
    //   false_body: false_stmt_block,
    // }
  }

  fn parse_return_stmt(&mut self) -> Stmt {
    self.next();
    // parse 'return' stmt with the lhs
    if self.consume(TokenType::Semicolon).is_none() {
      println!("{}", self.current().unwrap());
      let expr = self
        .parse_expr(Precedence::LOWEST)
        .expect("'return' is followed by an unexpected expr...");
      self.consume_or_panic(TokenType::Semicolon);
      Stmt::ReturnStmt { expr: Some(expr) }
    } else {
      // ';' is already consumed in the condition
      Stmt::ReturnStmt { expr: None }
    }
  }

  fn parse_expr(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
    let mut lhs = self.parse_unary_op()?;
    while !self.on_eof() && precedence < self.current_precedence() {
      lhs = self.parse_binary_op(lhs)?;
    }
    Some(lhs)
  }

  fn parse_id(&mut self) -> Option<Box<Expr>> {
    let token = self.current()?;
    let pos = token.position;
    let name = token.get_id_string();
    self.next();
    Some(Box::new(Expr::Id {
      name: name,
      position: pos,
    }))
  }

  fn parse_number(&mut self) -> Option<Box<Expr>> {
    let token = self.current()?;
    let pos = token.position;
    match token.ty {
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
      TokenType::Id(_) => self.parse_id(),
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
      TokenType::Assign => BinaryOpType::Assign,
      TokenType::Eq => BinaryOpType::Eq,
      TokenType::Ne => BinaryOpType::Ne,
      TokenType::Lt => BinaryOpType::Lt,
      TokenType::Le => BinaryOpType::Le,
      TokenType::Gt => BinaryOpType::Gt,
      TokenType::Ge => BinaryOpType::Ge,
      _ => return Some(lhs),
    };
    let pos = token.position;
    let precedence = Self::token_precedence(token);
    self.next();

    println!("{}", self.current().unwrap());

    let rhs = self.parse_expr(precedence)?;
    println!("lhs: {}", rhs);
    println!("{}", self.current().unwrap());
    Some(Box::new(Expr::BinaryOp {
      op: op,
      lhs: lhs,
      rhs: rhs,
      position: pos,
    }))
  }

  fn parse_grouped_expr(&mut self) -> Option<Box<Expr>> {
    if self.consume(TokenType::LParen).is_none() {
      panic!("Expected '(' but {} found...", self.current()?);
    }
    let expr = self.parse_expr(Precedence::LOWEST);
    match self.consume(TokenType::RParen) {
      Some(_) => expr,
      None => panic!("Expected ')' but {} found...", self.current()?),
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

  fn token_precedence(token: &Token) -> Precedence {
    match token.ty {
      TokenType::Assign => Precedence::ASSIGN,
      TokenType::Eq
      | TokenType::Ne
      | TokenType::Lt
      | TokenType::Le
      | TokenType::Gt
      | TokenType::Ge => Precedence::COMPARE,
      TokenType::Plus | TokenType::Minus => Precedence::SUM,
      TokenType::Aster | TokenType::Slash => Precedence::PRODUCT,
      _ => Precedence::LOWEST,
    }
  }
}

#[test]
fn test_parser() {
  test_parse("1 + 2;", r#"Stmt(Add@[2,3]{Num@[0,1]{1}, Num@[4,5]{2}})"#);
  test_parse(
    "-5 + (4 - 20) * 4;",
    r#"Stmt(Add@[3,4]{Minus@[0,1]{Num@[1,2]{5}}, Mul@[14,15]{Sub@[8,9]{Num@[6,7]{4}, Num@[10,12]{20}}, Num@[16,17]{4}}})"#,
  );
}

#[cfg(test)]
fn test_parse(input: &str, expected: &str) {
  let mut lexer = Lexer::new(input.chars().collect());
  let token_list = lexer.tokenize();
  let mut parser = Parser::new(token_list);
  let module = parser.parse();
  assert_eq!(format!("{}", module.stmt_list[0]), expected);
}
