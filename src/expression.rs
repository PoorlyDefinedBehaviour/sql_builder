use std::fmt::Debug;

pub trait Expression: Debug {
  fn build(&self) -> String;
}

impl Expression for String {
  fn build(&self) -> String {
    format!("\"{}\"", self)
  }
}

impl<'a> Expression for &'a str {
  fn build(&self) -> String {
    format!("\"{}\"", self)
  }
}

impl Expression for str {
  fn build(&self) -> String {
    format!("\"{}\"", self)
  }
}

impl Expression for i32 {
  fn build(&self) -> String {
    self.to_string()
  }
}

impl Expression for i64 {
  fn build(&self) -> String {
    self.to_string()
  }
}

impl Expression for u32 {
  fn build(&self) -> String {
    self.to_string()
  }
}

impl Expression for u64 {
  fn build(&self) -> String {
    self.to_string()
  }
}

impl Expression for bool {
  fn build(&self) -> String {
    if *self {
      String::from("true")
    } else {
      String::from("false")
    }
  }
}
