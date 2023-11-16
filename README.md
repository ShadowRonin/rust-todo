# Overview

Tiny todo app to practice rust.

Rust stack based on [NoBoilerplate](https://www.youtube.com/@NoBoilerplate)'s [stack](https://www.youtube.com/watch?v=pocWrUj68tU). Will server a webapp via htmx.

## Todo
- add UI via [htmx](https://htmx.org/)
- Setup clippy defaults or script
    - that way dont have to specify all the flags
- improve error handling? everything should use Eyre

## Crates and tools
- [Tokio](https://tokio.rs/); async runtime
- [Eyre](https://docs.rs/eyre/latest/eyre/); unified Errors/Result types
- [Color-eyre](https://docs.rs/color-eyre/latest/color_eyre/); colorful errors
- [Serde](https://serde.rs/); serialize/deserialize
- [Poem](https://github.com/poem-web/poem); web framework
- [Poem-openapi](https://docs.rs/poem-openapi/latest/poem_openapi/); open api
- [sqlx](https://docs.rs/sqlx/latest/sqlx/); sql integration with compiler time type checking
- [Chrono](https://crates.io/crates/chrono); Datetime
- [Bacon](https://github.com/Canop/bacon); background code checker, keep open in another terminal
- MySql via Docker Compose, using Beekeper Studio for a client

## Commands

- run w/ watch: `cargo watch -x run`
- Lint: `
cargo clippy --fix -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \
-W clippy::expect_used
`
- Bacon, background code checker, keep open in a terminal
`bacon`
- DB
    - Start: `docker compose up`
        - Note: make sure docker desktop is running on windows first
    - add migration: `sqlx migrate add <name>`
    - migrate: `sqlx migrate run`
    - [sqlx cli cheat sheet](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

## Resources
- NoBoilerplate
    - [build a lightsaber](https://github.com/0atman/noboilerplate/blob/main/scripts/06-build-your-rust-lightsaber.md)
    - [stack](https://github.com/0atman/noboilerplate/blob/main/scripts/30-poem.md)
- Let's Get Rusty's [serde explanation](https://github.com/letsgetrusty/json_parsing_example/blob/master/src/main.rs)