# beep-beep-server

This is an experimental implementation of a p2p server based on the proof-of-concept introduced with [beep-beep](https://github.com/p2panda/beep-beep). It should serve as a playground to play with more concrete implementation details. :goggles: :lab_coat:

## Requirements

* Rust [nightly](https://github.com/rust-lang/rustup#working-with-nightly-rust)
* PostgreSQL database
* Diesel [CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)

## Setup

1. Make sure you have the above mentioned requirements up and running.

2. Create postgres database:

  ```
  $ createdb beep-beep-development
  ```

3. Run database migration in `db` folder:

  ```
  $ cd db
  $ diesel migration run --database-url=postgres://postgres:postgres@localhost/beep-beep-development
  ```

## Usage

```
// Run the server on default port 8000
RUST_LOG=info cargo run -- --postgres-url=postgres://postgres:postgres@localhost/beep-beep-development
```

## Further links

* https://github.com/p2panda/beep-beep-client/
* https://github.com/p2panda/design-document

## License

GNU Affero General Public License v3.0 `AGPL-3.0`
