use crate::expression::Expression;
use std::fmt::Write;
pub fn update(table: &str) -> Update {
  Update::new(table)
}

#[derive(Debug)]
pub struct Update<'a> {
  table: &'a str,
  where_column: Option<&'a dyn Expression>,
  where_operator: Option<&'a str>,
  where_value: Option<&'a dyn Expression>,
  joins: Vec<Join<'a>>,
  set_clauses: Vec<(&'a str, &'a dyn Expression)>,
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

pub struct JoinBuilder<'a> {
  r#type: JoinType,
  parent_query: &'a mut Update<'a>,
  table: &'a dyn Expression,
  alias: Option<&'a str>,
}

impl<'a> JoinBuilder<'a> {
  pub fn on(
    &'a mut self,
    left: &'a dyn Expression,
    operator: &'a str,
    right: &'a dyn Expression,
  ) -> &'a mut Update<'a> {
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

impl<'a> Expression for Update<'a> {
  fn build(&self) -> String {
    let mut query = String::new();

    write!(&mut query, "(update {}", self.table).unwrap();

    for join in self.joins.iter() {
      write!(&mut query, " {}", join.build()).unwrap();
    }

    for (i, (key, value)) in self.set_clauses.iter().enumerate() {
      if i > 0 {
        write!(&mut query, ", set {} = {}", key, value.build()).unwrap();
      } else {
        write!(&mut query, " set {} = {}", key, value.build()).unwrap();
      }
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

    write!(&mut query, ")").unwrap();

    query
  }
}

impl<'a> Update<'a> {
  fn new(table: &'a str) -> Self {
    Self {
      table,
      where_column: None,
      where_operator: None,
      where_value: None,
      joins: Vec::new(),
      set_clauses: Vec::new(),
    }
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

  pub fn left_join(&'a mut self, table: &'a dyn Expression) -> JoinBuilder<'a> {
    JoinBuilder {
      r#type: JoinType::Left,
      parent_query: self,
      table,
      alias: None,
    }
  }

  pub fn right_join(&'a mut self, table: &'a dyn Expression) -> JoinBuilder<'a> {
    JoinBuilder {
      r#type: JoinType::Right,
      parent_query: self,
      table,
      alias: None,
    }
  }

  pub fn inner_join(&'a mut self, table: &'a dyn Expression) -> JoinBuilder<'a> {
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

  pub fn set(&'a mut self, column: &'a str, value: &'a dyn Expression) -> &'a mut Self {
    self.set_clauses.push((column, value));
    self
  }
}
