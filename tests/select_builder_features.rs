use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn select_builder_should_be_displayable() {
  let select = sql::Select::new().select("id, login").from("users");

  println!("{}", select);

  let query = select.as_string();
  let expected_query = "SELECT id, login FROM users";

  assert_eq!(query, expected_query);
}

#[test]
fn select_builder_should_be_debuggable() {
  let select = sql::Select::new().select("*").from("orders").where_clause("id = $1");

  println!("{:?}", select);

  let expected_query = "SELECT * FROM orders WHERE id = $1";
  let query = select.as_string();

  assert_eq!(query, expected_query);
}

#[test]
fn select_builder_should_be_cloneable() {
  let select_zipcode = sql::Select::new()
    .raw("/* test raw */")
    .select("zipcode")
    .from("address")
    .raw_before(sql::SelectClause::Where, "/* test raw_before */")
    .where_clause("login = $1")
    .raw_after(sql::SelectClause::Where, "/* test raw_after */");

  let select_city = select_zipcode.clone().select("city");

  let query_zipcode = select_zipcode.as_string();
  let query_city = select_city.as_string();

  let expected_query_zipcode = "\
    /* test raw */ \
    SELECT zipcode \
    FROM address \
    /* test raw_before */ \
    WHERE login = $1 \
    /* test raw_after */\
  ";
  let expected_query_city = "\
    /* test raw */ \
    SELECT zipcode, city \
    FROM address \
    /* test raw_before */ \
    WHERE login = $1 \
    /* test raw_after */\
  ";

  assert_eq!(query_zipcode, expected_query_zipcode);
  assert_eq!(query_city, expected_query_city);
}

#[test]
fn select_builder_should_be_able_to_conditionally_add_clauses() {
  let mut select = sql::Select::new().select("zipcode").from("address");

  if true {
    select = select.where_clause("login = $1").limit("$2");
  }

  let query = select.as_string();
  let expected_query = "SELECT zipcode FROM address WHERE login = $1 LIMIT $2";

  assert_eq!(query, expected_query);
}

#[test]
fn select_builder_should_be_composable() {
  fn project(select: sql::Select) -> sql::Select {
    select
      .select("u.id, u.name as user_name, u.login")
      .select("a.name as address_name")
      .select("o.name as product_name")
  }

  fn joins(select: sql::Select) -> sql::Select {
    select
      .from("users u")
      .inner_join("address a ON a.user_login = u.login")
      .inner_join("orders o ON o.user_login = u.login")
  }

  fn conditions(select: sql::Select) -> sql::Select {
    select.where_clause("u.login = $1").and("o.id = $2")
  }

  fn as_string(select: sql::Select) -> String {
    select.as_string()
  }

  let query = Some(sql::Select::new())
    .map(project)
    .map(joins)
    .map(conditions)
    .map(as_string)
    .unwrap();

  let expected_query = "\
      SELECT u.id, u.name as user_name, u.login, a.name as address_name, o.name as product_name \
      FROM users u \
      INNER JOIN address a ON a.user_login = u.login \
      INNER JOIN orders o ON o.user_login = u.login \
      WHERE u.login = $1 AND o.id = $2\
    ";

  assert_eq!(query, expected_query);
}
