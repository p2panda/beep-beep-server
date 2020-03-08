use iron::prelude::*;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use log::info;
use mount::Mount;
use std::net::SocketAddr;

use beep_beep_graphql::{Query, Mutation, Context};

pub struct GraphQLServer {
    context: Context,
}

impl GraphQLServer {
    pub fn new(context: Context) -> Self {
        GraphQLServer { context }
    }

    pub fn serve(self, port: u16, _ws_port: u16) {
        info!("Starting GraphQL HTTP server at localhost:{}", port);

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        // GraphQL handler for incoming queries
        let graphql_endpoint =
            GraphQLHandler::new(move |_| Ok(self.context.clone()), Query, Mutation);

        // Serve an HTML page to play with GraphQL queries
        let graphiql_endpoint = GraphiQLHandler::new("/graphql");

        // Mount routes and start HTTP server
        let mut mount = Mount::new();
        mount.mount("/", graphiql_endpoint);
        mount.mount("/graphql", graphql_endpoint);

        let chain = Chain::new(mount);
        // @TODO: Handle bind errors
        Iron::new(chain).http(addr).unwrap();
    }
}
