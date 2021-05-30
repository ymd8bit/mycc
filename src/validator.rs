use crate::ast::{Module, Stmt};

struct ModuleValidator {
  module: Box<Module>,
}

impl ModuleValidator {
  pub fn new(module: Box<Module>) -> Self {
    ModuleValidator { module: module }
  }

  pub fn check(&self) {
    let mut main_found = false;
    for stmt in self.module.stmt_list {
      match stmt {
        Stmt::FnStmt { name, args, body } => {
          if name == "main" {
            main_found = true;
          }
        }
        _ => panic!("currently FnStmt is only supported..."),
      }
    }
    if !main_found {
      panic!("'main' function not found in the module");
    }
  }
}
