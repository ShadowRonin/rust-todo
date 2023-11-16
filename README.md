Basic todo app using HTMX and rust.

Rust stack based on [NoBoilerplate](https://www.youtube.com/@NoBoilerplate)'s [stack](https://www.youtube.com/watch?v=pocWrUj68tU).

## Libraries and the like used
- [Tokio](https://tokio.rs/); runtime for async
- [Eyre](https://docs.rs/eyre/latest/eyre/); unified Errors/Result types
- [Serde](https://serde.rs/); serialize/deserialize

- [Color-eyre](https://docs.rs/color-eyre/latest/color_eyre/); colorful errors
- [Bacon](https://github.com/Canop/bacon); background code checker?

---below still need to be added---

- [sqlx](https://docs.rs/sqlx/latest/sqlx/); sql integration with compiler time type checking
- [Poem-openapi](https://docs.rs/poem-openapi/latest/poem_openapi/); open api
- [Chrono](https://crates.io/crates/chrono); Datetime

- MySql
- 

## commands?
Lint
`
cargo clippy --fix -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \
-W clippy::expect_used
`
Set up defaults so we dont have to specify all these flags?


Bacon, background code checker, keep open in a terminal
`bacon`

Start db `docker compose up`
Note: make sure docker desktop is running on windows first

## Resources
- NoBoilerplate
    - [build a lightsaber](https://github.com/0atman/noboilerplate/blob/main/scripts/06-build-your-rust-lightsaber.md)
    - [stack](https://github.com/0atman/noboilerplate/blob/main/scripts/30-poem.md)
    - Let's Get Rusty's [serde explanation](https://github.com/letsgetrusty/json_parsing_example/blob/master/src/main.rs)