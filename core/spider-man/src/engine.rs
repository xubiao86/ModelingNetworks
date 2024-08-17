use reqwest::{Client, Error};

use crate::model::{
    http_header::HTTPHeader, http_method::HttpMethod, response_model::ResponseModel, RequestTrait,
};

pub async fn request<T: RequestTrait>(model: T) -> Result<ResponseModel, Error> {
    let client = reqwest::Client::new();
    match model.http_method().into() {
        HttpMethod::GET => get(client, model).await,
        HttpMethod::POST => post(client, model).await,
        HttpMethod::PUT => put(client, model).await,
        HttpMethod::DELETE => delete(client, model).await,
        HttpMethod::HEAD => head(client, model).await,
        HttpMethod::UNKNOWN => panic!("unkonown http method, not support"),
    }
}

async fn get<T: RequestTrait>(client: Client, model: T) -> Result<ResponseModel, Error> {
    let _ = model;
    let header: HTTPHeader = model.http_header().into();
    let response = client
        .get(format!(
            "{}{}",
            model.http_url(),
            model.http_path().unwrap_or_default()
        ))
        .headers(header.inner)
        .query(&model.http_query())
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    Ok(ResponseModel {
        code: status.as_u16().into(),
        body: Some(body),
    })
}

async fn post<T: RequestTrait>(client: Client, model: T) -> Result<ResponseModel, Error> {
    let header: HTTPHeader = model.http_header().into();
    let response = client
        .post(format!(
            "{}{}",
            model.http_url(),
            model.http_path().unwrap_or_default()
        ))
        .headers(header.inner)
        .json(&model.http_body())
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    Ok(ResponseModel {
        code: status.as_u16().into(),
        body: Some(body),
    })
}

async fn put<T: RequestTrait>(client: Client, model: T) -> Result<ResponseModel, Error> {
    let header: HTTPHeader = model.http_header().into();
    let response = client
        .put(format!(
            "{}{}",
            model.http_url(),
            model.http_path().unwrap_or_default()
        ))
        .headers(header.inner)
        .json(&model.http_body())
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    Ok(ResponseModel {
        code: status.as_u16().into(),
        body: Some(body),
    })
}

async fn delete<T: RequestTrait>(client: Client, model: T) -> Result<ResponseModel, Error> {
    let header: HTTPHeader = model.http_header().into();
    let response = client
        .delete(format!(
            "{}{}",
            model.http_url(),
            model.http_path().unwrap_or_default()
        ))
        .headers(header.inner)
        .json(&model.http_body())
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    Ok(ResponseModel {
        code: status.as_u16().into(),
        body: Some(body),
    })
}

async fn head<T: RequestTrait>(client: Client, model: T) -> Result<ResponseModel, Error> {
    let header: HTTPHeader = model.http_header().into();
    let response = client
        .head(format!(
            "{}{}",
            model.http_url(),
            model.http_path().unwrap_or_default()
        ))
        .headers(header.inner)
        .json(&model.http_body())
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    Ok(ResponseModel {
        code: status.as_u16().into(),
        body: Some(body),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{engine::request, model::RequestTrait};

    struct GetRequestModel;

    struct PostRequestModel;

    impl RequestTrait for GetRequestModel {
        fn http_url(&self) -> String {
            "https://jsonplaceholder.typicode.com".to_string()
        }

        fn http_method(&self) -> String {
            "GET".to_string()
        }

        fn http_path(&self) -> Option<String> {
            Some("/posts/1".to_string())
        }
    }

    impl RequestTrait for PostRequestModel {
        fn http_url(&self) -> String {
            "https://jsonplaceholder.typicode.com".to_string()
        }

        fn http_method(&self) -> String {
            "POST".to_string()
        }

        fn http_path(&self) -> Option<String> {
            Some("/posts".to_string())
        }

        fn http_header(&self) -> HashMap<String, String> {
            let mut map = HashMap::new();
            map.insert(
                "Content-type".to_string(),
                "application/json; charset=UTF-8".to_string(),
            );
            map
        }

        fn http_body(&self) -> HashMap<String, String> {
            let mut map = HashMap::new();
            map.insert("title".to_string(), "foo".to_string());
            map.insert("body".to_string(), "bar".to_string());
            map.insert("userId".to_string(), "1".to_string());
            map
        }
    }

    #[tokio::test]
    async fn test_get() {
        let model = GetRequestModel {};
        let r = request(model).await;
        println!("res is {r:?}");
    }

    #[tokio::test]
    async fn test_post() {
        let model = PostRequestModel {};
        let r = request(model).await;
        println!("res is {r:?}");
    }
}
