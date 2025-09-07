CREATE TABLE IF NOT EXISTS users (
    user_id UUID UNIQUE PRIMARY KEY,
    login VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    birth_date DATE NOT NULL,
    gender VARCHAR(6) CHECK (gender IN ('male', 'female')),
    city_id UUID REFERENCES cities(city_id)
)