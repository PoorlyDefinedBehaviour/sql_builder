use crate::expression::Expression;
use std::fmt::{Debug, Write};

pub fn select<T: AsRef<str>>(columns: &[T]) -> Select<T> {
  Select::new(columns)
}

#[derive(Debug)]
pub struct Select<'a, T: AsRef<str>> {
  columns: &'a [T],
  table: Option<&'a dyn Expression>,
  where_column: Option<&'a dyn Expression>,
  where_operator: Option<&'a str>,
  where_value: Option<&'a dyn Expression>,
  joins: Vec<Join<'a>>,
  limit: Option<u32>,
  offset: Option<u32>,
}

#[derive(Debug, Clone)]
enum JoinType {
  Left,
  Right,
  Inner,
}

impl std::fmt::Display for JoinType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        JoinType::Left => "left",
        JoinType::Right => "right",
        JoinType::Inner => "inner",
      }
    )
  }
}

#[derive(Debug)]
struct Join<'a> {
  r#type: JoinType,
  table: &'a dyn Expression,
  left: &'a dyn Expression,
  operator: &'a str,
  right: &'a dyn Expression,
  alias: Option<&'a str>,
}

impl<'a> Expression for Join<'a> {
  fn build(&self) -> String {
    let mut buffer = String::new();

    write!(&mut buffer, "{} join {}", self.r#type, self.table.build()).unwrap();

    if let Some(alias) = self.alias {
      write!(&mut buffer, " as {}", alias).unwrap();
    }

    write!(
      &mut buffer,
      " on {} {} {}",
      self.left.build(),
      self.operator,
      self.right.build()
    )
    .unwrap();

    buffer
  }
}

impl<'a, T: AsRef<str>> Select<'a, T> {
  pub fn new(columns: &'a [T]) -> Self {
    Self {
      columns,
      table: None,
      where_column: None,
      where_operator: None,
      where_value: None,
      joins: Vec::new(),
      limit: None,
      offset: None,
    }
  }

  pub fn from(&mut self, table: &'a dyn Expression) -> &mut Self {
    self.table = Some(table);
    self
  }

  pub fn limit(&mut self, limit: u32) -> &mut Self {
    self.limit = Some(limit);
    self
  }

  pub fn offset(&mut self, offset: u32) -> &mut Self {
    self.offset = Some(offset);
    self
  }

  pub fn r#where(
    &mut self,
    column: &'a dyn Expression,
    operator: &'a str,
    value: &'a dyn Expression,
  ) -> &mut Self {
    self.where_column = Some(column);
    self.where_operator = Some(operator);
    self.where_value = Some(value);
    self
  }

  pub fn left_join(&'a mut self, table: &'a dyn Expression) -> JoinBuilder<'a, T> {
    JoinBuilder {
      r#type: JoinType::Left,
      parent_query: self,
      table,
      alias: None,
    }
  }

  pub fn right_join(&'a mut self, table: &'a dyn Expression) -> JoinBuilder<'a, T> {
    JoinBuilder {
      r#type: JoinType::Right,
      parent_query: self,
      table,
      alias: None,
    }
  }

  pub fn inner_join(&'a mut self, table: &'a dyn Expression) -> JoinBuilder<'a, T> {
    JoinBuilder {
      r#type: JoinType::Inner,
      parent_query: self,
      table,
      alias: None,
    }
  }

  fn join(&mut self, join: Join<'a>) {
    self.joins.push(join);
  }
}

pub struct JoinBuilder<'a, T: AsRef<str>> {
  r#type: JoinType,
  parent_query: &'a mut Select<'a, T>,
  table: &'a dyn Expression,
  alias: Option<&'a str>,
}

impl<'a, T: AsRef<str>> JoinBuilder<'a, T> {
  pub fn on(
    &'a mut self,
    left: &'a dyn Expression,
    operator: &'a str,
    right: &'a dyn Expression,
  ) -> &'a mut Select<'a, T> {
    self.parent_query.join(Join {
      r#type: self.r#type.clone(),
      table: self.table,
      left,
      operator,
      right,
      alias: self.alias,
    });
    self.parent_query
  }

  pub fn r#as(&'a mut self, alias: &'a str) -> &'a mut Self {
    self.alias = Some(alias);
    self
  }
}

impl<'a, T: AsRef<str> + Debug> Expression for Select<'a, T> {
  fn build(&self) -> String {
    let mut query = String::new();

    write!(&mut query, "(select ").unwrap();

    for (i, column) in self.columns.iter().enumerate() {
      if i > 0 {
        write!(&mut query, ", {}", column.as_ref()).unwrap();
      } else {
        write!(&mut query, "{}", column.as_ref()).unwrap();
      }
    }

    write!(
      &mut query,
      " from {}",
      self.table.expect("missing table").build()
    )
    .unwrap();

    for join in self.joins.iter() {
      write!(&mut query, " {}", join.build()).unwrap();
    }

    if self.where_column.is_some() {
      write!(
        &mut query,
        " where {} {} {}",
        self
          .where_column
          .expect("missing condition in where column")
          .build(),
        self
          .where_operator
          .expect("missing operator in where condition"),
        self
          .where_value
          .expect("missing value in where condition")
          .build()
      )
      .unwrap();
    }

    if let Some(offset) = self.offset {
      write!(&mut query, " offset {}", offset).unwrap();
    }

    if let Some(limit) = self.limit {
      write!(&mut query, " limit {}", limit).unwrap();
    }

    write!(&mut query, ")").unwrap();

    query
  }
}
