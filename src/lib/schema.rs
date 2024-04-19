use serde_json::Number;
use std::collections::HashMap;

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

//Deserialize dynamic JSON
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
