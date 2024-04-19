use serde_json::Value;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub _id: UserID, //username
    pub pwd: String,
    pub key: String
}

pub type UserID = String;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserCredential {
    pub _id: UserID, //username
    pub pwd: String
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Bin {
    pub json: Value,
    pub owner: UserID
}

impl Bin {
    pub fn new(json: Value, uid: UserID) -> Self {{
        Self {
            json,
            owner: uid
        }
    }}
}