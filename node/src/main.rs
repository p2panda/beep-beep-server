use async_ctrlc::CtrlC;
use clap::{App, Arg};
use std::fs;

use beep_beep_graphql::Context;
use beep_beep_http::GraphQLServer;

#[async_std::main]
async fn main() {
    env_logger::init();

    // Setup CLI using Clap, provide general info and capture configuration variables
    let matches = App::new("beep-beep")
        .version("0.1.0")
        .author("pandagang, Inc.")
        .about("Fictional p2p protocol")
        .arg(
            Arg::with_name("postgres-url")
                .takes_value(true)
                .required(true)
                .long("postgres-url")
                .value_name("URL")
                .help("Location of the Postgres database used for storing entities"),
        )
        .arg(
            Arg::with_name("connection-pool-size")
                .long("connection-pool-size")
                .default_value("10")
                .value_name("CONNECTION_POOL_SIZE")
                .help("Limits the number of connections in the store's connection pool"),
        )
        .arg(
            Arg::with_name("http-port")
                .default_value("8000")
                .long("http-port")
                .value_name("PORT")
                .help("Port for the GraphQL HTTP server"),
        )
        .arg(
            Arg::with_name("ws-port")
                .default_value("8001")
                .long("ws-port")
                .value_name("PORT")
                .help("Port for the GraphQL WebSocket server"),
        )
        .arg(
            Arg::with_name("cddl-schema")
                .default_value("schema.cddl")
                .long("cddl-schema")
                .value_name("FILE")
                .help("Path for CDDL schema file"),
        )
        .get_matches();

    // Obtain ports to use for the GraphQL server
    let http_port: u16 = matches
        .value_of("http-port")
        .unwrap()
        .parse()
        .expect("invalid GraphQL HTTP server port");
    let ws_port: u16 = matches
        .value_of("ws-port")
        .unwrap()
        .parse()
        .expect("invalid GraphQL WebSocket server port");

    // Safe to unwrap because a value is required by CLI
    let postgres_url = matches.value_of("postgres-url").unwrap().to_string();

    let pool_size: u32 = matches
        .value_of("connection-pool-size")
        .unwrap()
        .parse()
        .expect("invalid --connection-pool-size value");

    if pool_size <= 1 {
        panic!("--connection-pool-size must be > 1")
    }

    // Get database pool
    let pool = beep_beep_db::get_connection_pool(postgres_url, pool_size);

    // Read CDDL schema
    let cddl_schema = fs::read_to_string(matches.value_of("cddl-schema").unwrap().to_string())
        .expect("could not read cddl schema file");

    let context = Context::new(pool, cddl_schema);

    // Start HTTP server with GraphQL interface
    let graphql_server = GraphQLServer::new(context);
    graphql_server.serve(http_port, ws_port);

    CtrlC::new().unwrap().await;
}
