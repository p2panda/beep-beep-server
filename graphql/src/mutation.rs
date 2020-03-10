use bamboo_core::entry::decode;
use bamboo_core::{lipmaa, verify};
use bamboo_core::{YamfHash, YamfSignatory};
use juniper::FieldResult;

use beep_beep_db::models::messages;

use crate::context::Context;

pub struct Mutation;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Bamboo entry")]
struct Message {
    encoded_entry: String,
    encoded_payload: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "Bamboo entry")]
struct NewMessage {
    encoded_entry: String,
    encoded_payload: String,
}

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn post_message(context: &Context, message: NewMessage) -> FieldResult<Message> {
        // Get connection from database pool
        let connection = context.pool.get().unwrap();

        // Decode payload and entry from request
        let entry_bytes = hex::decode(&message.encoded_entry).map_err(|e| e.to_string())?;
        let payload_bytes = hex::decode(&message.encoded_payload).map_err(|e| e.to_string())?;

        // Decode bamboo entry
        let entry = decode(&entry_bytes).map_err(|e| "Can't decode message")?;

        // Get the author public address
        let author = match &entry.author {
            YamfSignatory::Ed25519(vec, _) => hex::encode(vec),
        };

        // Get payload hash
        let payload_hash = match &entry.payload_hash {
            YamfHash::Blake2b(vec) => hex::encode(vec),
        };

        // Get related bamboo entries
        let previous_msg = messages::get_message(
            &connection,
            author.clone(),
            entry.seq_num as i64 - 1,
            entry.log_id as i64,
        )
        .map_err(|e| "Error getting message")?
        .map(|msg| msg.entry_bytes)
        .map(|msg| hex::decode(msg).unwrap());

        let lipmaa_msg = messages::get_message(
            &connection,
            author.clone(),
            lipmaa(entry.seq_num) as i64,
            entry.log_id as i64,
        )
        .map_err(|e| "Error getting limpmaa message")?
        .map(|msg| msg.entry_bytes)
        .map(|msg| hex::decode(msg).unwrap());

        // Verify bamboo entry integrity
        let result = verify(
            &entry_bytes,
            Some(&payload_bytes),
            lipmaa_msg.as_deref(),
            previous_msg.as_deref(),
        )
        .map_err(|e| "Can't verify message")?;

        let new_message = messages::NewMessage {
            author: &author,
            entry_bytes: &message.encoded_entry,
            log_id: entry.log_id as i64,
            payload_bytes: &message.encoded_payload,
            payload_hash: &payload_hash,
            seqnum: entry.seq_num as i64,
        };

        // This throws an error when we try to create
        // the same message again (via PRIMARY KEY)
        messages::insert_message(&connection, &new_message)
            .map_err(|e| "Error creating message")?;

        Ok(Message {
            encoded_entry: message.encoded_entry,
            encoded_payload: message.encoded_payload,
        })
    }
}
