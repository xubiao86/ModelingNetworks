use std::collections::HashMap;

pub mod http_header;
pub mod http_method;
pub mod request_model;
pub mod response_model;

pub trait RequestTrait: Send + Sync {
    fn http_url(&self) -> String;

    fn http_path(&self) -> Option<String> {
        None
    }

    fn http_method(&self) -> String;

    fn http_header(&self) -> HashMap<String, String> {
        HashMap::default()
    }

    fn http_query(&self) -> HashMap<String, String> {
        HashMap::default()
    }

    fn http_body(&self) -> HashMap<String, String> {
        HashMap::default()
    }
}
