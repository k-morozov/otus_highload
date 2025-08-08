pub const CREATE_TABLES: [&str; 5] = [
    r#"
    CREATE TABLE IF NOT EXISTS cities (
        city_id UUID UNIQUE PRIMARY KEY,
        city_name VARCHAR UNIQUE NOT NULL
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS interests (
        interest_id UUID UNIQUE PRIMARY KEY,
        interest_name VARCHAR UNIQUE NOT NULL
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS users (
        user_id UUID UNIQUE PRIMARY KEY,
        login VARCHAR UNIQUE NOT NULL,
        name VARCHAR NOT NULL,
        surname VARCHAR NOT NULL,
        birth_date DATE NOT NULL,
        gender VARCHAR(6) CHECK (gender IN ('male', 'female')),
        city_id UUID REFERENCES cities(city_id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS user_credentials (
        user_id UUID REFERENCES users(user_id),
        password VARCHAR NOT NULL
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS user_interests (
        user_id UUID REFERENCES users(user_id),
        interest_id UUID REFERENCES interests(interest_id)
    )
    "#,
];
