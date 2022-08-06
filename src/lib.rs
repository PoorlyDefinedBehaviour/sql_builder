pub mod expression;
pub mod select;
pub use select::select;
pub mod update;
pub use update::update;

#[cfg(test)]
mod tests {
  use crate::expression::Expression;

  use super::*;

  #[test]
  fn select_tests() {
    let tests = vec![
      (
        select(&["user.id", "user.name", "user.email"])
          .from(&"user")
          .build(),
        r#"(select user.id, user.name, user.email from "user")"#,
      ),
      (
        select(&["user.id", "user.name", "user.email"])
          .from(select(&["*"]).from(&"user"))
          .build(),
        r#"(select user.id, user.name, user.email from (select * from "user"))"#,
      ),
      (
        select(&["user.id", "user.name", "user.email"])
          .from(&"user")
          .r#where(&"user.name", "=", &"john doe")
          .build(),
        r#"(select user.id, user.name, user.email from "user" where "user.name" = "john doe")"#,
      ),
    ];

    for (q, expected) in tests {
      assert_eq!(q, expected);
    }
  }

  #[test]
  fn left_join_tests() {
    let tests = vec![
      (
        select(&["*"])
          .from(&"user")
          .left_join(&"address")
          .on(&"address.id", "=", &3)
          .build(),
        r#"(select * from "user" left join "address" on "address.id" = 3)"#,
      ),
      (
        select(&["*"])
          .from(&"user")
          .left_join(
            select(&["*"])
              .from(&"destination")
              .r#where(&"destination.name", "=", &"VK"),
          )
          .r#as("destination")
          .on(&"destination.visited", "=", &false)
          .r#where(&"user.created_at", ">", &"2020-04-01")
          .build(),
        r#"(select * from "user" left join (select * from "destination" where "destination.name" = "VK") as destination on "destination.visited" = false where "user.created_at" > "2020-04-01")"#,
      ),
    ];

    for (q, expected) in tests {
      assert_eq!(q, expected);
    }
  }

  #[test]
  fn limit() {
    assert_eq!(
      select(&["id"]).from(&"user").limit(10).build(),
      r#"(select id from "user" limit 10)"#
    )
  }

  #[test]
  fn offset() {
    assert_eq!(
      select(&["id"]).from(&"user").offset(10).build(),
      r#"(select id from "user" offset 10)"#
    )
  }

  #[test]
  fn right_join_tests() {
    let tests = vec![
      (
        select(&["*"])
          .from(&"user")
          .right_join(&"address")
          .on(&"address.id", "=", &3)
          .build(),
        r#"(select * from "user" right join "address" on "address.id" = 3)"#,
      ),
      (
        select(&["*"])
          .from(&"user")
          .right_join(
            select(&["*"])
              .from(&"destination")
              .r#where(&"destination.name", "=", &"VK"),
          )
          .r#as("destination")
          .on(&"destination.visited", "=", &false)
          .r#where(&"user.created_at", ">", &"2020-04-01")
          .build(),
        r#"(select * from "user" right join (select * from "destination" where "destination.name" = "VK") as destination on "destination.visited" = false where "user.created_at" > "2020-04-01")"#,
      ),
    ];

    for (q, expected) in tests {
      assert_eq!(q, expected);
    }
  }

  #[test]
  fn inner_join_tests() {
    let tests = vec![
      (
        select(&["*"])
          .from(&"user")
          .inner_join(&"address")
          .on(&"address.id", "=", &3)
          .build(),
        r#"(select * from "user" inner join "address" on "address.id" = 3)"#,
      ),
      (
        select(&["*"])
          .from(&"user")
          .inner_join(
            select(&["*"])
              .from(&"destination")
              .r#where(&"destination.name", "=", &"VK"),
          )
          .r#as("destination")
          .on(&"destination.visited", "=", &false)
          .r#where(&"user.created_at", ">", &"2020-04-01")
          .build(),
        r#"(select * from "user" inner join (select * from "destination" where "destination.name" = "VK") as destination on "destination.visited" = false where "user.created_at" > "2020-04-01")"#,
      ),
    ];

    for (q, expected) in tests {
      assert_eq!(q, expected);
    }
  }

  #[test]
  fn update_tests() {
    let tests = vec![
      (
        update("employee")
          .set("name", &"John Doe")
          .r#where(&"id", "<", &100)
          .build(),
        r#"(update employee set name = "John Doe" where "id" < 100)"#,
      ),
      (
        update("employee")
          .set("name", &"John Doe")
          .r#where(&"id", "<", &100)
          .build(),
        r#"(update employee set name = "John Doe" where "id" < 100)"#,
      ),
      (
        update("employee")
          .inner_join(&"address")
          .on(&"address.id", "=", &"employee.address_id")
          .set("address.street", &"name")
          .set("address.number", &"number")
          .set("address.city_id", &1)
          .r#where(&"employee.id", "=", &2)
          .build(),
        r#"(update employee inner join "address" on "address.id" = "employee.address_id" set address.street = "name", set address.number = "number", set address.city_id = 1 where "employee.id" = 2)"#,
      ),
    ];

    for (q, expected) in tests {
      assert_eq!(q, expected);
    }
  }
}
