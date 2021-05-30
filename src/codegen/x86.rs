use crate::ast::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

struct Env {
  sym_table: HashMap<String, usize>,
  index: usize,
}
impl Env {
  fn new() -> Self {
    Env {
      sym_table: HashMap::new(),
      index: 0,
    }
  }

  fn alloc(&mut self, var_name: &str) -> usize {
    let offset = self.index * 8;
    self.sym_table.insert(String::from(var_name), offset);
    self.index += 1;
    offset
  }

  fn get_offset(&mut self, var_name: &str) -> Option<usize> {
    match self.sym_table.get(var_name) {
      Some(offset) => Some(offset.clone()),
      None => None,
    }
  }

  // fn allocated(&mut self, var_name: &str) -> bool {
  //   self.sym_table.contains_key(var_name)
  // }

  // fn num_vars(&self) -> usize {
  //   self.sym_table.len()
  // }
}

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

    for stmt in module.stmt_list {
      match *stmt {
        Stmt::FnStmt { name, args, body } => {
          self.gen_fn(&name, args, body);
        }
        _ => panic!("currently FnStmt is only supported..."),
      }
    }
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

  fn gen_fn(&mut self, name: &str, args: ArgList, body: Vec<Box<Stmt>>) {
    let mut env = Env::new();
    for arg in args.container.iter() {
      env.alloc(&arg.name);
    }
    self.gen_fn_prolouge(&name, &env);
    self.gen_fn_body(&name, args, body, &mut env);
    self.gen_fn_epilouge(&name);
  }

  fn gen_fn_prolouge(&mut self, name: &str, _env: &Env) {
    self.set(&format!("{}:", name));
    self.inc_indent();
    self.set("push rbp");
    self.set("mov rbp, rsp");
    self.set(&format!("# function '{}' begin", name));
  }

  fn gen_fn_epilouge(&mut self, name: &str) {
    self.set(&format!("# function '{}' end", name));
    self.set("pop rax");
    self.set("mov rsp, rbp");
    self.set("pop rbp");
    self.set("ret");
    self.dec_indent();
  }

  fn gen_fn_body(&mut self, _name: &str, _args: ArgList, body: Vec<Box<Stmt>>, env: &mut Env) {
    for stmt in body {
      match *stmt {
        Stmt::ExprStmt { expr } => self.gen_expr(expr, env),
        _ => panic!("FnStmt is not supported in a function..."),
      }
    }
  }

  fn gen_lvalue(&mut self, expr: Box<Expr>, env: &mut Env) {
    match *expr {
      Expr::Id { name, position: _ } => {
        let offset = match env.get_offset(&name) {
          Some(offset) => offset,
          None => env.alloc(&name),
        };
        self.set("mov rax, rbp");
        self.set(&format!("sub rax, {}", offset));
        self.set("push rax");
      }
      _ => panic!("Only Id can be refered as lvalue..."),
    }
  }

  fn gen_expr(&mut self, expr: Box<Expr>, env: &mut Env) {
    match *expr {
      Expr::Id {
        name: _,
        position: _,
      } => {
        self.gen_lvalue(expr, env);
        self.set("pop rax");
        self.set("mov rax, [rax]");
        self.set("push rax");
      }
      Expr::Number { value, position: _ } => {
        self.set(&format!("push {}", value));
      }
      Expr::UnaryOp {
        op,
        rhs,
        position: _,
      } => {
        self.gen_expr(rhs, env);
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
        match op {
          BinaryOpType::Assign => {
            self.gen_lvalue(lhs, env);
            self.gen_expr(rhs, env);
            self.set("pop rdi");
            self.set("pop rax");
            self.set("mov [rax], rdi");
            self.set("push rdi");
          }
          _ => {
            self.gen_expr(lhs, env);
            self.gen_expr(rhs, env);
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
              _ => (),
            };
          }
        };
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
