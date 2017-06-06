# Pinto

[![](https://travis-ci.org/jacobbudin/pinto.svg?branch=master)][travis-ci]

Pinto is a small, easy-to-use library for constructing SQL queries programmatically in [Rust](https://www.rust-lang.org).

## Databases

The library aims to generate queries compatible with [PostgreSQL](https://www.postgresql.org), [MySQL](https://www.mysql.com), and [SQLite](https://sqlite.org).

## Install

Add `pinto` as a dependency:

```toml
[dependencies]
pinto = "0.1"
```

## Example

```rust
let query = query_builder::select("users")
    .fields(&["id", "name"])
    .filter("name = $1")
    .order_by("id", query_builder::Order::Asc)
    .build();
assert_eq!("SELECT id, name FROM users WHERE name = $1 ORDER BY id ASC;", query);
```

## Features

### Statements

- `SELECT`
	- Table alias (`AS`)
	- Field selection
	- `WHERE` clause
	- `ORDER BY` clause
	- `LIMIT` and `OFFSET` clause

## Roadmap

- Table joins

## License

MIT

[travis-ci]: https://travis-ci.org/jacobbudin/pinto
