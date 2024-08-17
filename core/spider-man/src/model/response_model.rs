use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel {
    pub code: i32,
    pub body: Option<String>,
}
