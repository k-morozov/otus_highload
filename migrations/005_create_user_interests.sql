CREATE TABLE IF NOT EXISTS user_interests (
    user_id UUID REFERENCES users(user_id),
    interest_id UUID REFERENCES interests(interest_id)
)