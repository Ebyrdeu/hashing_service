use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HashedPassword {
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HashedPasswordWithSalt {
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IsEqual {
    pub is_equal: bool,
}

#[derive(Debug, Deserialize)]
pub struct ManualSalt {
    pub password: String,
    pub rounds: usize,
    pub salt: String,
}

#[derive(Debug, Deserialize)]
pub struct AutoSalt {
    pub password: String,
    pub rounds: usize,
}
