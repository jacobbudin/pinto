/// A module to create SQL-based queries programmatically.
pub mod query_builder {
    use std::collections::HashMap;

    /// `DELETE`
    pub struct Delete<'a> {
        table: &'a str,
        conditions: Option<Vec<&'a str>>,
    }

    /// `INSERT`
    pub struct Insert<'a> {
        table: &'a str,
        values: HashMap<&'a str, &'a str>,
    }

    /// `SELECT`
    pub struct Select<'a> {
        table: &'a str,
        aliases: Option<HashMap<&'a str, &'a str>>,
        fields: Option<Vec<&'a str>>,
        order: Option<Vec<(&'a str, Order)>>,
        conditions: Option<Vec<&'a str>>,
        limit: usize,
        offset: usize,
    }

    /// `UPDATE`
    pub struct Update<'a> {
        table: &'a str,
        values: HashMap<&'a str, &'a str>,
        conditions: Option<Vec<&'a str>>,
    }

    /// The direction of an `ORDER` clause's expression
    pub enum Order { Asc, Desc }

    /// Combine a vector of `String`s, with the `sep` `str` between each value
    fn join(v: &Vec<&str>, sep: &str) -> String {
        let mut s = String::new();
        let last_i = v.len() - 1;
        for (i, val) in v.iter().enumerate() {
            s += val;
            if i != last_i {
                s += sep;
            }
        }
        s
    }

    impl<'a> Delete<'a> {
        /// Construct a new `DELETE` query builder
        pub fn new(table: &'a str) -> Self {
            let query_builder = Delete {
                table: table,
                conditions: None,
            };

            query_builder
        }

        /// Filter result set based on conditions (`WHERE` clause)
        pub fn filter(&mut self, expr: &'a str) -> &mut Self {
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            match self.conditions {
                Some(ref mut current_conditions) => {
                    current_conditions.push(expr);
                },
                None => unreachable!(),
            }

            self
        }


        /// Generate SQL query (`String`) from subsequent method calls
        pub fn build(&self) -> String {
            let mut query = String::from("DELETE FROM ");
            query += self.table;

            if let Some(ref conditions) = self.conditions {
                query += " WHERE ";
                query += join(conditions, " AND ").as_str();
            }

            query += ";";
            query
        }
    }

    impl<'a> Insert<'a> {
        /// Construct a new `INSERT` query builder
        pub fn new(table: &'a str) -> Self {
            let query_builder = Insert {
                table: table,
                values: HashMap::new(),
            };

            query_builder
        }

        /// Set a field value
        pub fn set(&mut self, field: &'a str, value: &'a str) -> &mut Self {
            let _ = self.values.insert(field, value);
            self
        }

        /// Generate SQL query (`String`) from subsequent method calls
        pub fn build(&self) -> String {
            let mut query = String::from("INSERT INTO ");
            query += self.table;

            let mut columns: Vec<&str> = Vec::new();
            let mut values: Vec<&str> = Vec::new();

            for (field, value) in self.values.iter() {
                columns.push(field);
                values.push(value);
            }

            query += " (";
            query += join(&columns, ", ").as_str();
            query += ") VALUES (";
            query += join(&values, ", ").as_str();
            query += ");";
            query
        }
    }

    impl<'a> Select<'a> {
        /// Construct a new `SELECT` query builder
        pub fn new(table: &'a str) -> Self {
            let query_builder = Select {
                table: table,
                aliases: None,
                fields: None,
                order: None,
                conditions: None,
                limit: 0usize,
                offset: 0usize,
            };

            query_builder
        }

        /// Set a table alias (`AS`)
        pub fn alias(&mut self, table: &'a str, alias: &'a str) -> &mut Self {
            if self.aliases.is_none() {
                self.aliases = Some(HashMap::new());
            }

            match self.aliases {
                Some(ref mut aliases) => {
                    aliases.insert(table, alias);
                },
                None => unreachable!(),
            }

            self
        }

        /// Specify desired table fields in result set
        pub fn fields(&mut self, fields: &[&'a str]) -> &mut Self {
            if self.fields.is_none() {
                self.fields = Some(Vec::new());
            }

            match self.fields {
                Some(ref mut current_fields) => {
                    for field in fields {
                        current_fields.push(field);
                    }
                },
                None => unreachable!(),
            }

            self 
        }

        /// Filter result set based on conditions (`WHERE` clause)
        pub fn filter(&mut self, expr: &'a str) -> &mut Self {
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            match self.conditions {
                Some(ref mut current_conditions) => {
                    current_conditions.push(expr);
                },
                None => unreachable!(),
            }

            self
        }

        /// Order result set based on the value of an expression (`ORDER BY` clause)
        pub fn order_by(&mut self, expr: &'a str, direction: Order) -> &mut Self {
            if self.order.is_none() {
                self.order = Some(Vec::new());
            }

            match self.order {
                Some(ref mut current_order) => {
                    let order = (expr, direction);
                    current_order.push(order);
                },
                None => unreachable!(),
            }

            self 
        }

        #[allow(unused_variables)]
        pub fn inner_join(&mut self, table: &str, on_left: &str, on_right: &str) -> &mut Self {
            self
        }

        #[allow(unused_variables)]
        pub fn left_join(&mut self, table: &str, on_left: &str, on_right: &str) -> &mut Self {
            self
        }

        /// Limit number of rows in result set (`LIMIT`)
        pub fn limit(&mut self, limit: usize) -> &mut Self {
            self.limit = limit;
            self
        }

        /// Offset number of rows in result set (`OFFSET`)
        pub fn offset(&mut self, offset: usize) -> &mut Self {
            self.offset = offset;
            self
        }

        /// Generate SQL query (`String`) from subsequent method calls
        pub fn build(&self) -> String {
            let mut query = String::from("SELECT ");

            match self.fields {
                Some(ref fields) => {
                    query += join(fields, ", ").as_str();
                },
                None => query += "*",
            }

            query += " FROM ";
            query += self.table;

            if let Some(ref aliases) = self.aliases {
                if let Some(ref alias) = aliases.get(self.table) {
                    query += " AS ";
                    query += *alias;
                }
            }

            if let Some(ref conditions) = self.conditions {
                query += " WHERE ";
                query += join(conditions, " AND ").as_str();
            }

            if let Some(ref order) = self.order {
                query += " ORDER BY ";
                for item in order.iter() {
                    let (ref expr, ref dir) = *item;
                    query += expr;
                    match *dir {
                        Order::Asc => query += " ASC",
                        Order::Desc => query += " DESC",
                    }
                }
            }

            if self.limit != 0 {
                query += " LIMIT ";
                query += self.limit.to_string().as_str();
            }

            if self.offset != 0 {
                query += ", ";
                query += self.offset.to_string().as_str();
            }

            query += ";";
            query
        }
    }

    impl<'a> Update<'a> {
        /// Construct a new `UPDATE` query builder
        pub fn new(table: &'a str) -> Self {
            let query_builder = Update {
                table: table,
                values: HashMap::new(),
                conditions: None,
            };

            query_builder
        }

        /// Set a field value
        pub fn set(&mut self, field: &'a str, value: &'a str) -> &mut Self {
            let _ = self.values.insert(field, value);
            self
        }

        /// Filter result set based on conditions (`WHERE` clause)
        pub fn filter(&mut self, expr: &'a str) -> &mut Self {
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            match self.conditions {
                Some(ref mut current_conditions) => {
                    current_conditions.push(expr);
                },
                None => unreachable!(),
            }

            self
        }

        /// Generate SQL query (`String`) from subsequent method calls
        pub fn build(&self) -> String {
            let mut query = String::from("UPDATE ");
            query += self.table;

            let assignments: Vec<String>;
            assignments = self.values.iter().map(|(&field, &value)| {
                let mut assignment = String::from(field);
                assignment += " = ";
                assignment += value;
                assignment
            }).collect();

            query += " SET ";
            query += assignments.join(" AND ").as_str();

            if let Some(ref conditions) = self.conditions {
                query += " WHERE ";
                query += join(conditions, " AND ").as_str();
            }

            query += ";";
            query
        }
    }

    /// Helper function to construct new `DELETE` query builder
    pub fn delete(table: &str) -> Delete {
        Delete::new(table)
    }

    /// Helper function to construct new `INSERT` query builder
    pub fn insert(table: &str) -> Insert {
        Insert::new(table)
    }

    /// Helper function to construct new `SELECT` query builder
    pub fn select(table: &str) -> Select {
        Select::new(table)
    }

    /// Helper function to construct new `UPDATE` query builder
    pub fn update(table: &str) -> Update {
        Update::new(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_query() {
        let query = query_builder::delete("users")
            .build();
        assert_eq!("DELETE FROM users;", query);
    }

    #[test]
    fn test_delete_query_with_conditions() {
        let query = query_builder::delete("users")
            .filter("name = $1")
            .filter("karma <= $2")
            .build();
        assert_eq!("DELETE FROM users WHERE name = $1 AND karma <= $2;", query);
    }

    #[test]
    fn test_insert_query() {
        let query = query_builder::insert("users")
            .set("name", "$1")
            .set("karma", "$2")
            .build();
        let possibility1 = "INSERT INTO users (name, karma) VALUES ($1, $2);" == query;
        let possibility2 = "INSERT INTO users (karma, name) VALUES ($2, $1);" == query;
        assert!(possibility1 || possibility2);
    }

    #[test]
    fn test_select_query() {
        let query = query_builder::select("users")
            .build();
        assert_eq!("SELECT * FROM users;", query);
    }

    #[test]
    fn test_select_query_with_fields() {
        let query = query_builder::select("users")
            .fields(&["id", "name"])
            .build();
        assert_eq!("SELECT id, name FROM users;", query);
    }

    #[test]
    fn test_select_query_with_alias() {
        let query = query_builder::select("users")
            .alias("users", "u")
            .fields(&["id", "name"])
            .build();
        assert_eq!("SELECT id, name FROM users AS u;", query);
    }
 
    #[test]
    fn test_select_query_with_limit() {
        let query = query_builder::select("users")
            .fields(&["id", "name"])
            .limit(15)
            .build();
        assert_eq!("SELECT id, name FROM users LIMIT 15;", query);
    }
 
    #[test]
    fn test_select_query_with_offset() {
        let query = query_builder::select("users")
            .fields(&["id", "name"])
            .limit(15)
            .offset(30)
            .build();
        assert_eq!("SELECT id, name FROM users LIMIT 15, 30;", query);
    }
 
    #[test]
    fn test_select_query_with_conditions() {
        let query = query_builder::select("users")
            .fields(&["id", "name"])
            .filter("id = $1")
            .filter("name = $2")
            .build();
        assert_eq!("SELECT id, name FROM users WHERE id = $1 AND name = $2;", query);
    }
 
    #[test]
    fn test_select_query_with_order() {
        let query = query_builder::select("users")
            .fields(&["id", "name"])
            .filter("name = $1")
            .order_by("id", query_builder::Order::Asc)
            .build();
        assert_eq!("SELECT id, name FROM users WHERE name = $1 ORDER BY id ASC;", query);
    }

    #[test]
    fn test_update_query() {
        let query = query_builder::update("users")
            .set("karma", "0")
            .set("last_login", "1970-01-01")
            .build();
        let possibility1 = "UPDATE users SET karma = 0 AND last_login = 1970-01-01;" == query;
        let possibility2 = "UPDATE users SET last_login = 1970-01-01 AND karma = 0;" == query;
        assert!(possibility1 || possibility2);
    }

    #[test]
    fn test_update_query_with_conditions() {
        let query = query_builder::update("users")
            .set("karma", "0")
            .filter("name = $1")
            .filter("last_login < $2")
            .build();
        assert_eq!("UPDATE users SET karma = 0 WHERE name = $1 AND last_login < $2;", query);
    }
}
