CREATE TABLE IF NOT EXISTS messages (
  PRIMARY KEY(author, log_id, seqnum),
  author TEXT NOT NULL,
  entry_bytes TEXT NOT NULL,
  log_id BIGINT NOT NULL,
  payload_bytes TEXT,
  payload_hash TEXT NOT NULL,
  seqnum BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS messages_author_index ON messages(author);
CREATE INDEX IF NOT EXISTS messages_author_log_id_index ON messages(author, log_id);
CREATE INDEX IF NOT EXISTS messages_author_seqnum_index ON messages(author, seqnum);
