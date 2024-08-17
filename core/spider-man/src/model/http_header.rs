use std::{collections::HashMap, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName};

pub struct HTTPHeader {
    pub inner: HeaderMap,
}

impl From<HashMap<String, String>> for HTTPHeader {
    fn from(value: HashMap<String, String>) -> Self {
        let mut header = HeaderMap::new();
        for (k, v) in value {
            header.insert(
                HeaderName::from_str(k.as_str()).unwrap(),
                v.parse().unwrap(),
            );
        }
        HTTPHeader { inner: header }
    }
}
