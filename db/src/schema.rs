table! {
    messages (author, log_id, seqnum) {
        author -> Text,
        entry_bytes -> Text,
        log_id -> Int8,
        payload_bytes -> Nullable<Text>,
        payload_hash -> Text,
        seqnum -> Int8,
    }
}
