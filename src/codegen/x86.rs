use crate::ast::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

pub struct Codegen {
  pub code_list: Vec<String>,
  indent: usize,
}

impl Codegen {
  pub fn new() -> Self {
    Codegen {
      code_list: Vec::new(),
      indent: 0,
    }
  }

  pub fn run(&mut self, module: Box<Module>) {
    self.gen_module(module);
  }

  pub fn export(&mut self, file_path: &Path, module: Box<Module>) {
    self.run(module);
    let out_file = File::create(file_path).expect(&format!(
      "file create failed: {}",
      file_path.to_string_lossy()
    ));
    let mut writer = BufWriter::new(out_file);
    for line in &self.code_list {
      writer
        .write_all(line.as_bytes())
        .expect(&format!("Code line write failed at {}", &line));
    }
    writer.flush().expect(&format!(
      "Code export failed to {}",
      file_path.to_string_lossy()
    ))
  }

  fn gen_module(&mut self, module: Box<Module>) {
    self.gen_module_prolouge();
    self.gen_fn("main", module.stmt_list);
  }

  fn gen_module_prolouge(&mut self) {
    // self.set(".section __DATA,__data");
    // self.set("print_msg:");
    // self.inc_indent();
    // self.set(".asciz \"%d\\n\"");
    // self.dec_indent();
    // self.set_newline();
    // TEXT section
    self.set(".intel_syntax noprefix");
    self.set(".globl main");
    self.set_newline();
  }

  pub fn gen_fn(&mut self, fn_name: &str, stmt_list: Vec<Box<Stmt>>) {
    self.gen_fn_prolouge(fn_name);
    self.gen_fn_body(fn_name, stmt_list);
    self.gen_fn_epilouge(fn_name);
  }

  pub fn gen_fn_prolouge(&mut self, fn_name: &str) {
    self.set(&format!("{}:", fn_name));
    self.inc_indent();
    self.set("push rbp");
    self.set("mov rbp, rsp");
    self.set(&format!("# function '{}' begin", fn_name));
  }

  pub fn gen_fn_epilouge(&mut self, fn_name: &str) {
    self.set(&format!("# function '{}' end", fn_name));
    self.set("pop rax");
    self.set("mov rsp, rbp");
    self.set("pop rbp");
    self.set("ret");
    self.dec_indent();
  }

  pub fn gen_fn_body(&mut self, fn_name: &str, stmt_list: Vec<Box<Stmt>>) {
    for stmt in stmt_list {
      match *stmt {
        Stmt::ExprStmt { expr } => self.gen_expr(expr),
        _ => panic!("FnStmt is not supported now..."),
      }
    }
  }

  pub fn gen_expr(&mut self, expr: Box<Expr>) {
    match *expr {
      Expr::Number { value, position: _ } => {
        self.set(&format!("push {}", value));
      }
      Expr::UnaryOp {
        op,
        rhs,
        position: _,
      } => {
        self.gen_expr(rhs);
        self.set("pop rdi");
        self.set("mov rax, 0");
        match op {
          UnaryOpType::Minus => self.set("sub rax, rdi"),
          UnaryOpType::Plus => self.set("add rax, rdi"),
        }
        self.set("push rax");
      }
      Expr::BinaryOp {
        op,
        lhs,
        rhs,
        position: _,
      } => {
        self.gen_expr(lhs);
        self.gen_expr(rhs);
        self.set("pop rdi");
        self.set("pop rax");
        match op {
          BinaryOpType::Add => self.set("add rax, rdi"),
          BinaryOpType::Sub => self.set("sub rax, rdi"),
          BinaryOpType::Mul => self.set("imul rax, rdi"),
          BinaryOpType::Div => {
            self.set("cqo");
            self.set("idiv rdi");
          }
        }
        self.set("push rax");
      }
    }
  }

  fn set(&mut self, cmd: &str) {
    let mut indent = String::new();
    for _ in 0..self.indent {
      indent += " ";
    }
    self.code_list.push(format!("{}{}\n", indent, cmd));
  }

  fn set_newline(&mut self) {
    self.set("");
  }

  fn inc_indent(&mut self) {
    self.indent += 2;
  }

  fn dec_indent(&mut self) {
    self.indent -= 2;
  }

  // pub fn save(&mut self, ) -> Box<Vec<String>> {
  //   let file_name = String::from(file_name) + file_name;
  // let out_file = File::create(file_name)?;
  // let mut writer = BufWriter::new(out_file);
  // for line in self.asm_list {
  //   writer.write_all(line.as_bytes())?;
  // }
  // writer.flush()?;
  // }
}
