use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

pub trait ToSimpleString: fmt::Display {
  fn to_simple_string(&self) -> String;
}

pub struct Printer {
  contents: Vec<String>,
  indent: usize,
}

impl Printer {
  pub fn new() -> Self {
    Printer {
      contents: Vec::new(),
      indent: 0,
    }
  }

  pub fn show(&self) {
    println!("{}", self.get_string());
  }

  pub fn get_string(&self) -> String {
    self.contents.join("\n")
  }

  fn set(&mut self, cmd: &str) {
    let mut indent = String::new();
    for _ in 0..self.indent {
      indent += " ";
    }
    self.contents.push(format!("{}{}\n", indent, cmd));
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
}
