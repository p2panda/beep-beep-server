use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::messages;
use crate::schema::messages::dsl;

#[derive(Queryable)]
pub struct Message {
    pub author: String,
    pub entry_bytes: String,
    pub log_id: i64,
    pub payload_bytes: Option<String>,
    pub payload_hash: String,
    pub seqnum: i64,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub author: &'a str,
    pub entry_bytes: &'a str,
    pub log_id: i64,
    pub payload_bytes: &'a str,
    pub payload_hash: &'a str,
    pub seqnum: i64,
}

pub fn get_last_message(
    connection: &PgConnection,
    author: String,
    log_id: i64,
) -> Result<Option<Message>, diesel::result::Error> {
    dsl::messages
        .order_by(dsl::seqnum.desc())
        .filter(dsl::author.eq(author).and(dsl::log_id.eq(log_id)))
        .first::<Message>(connection)
        .optional()
}

pub fn get_message(
    connection: &PgConnection,
    author: String,
    seqnum: i64,
    log_id: i64,
) -> Result<Option<Message>, diesel::result::Error> {
    dsl::messages
        .filter(
            dsl::seqnum
                .eq(seqnum)
                .and(dsl::author.eq(author).and(dsl::log_id.eq(log_id))),
        )
        .first::<Message>(connection)
        .optional()
}

pub fn insert_message(
    connection: &PgConnection,
    new_message: &NewMessage,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(messages::table)
        .values(new_message)
        .execute(connection)
}
