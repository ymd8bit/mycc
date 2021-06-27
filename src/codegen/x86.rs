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
    let offset = (self.index + 1) * 8;
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
  label_index: usize,
  rsp_count: i64,
}

impl Codegen {
  pub fn new() -> Self {
    Codegen {
      code_list: Vec::new(),
      indent: 0,
      label_index: 0,
      rsp_count: 0,
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
          self.set_newline();
        }
        _ => panic!("currently FnStmt is only supported..."),
      }
    }
  }

  fn gen_module_prolouge(&mut self) {
    self.set(".intel_syntax noprefix");
    self.set_newline();
    self.set(".text");
    self.set(".section .rodata");
    self.set(".LC0:");
    self.inc_indent();
    self.set(".string \"%lu\\n\"");
    self.set(".text");
    self.dec_indent();
    self.set_newline();
    self.set(".globl main");
    self.set_newline();
  }

  fn gen_fn(&mut self, name: &str, args: ArgList, body: Vec<Box<Stmt>>) {
    self.rsp_count = 0;
    let mut env = Env::new();
    self.gen_fn_prolouge(&name, &env);

    let reg_names = self.arg_register_names();
    for (i, arg) in args.container.iter().enumerate() {
      let offset = env.alloc(&arg.name);
      let reg_name = reg_names[i];
      self.set(&format!("mov -{}[rbp], {}", offset, reg_name));
    }
    self.set(&format!("sub rsp, {}", args.container.len() * 8));

    self.gen_block(body, &mut env);
    self.gen_fn_epilouge(&name);
  }

  fn gen_fn_prolouge(&mut self, name: &str, _env: &Env) {
    self.set(&format!("{}:", name));
    self.inc_indent();
    self.set_push("rbp");
    self.set("mov rbp, rsp");
    self.set(&format!("# function '{}' begin", name));
  }

  fn gen_fn_epilouge(&mut self, name: &str) {
    self.set(&format!("# function '{}' end", name));
    self.set_pop("rax");
    self.set("mov rsp, rbp");
    self.set_pop("rbp");
    self.set("ret");
    self.dec_indent();
  }

  fn gen_block(&mut self, body: Vec<Box<Stmt>>, env: &mut Env) {
    for stmt in body {
      match *stmt {
        Stmt::ExprStmt { expr } => self.gen_expr(&expr, env),
        Stmt::IfStmt {
          cond,
          true_body,
          false_body,
        } => self.gen_if(cond, true_body, false_body, env),
        Stmt::ForStmt {
          cond,
          prologue,
          epilogue,
          body,
        } => self.gen_for(cond, prologue, epilogue, body, env),
        Stmt::ReturnStmt { expr } => {
          self.gen_return(expr, env);
        }
        Stmt::FnStmt { .. } => panic!("FnStmt is not supported in a function..."),
      }
    }
  }

  fn gen_lvalue(&mut self, expr: &Box<Expr>, env: &mut Env, alloc_ok: bool) {
    match &**expr {
      Expr::Id { name, position: _ } => {
        let offset = match env.get_offset(&name) {
          Some(offset) => offset,
          None => {
            if alloc_ok {
              env.alloc(&name)
            } else {
              panic!("Non allocated id found...")
            }
          }
        };
        self.set("mov rax, rbp");
        self.set(&format!("sub rax, {}", offset));
        self.set_push("rax");
      }
      _ => panic!("Only Id can be refered as lvalue..."),
    }
  }

  fn gen_if(
    &mut self,
    cond: Box<Expr>,
    true_body: Vec<Box<Stmt>>,
    false_body: Option<Vec<Box<Stmt>>>,
    env: &mut Env,
  ) {
    self.gen_expr(&&cond, env);
    self.set_pop("rax");
    self.set("cmp rax, 0");
    if let Some(false_body) = false_body {
      self.set(&format!("je .Lelse_{}", self.label_index));
      self.gen_block(true_body, env);
      self.set(&format!("jmp .Lend_{}", self.label_index));
      self.set(&format!(".Lelse_{}:", self.label_index));
      self.gen_block(false_body, env);
    } else {
      self.set(&format!("je .Lend_{}", self.label_index));
      self.gen_block(true_body, env);
    }
    self.set(&format!(".Lend_{}:", self.label_index));
    self.label_index += 1;
  }

  fn gen_for(
    &mut self,
    cond: Option<Box<Expr>>,
    prologue: Option<Box<Expr>>,
    epilogue: Option<Box<Expr>>,
    body: Vec<Box<Stmt>>,
    env: &mut Env,
  ) {
    if let Some(expr) = prologue {
      self.gen_expr(&expr, env);
    }
    let label_begin = self.set_label("for_begin");
    let label_end = self.make_label("for_end");
    if let Some(expr) = cond {
      self.gen_expr(&expr, env);
    }
    self.set_pop("rax");
    self.set("cmp rax, 0");
    self.set(&format!("je {}", label_end));
    self.gen_block(body, env);
    if let Some(expr) = epilogue {
      self.gen_expr(&expr, env);
    }
    self.set(&format!("jmp {}", label_begin));
    let _ = self.set_label("for_end");
    self.label_index += 1;
  }

  fn gen_return(&mut self, lhs: Option<Box<Expr>>, env: &mut Env) {
    if let Some(lhs) = lhs {
      self.gen_expr(&lhs, env);
    }
    self.set_pop("rax");
    self.set("mov rsp, rbp");
    self.set_pop("rbp");
    self.set("ret");
  }

  fn gen_expr(&mut self, expr: &Box<Expr>, env: &mut Env) {
    match &**expr {
      Expr::Id {
        name: _,
        position: _,
      } => {
        self.gen_lvalue(expr, env, false);
        self.set_pop("rax");
        self.set("mov rax, [rax]");
        self.set_push("rax");
      }
      Expr::Number { value, position: _ } => {
        self.set_push(&value.to_string());
      }
      Expr::Call {
        name,
        args,
        position: _,
      } => {
        if args.len() > 6 {
          panic!("Currently call args must be less than 7...");
        }
        for arg in args.iter() {
          self.gen_expr(&arg, env);
        }
        // we must pop in reverse order, so indexing is complex
        let reg_names = self.arg_register_names();
        let num_args = args.len();
        for i in 1..=num_args {
          let reg_name = reg_names[num_args - i];
          self.set_pop("rax");
          self.set(&format!("mov {}, rax", reg_name));
        }
        // push dummy for aligning RSP by 16 bytes
        if self.rsp_count % 2 == 0 {
          self.set("sub rsp, 8");
        }
        self.set(&format!("call {}", name));
        // pop dummy for aligning RSP by 16 bytes
        if self.rsp_count % 2 == 0 {
          self.set("add rsp, 8");
        }
        self.set_push("rax");
      }
      Expr::UnaryOp {
        op,
        rhs,
        position: _,
      } => {
        self.gen_expr(&rhs, env);
        self.set_pop("rdi");
        self.set("mov rax, 0");
        match op {
          UnaryOpType::Minus => self.set("sub rax, rdi"),
          UnaryOpType::Plus => self.set("add rax, rdi"),
        }
        self.set_push("rax");
      }
      Expr::BinaryOp {
        op,
        lhs,
        rhs,
        position: _,
      } => {
        match op {
          BinaryOpType::Assign => {
            self.gen_lvalue(&lhs, env, true);
            self.gen_expr(&rhs, env);
            self.set_pop("rdi");
            self.set_pop("rax");
            self.set("mov [rax], rdi");
            self.set_push("rdi");
          }
          BinaryOpType::Inc | BinaryOpType::Dec => {
            self.gen_lvalue(&lhs, env, false);
            self.gen_expr(&rhs, env);
            self.set_pop("rdi");
            self.set_pop("rax");
            self.set("mov rcx, [rax]");
            match op {
              BinaryOpType::Inc => self.set("add rcx, rdi"),
              BinaryOpType::Dec => self.set("sub rcx, rdi"),
              _ => panic!("Unreachable"),
            };
            self.set("mov [rax], rcx");
            self.set_push("rdi");
          }
          _ => {
            self.gen_expr(&lhs, env);
            self.gen_expr(&rhs, env);
            self.set_pop("rdi");
            self.set_pop("rax");
            match op {
              BinaryOpType::Add => self.set("add rax, rdi"),
              BinaryOpType::Sub => self.set("sub rax, rdi"),
              BinaryOpType::Mul => self.set("imul rax, rdi"),
              BinaryOpType::Div => {
                self.set("cqo");
                self.set("idiv rdi");
              }
              _ => {
                self.set("cmp rax, rdi");
                match op {
                  BinaryOpType::Eq => {
                    self.set("sete al");
                  }
                  BinaryOpType::Ne => {
                    self.set("setne al");
                  }
                  BinaryOpType::Lt => {
                    self.set("setl al");
                  }
                  BinaryOpType::Le => {
                    self.set("setle al");
                  }
                  BinaryOpType::Gt => {
                    self.set("setg al");
                  }
                  BinaryOpType::Ge => {
                    self.set("setge al");
                  }
                  _ => panic!("Unreachable"),
                };
                self.set("movzb rax, al");
              }
            };
          }
        };
        self.set_push("rax");
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

  fn set_push(&mut self, reg_name: &str) {
    self.set(&format!("push {}", reg_name));
    self.rsp_count += 1;
  }

  fn set_pop(&mut self, reg_name: &str) {
    self.set(&format!("pop {}", reg_name));
    self.rsp_count -= 1;
  }

  fn set_label(&mut self, name: &str) -> String {
    let label = self.make_label(name);
    self.code_list.push(format!("{}:\n", label));
    label
  }

  fn make_label(&mut self, name: &str) -> String {
    format!(".L{}_{}", name, self.label_index)
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

  fn arg_register_names(&self) -> Vec<&'static str> {
    vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"]
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
