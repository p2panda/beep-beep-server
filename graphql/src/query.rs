use bamboo_core::entry::decode;
use bamboo_core::lipmaa;
use juniper::FieldResult;

use beep_beep_db::models::messages;

use crate::context::Context;

pub struct Query;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Arguments required to sign a new bamboo message")]
struct NextMessageArguments {
    encoded_entry_backlink: Option<String>,
    encoded_entry_lipmaa: Option<String>,
    last_seq_num: i32,
}

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn nextMessageArguments(
        context: &Context,
        author: String,
        log_id: i32,
    ) -> FieldResult<NextMessageArguments> {
        // Get connection from database pool
        let connection = context.pool.get().unwrap();

        // Get related bamboo entries
        let previous_msg_hex =
            messages::get_last_message(&connection, author.clone(), log_id as i64)
                .map_err(|e| "Error getting last message")?
                .map(|msg| msg.entry_bytes);

        // No backlink found, this is our first post
        if previous_msg_hex.is_none() {
            return Ok(NextMessageArguments {
                encoded_entry_lipmaa: None,
                encoded_entry_backlink: None,
                last_seq_num: 0,
            });
        }

        let previous_msg_hex = previous_msg_hex.unwrap();

        let previous_msg = hex::decode(&previous_msg_hex).map_err(|e| e.to_string())?;

        let previous_entry = decode(&previous_msg).map_err(|e| "Can't decode message")?;

        let last_seq_num = previous_entry.seq_num;

        let lipmaa_msg_hex = messages::get_message(
            &connection,
            author.clone(),
            lipmaa(last_seq_num + 1) as i64,
            log_id as i64,
        )
        .map_err(|e| "Error getting limpmaa message")?
        .map(|msg| msg.entry_bytes);

        Ok(NextMessageArguments {
            encoded_entry_lipmaa: lipmaa_msg_hex,
            encoded_entry_backlink: Some(previous_msg_hex),
            last_seq_num: last_seq_num as i32,
        })
    }
}
