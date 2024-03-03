use reqwest;
use crate::{message::Message, prelude::*};

pub struct Transport {
    client: reqwest::Client,
}

impl Transport {
    pub fn new() -> Self {
        Transport {
            client: reqwest::Client::new(),
        }
    }

    pub async fn post(&self, url: &str, messages: Vec<Message>) -> Result<String> {
        let response = self.client
        .post(url)
        // TODO serialise the message vec
        .send()
        .await
        .unwrap();
        
        Ok(response.text().await.unwrap())
    }
}