# contactlist-rust
ContactList is a self-hosted web-based contact manager that focuses on simplicity. This repository is for ContactList written in Rust.

## Setup
ContactList uses Diesel for database management. Currently, just SQLite3 is supported for the database. When needed, support for other databases would be implemented.

WARNING: do not use cargo run or cargo build unless you know what you are doing; the "runner" script handles building and running

### Debian/Ubuntu
The Diesel CLI is used by the "runner" shell script for database setup.

```sh
sudo apt install libsqlite3-dev
cargo install diesel_cli --no-default-features --features
```

To-Do: NPM setup and node module installation

## Configuration
The default configuration is intentionally basic. The software is meant to be configured to the needs of the user.

