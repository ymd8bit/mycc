use crate::token::Position;
use crate::utils::ToSimpleString;
use std::fmt;

#[derive(Debug)]
pub enum UnaryOpType {
  Plus,
  Minus,
}

impl ToSimpleString for UnaryOpType {
  fn to_simple_string(&self) -> String {
    match self {
      UnaryOpType::Plus => String::from("Plus"),
      UnaryOpType::Minus => String::from("Minus"),
    }
  }
}

impl fmt::Display for UnaryOpType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
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

impl ToSimpleString for BinaryOpType {
  fn to_simple_string(&self) -> String {
    match self {
      BinaryOpType::Add => String::from("Add"),
      BinaryOpType::Sub => String::from("Sub"),
      BinaryOpType::Mul => String::from("Mul"),
      BinaryOpType::Div => String::from("Div"),
    }
  }
}

impl fmt::Display for BinaryOpType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

#[derive(Debug)]
pub enum Expr {
  Number {
    value: u64,
    position: Position,
  },
  UnaryOp {
    op: UnaryOpType,
    rhs: Box<Expr>,
    position: Position,
  },
  BinaryOp {
    op: BinaryOpType,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    position: Position,
  },
}

impl ToSimpleString for Expr {
  fn to_simple_string(&self) -> String {
    match self {
      Expr::Number { value, position } => format!("Num{}{{{}}}", position, value),
      Expr::UnaryOp { op, rhs, position } => format!("{}{}{{{}}}", op, position, rhs),
      Expr::BinaryOp {
        op,
        rhs,
        lhs,
        position,
      } => format!("{}{}{{{}, {}}}", op, position, lhs, rhs),
    }
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

#[derive(Debug)]
pub struct Arg {
  name: String,
}

#[derive(Debug)]
pub enum Stmt {
  ExprStmt { expr: Box<Expr> },
  FnStmt { name: String, args: Vec<Arg> },
}

impl ToSimpleString for Stmt {
  fn to_simple_string(&self) -> String {
    match self {
      Stmt::ExprStmt { expr } => format!("Stmt({})", expr),
      Stmt::FnStmt { name, args } => {
        format!("Fn({}, {:?})", name, args)
      }
    }
  }
}
impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

#[derive(Debug)]
pub struct Module {
  pub stmt_list: Vec<Box<Stmt>>,
}

impl Module {
  pub fn new() -> Module {
    Module {
      stmt_list: Vec::new(),
    }
  }

  pub fn add_stmt(&mut self, stmt: Box<Stmt>) {
    self.stmt_list.push(stmt);
  }
}

impl ToSimpleString for Module {
  fn to_simple_string(&self) -> String {
    let mut s = String::from("Module {\n");
    for (i, stmt) in self.stmt_list.iter().enumerate() {
      s.push_str(&format!("  {}: ", i));
      s.push_str(&stmt.to_simple_string());
      s.push('\n');
    }
    s.push_str("}");
    s
  }
}
impl fmt::Display for Module {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}
