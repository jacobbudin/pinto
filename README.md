# Pinto

[![](https://img.shields.io/crates/v/pinto.svg)][crate]
[![](https://travis-ci.org/jacobbudin/pinto.svg?branch=master)][travis-ci]

Pinto is a small, easy-to-use library for constructing SQL queries programmatically in [Rust](https://www.rust-lang.org).

⚠️ This library does not provide query parameterization. Do not use raw user-supplied data in your queries. If inputs are not properly escaped, your software will be suspectible to [SQL injection](https://en.wikipedia.org/wiki/SQL_injection) attacks.

## Compatibility

The library aims to generate queries compatible with [PostgreSQL](https://www.postgresql.org), [MySQL](https://www.mysql.com), and [SQLite](https://sqlite.org).

## Install

Add [`pinto`](https://crates.io/crates/pinto) as a dependency:

```toml
[dependencies]
pinto = "0.4"
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

See included tests for additional examples.

## Features

### Statements

- `DELETE`
	- `WHERE` clause
- `INSERT`
- `SELECT`
	- Table alias (`AS`)
	- Field selection
	- `JOIN` clause
	- `WHERE` clause
	- `ORDER BY` clause
	- `LIMIT` and `OFFSET` clause
- `UPDATE`
	- `WHERE` clause

## Documentation

- ["First Steps"](https://github.com/jacobbudin/pinto/wiki/First-Steps) (recommended for beginners)
- [API documentation](https://docs.rs/pinto)

## License

MIT

[crate]: https://crates.io/crates/pinto
[travis-ci]: https://travis-ci.org/jacobbudin/pinto
