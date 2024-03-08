use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE, HeaderMap};
use serde_derive::Serialize;
use crate::{message::Message, prelude::*};

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

    pub async fn post(&self, url: String, model: String, messages: Vec<Message>, api_key: String) -> Result<String> {
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("stream", HeaderValue::from_static("false"));
        headers.insert("Authorization", HeaderValue::from_str(api_key.as_str()).unwrap());

        let body = TransportBody::new(messages, model.to_string());
        let body = serde_json::to_string(&body).unwrap();

        // TODO build body - messages, stream:false and model name
        let response = self.client
        .post(url)
        .headers(headers)
        .body(body)
        // TODO serialise the message vec
        .send()
        .await
        .unwrap();

        let text = response.text().await.unwrap();
        
        Ok(text)
    }
}