# beep-beep-server

## Requirements

* Rust nightly
* PostgreSQL database

## Setup

Install requirements for your system: Rust and Postgres

For mac, install [Postgres.app](https://postgresapp.com/de/) and then
```
$ brew install rustup
```

Install rust and diesel, which is required for running database migrations
```
$ rustup-init
$ rustup default nightly
$ cargo install diesel_cli --no-default-features --features postgres
```

Create postgres database
```
$ createdb beep-beep-development
```

Init database

```
$ cd db
$ diesel migration run --database-url=postgres://postgres:postgres@localhost/beep-beep-development

```

## Usage

```
RUST_LOG=info cargo run -- --postgres-url=postgres://postgres:postgres@localhost/beep-beep-development
```
