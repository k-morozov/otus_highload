use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginRequestBody {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginResponseBody {
    pub token: String,
}
