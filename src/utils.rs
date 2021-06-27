use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufWriter};
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn print_file(file_path: &Path) {
  if let Ok(lines) = read_lines(file_path) {
    for line in lines {
      if let Ok(ip) = line {
        println!("{}", ip);
      }
    }
  }
}
