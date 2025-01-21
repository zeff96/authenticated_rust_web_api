-- Add up migration script here

CREATE TABLE IF NOT EXISTS posts(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    CONSTRAINT posts_fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE INDEX posts_user_id_idx ON posts(user_id);