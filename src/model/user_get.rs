use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGetRequestBody {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserGetResponseBody {
    pub name: String,
    pub surname: String,
}
