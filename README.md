# beep-beep-server

## Requirements

* Rust nightly
* PostgreSQL database

## Setup

Run

```
$ cargo install diesel_cli
$ cd db
$ diesel migration run

```

## Usage

```
RUST_LOG=info cargo run -- --postgres-url=postgres://postgres:postgres@localhost/beep-beep-development
```
