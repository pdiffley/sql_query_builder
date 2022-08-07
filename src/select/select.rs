use crate::{
  behavior::{push_unique, Concat, WithQuery},
  fmt,
  structure::{SelectBuilder, SelectClause},
};

impl<'a> SelectBuilder<'a> {
  /// The same as `where_clause` method, useful to write more idiomatic SQL query
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .where_clause("login = foo")
  ///   .and("active = true");
  /// ```
  pub fn and(mut self, condition: &'a str) -> Self {
    self = self.where_clause(condition);
    self
  }

  /// Gets the current state of the SelectBuilder returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the SelectBuilder into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users")
  ///   .where_clause("login = foo")
  ///   .and("active = true")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// WHERE login = foo AND active = true
  /// ```
  ///
  /// You can debug different parts of the select putting it in another position
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select_query = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users")
  ///   .debug()
  ///   .where_clause("login = foo")
  ///   .and("active = true")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// The except clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn except(mut self, select: Self) -> Self {
    self._except.push(select);
    self
  }

  /// The from clause
  pub fn from(mut self, tables: &'a str) -> Self {
    push_unique(&mut self._from, tables.trim().to_owned());
    self
  }

  /// The group by clause
  pub fn group_by(mut self, column: &'a str) -> Self {
    push_unique(&mut self._group_by, column.trim().to_owned());
    self
  }

  /// The having clause
  pub fn having(mut self, condition: &'a str) -> Self {
    push_unique(&mut self._having, condition.trim().to_owned());
    self
  }

  /// The cross join clause
  pub fn cross_join(mut self, table: &'a str) -> Self {
    let table = table.trim();
    let table = format!("CROSS JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The inner join clause
  pub fn inner_join(mut self, table: &'a str) -> Self {
    let table = table.trim();
    let table = format!("INNER JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The left join clause
  pub fn left_join(mut self, table: &'a str) -> Self {
    let table = table.trim();
    let table = format!("LEFT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The right join clause
  pub fn right_join(mut self, table: &'a str) -> Self {
    let table = table.trim();
    let table = format!("RIGHT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The intersect clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn intersect(mut self, select: Self) -> Self {
    self._intersect.push(select);
    self
  }

  /// The limit clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .limit("123");
  ///
  /// let select = SelectBuilder::new()
  ///   .limit("1000")
  ///   .limit("123");
  /// ```
  pub fn limit(mut self, num: &'a str) -> Self {
    self._limit = num.trim();
    self
  }

  /// Create SelectBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The offset clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .offset("1500");
  ///
  /// let select = SelectBuilder::new()
  ///   .offset("1000")
  ///   .offset("1500");
  /// ```
  pub fn offset(mut self, num: &'a str) -> Self {
    self._offset = num.trim();
    self
  }

  /// The order by clause
  pub fn order_by(mut self, column: &'a str) -> Self {
    push_unique(&mut self._order_by, column.trim().to_owned());
    self
  }

  /// Prints the current state of the SelectBuilder into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let raw_query = "select * from users u inner join address addr on u.login = addr.owner_login";
  /// let select_query = SelectBuilder::new()
  ///   .raw(raw_query)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// select * from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{SelectClause, SelectBuilder};
  ///
  /// let raw_join = "inner join address addr on u.login = addr.owner_login";
  /// let select_query = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users u")
  ///   .raw_after(SelectClause::From, raw_join)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users u
  /// inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_after(mut self, clause: SelectClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{SelectClause, SelectBuilder};
  ///
  /// let raw_query = "from users u inner join address addr on u.login = addr.owner_login";
  /// let select_query = SelectBuilder::new()
  ///   .select("*")
  ///   .raw_before(SelectClause::Where, raw_query)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_before(mut self, clause: SelectClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The select clause
  pub fn select(mut self, column: &'a str) -> Self {
    push_unique(&mut self._select, column.trim().to_owned());
    self
  }

  /// The union clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
    self
  }

  /// The where clause
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .from("users")
  ///   .where_clause("login = $1");
  /// ```
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }

  /// The with clause, this method can be used enabling the feature flag `postgresql`
  /// ```
  /// use sql_query_builder::{InsertBuilder, SelectBuilder};
  ///
  /// let logins = SelectBuilder::new().select("login").from("users").where_clause("id in ($1)");
  /// let select = SelectBuilder::new()
  ///   .with("logins", logins)
  ///   .select("name, price")
  ///   .from("orders")
  ///   .where_clause("owner_login in (select * from logins)")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WITH logins AS (
  ///   SELECT login
  ///   FROM users
  ///   WHERE id in ($1)
  /// )
  /// SELECT name, price
  /// FROM orders
  /// WHERE owner_login in (select * from active_users)
  /// ```
  #[cfg(feature = "postgresql")]
  pub fn with(mut self, name: &'a str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim(), std::sync::Arc::new(query)));
    self
  }
}

impl WithQuery for SelectBuilder<'_> {}

impl std::fmt::Display for SelectBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for SelectBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}
