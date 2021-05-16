use std::fmt;

pub trait ToSimpleString: fmt::Display {
  fn to_simple_string(&self) -> String;
}
