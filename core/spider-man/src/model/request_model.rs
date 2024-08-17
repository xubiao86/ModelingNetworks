use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RequestTrait;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestModel {
    pub url: String,
    pub path: Option<String>,
    pub method: String,
    pub header: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

impl RequestTrait for RequestModel {
    fn http_url(&self) -> String {
        self.url.clone()
    }

    fn http_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn http_method(&self) -> String {
        self.method.clone()
    }

    fn http_header(&self) -> HashMap<String, String> {
        self.header.clone()
    }

    fn http_query(&self) -> HashMap<String, String> {
        self.query.clone()
    }

    fn http_body(&self) -> HashMap<String, String> {
        self.body.clone()
    }
}
