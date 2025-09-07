 CREATE TABLE IF NOT EXISTS interests (
    interest_id UUID UNIQUE PRIMARY KEY,
    interest_name VARCHAR UNIQUE NOT NULL
)