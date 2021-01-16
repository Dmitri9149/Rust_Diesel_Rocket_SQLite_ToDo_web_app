-- Your SQL goes here
CREATE TABLE task (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    done TEXT NOT NULL DEFAULT 'pending',
    PRIMARY KEY (id)
);
