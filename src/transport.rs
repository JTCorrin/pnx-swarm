use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE, HeaderMap};
use serde_derive::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::message::Message;

#[derive(Debug, Serialize)]
struct TransportBody {
    messages: Vec<Message>,
    stream: bool,
    model: String,
}

impl TransportBody {
    pub fn new(messages: Vec<Message>, model: String) -> Self {
        TransportBody {
            messages,
            model,
            stream: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transport {
    client: reqwest::Client,
}

impl Transport {
    pub fn new() -> Self {
        Transport {
            client: reqwest::Client::new(),
        }
    }

    pub async fn post<T: DeserializeOwned>(
        &self, 
        url: String, 
        model: String, 
        messages: Vec<Message>, 
        api_key: String
    ) -> Result<T, reqwest::Error> {
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(api_key.as_str()).unwrap());

        let body = TransportBody::new(messages, model.to_string());

        let response = self.client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .json::<T>()
        .await?;


        Ok(response)
    }
}

#[cfg(test)]
#[path = "./_tests/transport.rs"]
mod tests;
