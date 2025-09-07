CREATE TABLE IF NOT EXISTS user_credentials (
    user_id UUID REFERENCES users(user_id),
    password VARCHAR NOT NULL
)