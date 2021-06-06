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
  Assign,
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
      BinaryOpType::Assign => String::from("Assign"),
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
  Id {
    name: String,
    position: Position,
  },
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
      Expr::Id { name, position } => format!("Id{}{{{}}}", position, name),
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

#[derive(Debug, Clone)]
pub struct Arg {
  pub name: String,
}
impl ToSimpleString for Arg {
  fn to_simple_string(&self) -> String {
    format!("'{}'", self.name)
  }
}
impl fmt::Display for Arg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

#[derive(Debug)]
pub struct ArgList {
  pub container: Vec<Arg>,
  index: usize,
}
impl ArgList {
  pub fn new() -> Self {
    ArgList {
      container: Vec::new(),
      index: 0,
    }
  }

  // pub fn len(&self) -> usize {
  //   self.container.len()
  // }

  pub fn push(&mut self, arg: Arg) {
    self.container.push(arg);
  }
}

impl ToSimpleString for ArgList {
  fn to_simple_string(&self) -> String {
    let mut args_str = String::from("[");
    for (i, arg) in self.container.iter().enumerate() {
      if i != 0 {
        args_str.push_str(", ");
      }
      args_str.push_str(&arg.to_simple_string());
    }
    args_str.push_str("]");
    args_str
  }
}
impl fmt::Display for ArgList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_simple_string())
  }
}

#[derive(Debug)]
pub enum Stmt {
  ExprStmt {
    expr: Box<Expr>,
  },
  ReturnStmt {
    expr: Option<Box<Expr>>,
  },
  FnStmt {
    name: String,
    args: ArgList,
    body: Vec<Box<Stmt>>,
  },
}

impl ToSimpleString for Stmt {
  fn to_simple_string(&self) -> String {
    match self {
      Stmt::ExprStmt { expr } => format!("Stmt({})", expr),
      Stmt::ReturnStmt { expr } => match expr {
        Some(expr) => format!("Return({})", expr),
        None => format!("Return()"),
      },
      Stmt::FnStmt { name, args, body } => {
        let mut fn_str = format!("Fn({}, {}) {{", name, args);
        for (i, stmt) in body.iter().enumerate() {
          fn_str.push_str("\n");
          fn_str.push_str(&format!("  {}: ", i));
          fn_str.push_str(&stmt.to_simple_string());
        }
        fn_str.push_str("}");
        fn_str
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
