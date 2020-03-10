use graphql_parser::{parse_schema, schema::Document};

pub struct Schema {
    pub document: Document,
}

impl Schema {
    pub fn new(document: Document) -> Self {
        Schema { document }
    }

    pub fn parse(raw: &str) -> Result<Self, std::io::Error> {
        let document = parse_schema(&raw).unwrap();

        let schema = Schema { document };

        Ok(schema)
    }
}
