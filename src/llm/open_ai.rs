use crate::prelude::*;
use super::{LLM, ModelName, NoModelName, APIKey, NoAPIKey};

#[derive(Debug, Clone)]
pub struct OpenAI {
    base_url: String,
    pub model_name: String,
    pub api_key: String,
    end_point: String,
}

impl OpenAI {
    pub fn new(api_key: impl Into<String>, model_name: impl Into<String>) -> Self {
        OpenAI {
            base_url: "https://api.openai.com".to_string(),
            end_point: "/chat/completions".to_string(),
            model_name: model_name.into(),
            api_key: api_key.into(),
        }
    }
}

impl LLM for OpenAI {
    fn call(&self, message: String) -> Result<String> {
        todo!("Implement OpenAI call method")
    }
}

#[derive(Debug, Default)]
pub struct OpenAIBuilder<M, K>
where M: Clone, K: Clone {
    model_name: M,
    api_key: K,
}

impl OpenAIBuilder<NoModelName, NoAPIKey> {
    pub fn new() -> Self {
        OpenAIBuilder::default()
    }
}

impl OpenAIBuilder<ModelName, APIKey> {
    pub fn build(&self) -> Result<OpenAI> {
        Ok(
           OpenAI { 
                model_name: self.model_name.0.clone(), 
                api_key: self.api_key.0.clone(),
                base_url: "https://api.openai.com".to_string(),
                end_point: "/chat/completions".to_string(),
            }
        )
    }
}

impl<M: Clone, K: Clone> OpenAIBuilder<M, K> {

    pub fn model_name(&mut self, model_name: impl Into<String>) -> OpenAIBuilder<ModelName, K> {
        OpenAIBuilder {
            model_name: ModelName(model_name.into()),
            api_key: self.api_key.clone(),
        }
    }

    pub fn api_key(&mut self, api_key: impl Into<String>) -> OpenAIBuilder<M, APIKey> {
        OpenAIBuilder {
            model_name: self.model_name.clone(),
            api_key: APIKey(api_key.into()),
        }
    }
}

#[cfg(test)]
#[path = "../_tests/open_ai.rs"]
mod tests;