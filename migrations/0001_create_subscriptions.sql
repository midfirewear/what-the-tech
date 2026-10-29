CREATE TABLE IF NOT EXISTS subscriptions (
    channel_id  TEXT    NOT NULL,
    topic       TEXT    NOT NULL,
    added_by    TEXT    NOT NULL,
    created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
    PRIMARY KEY (channel_id, topic)
);
