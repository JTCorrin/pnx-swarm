use crate::prelude::*;

pub trait LLM {
    fn call(&self, message: impl Into<String>) -> Result<String>;
}

#[derive(Debug, Clone)]
pub struct Ollama {
    pub base_url: String,
    pub model_name: String,
    api_key: String,
    end_point: String,
}

impl Default for Ollama {
    fn default() -> Self {
        Ollama {
            base_url: "http://localhost:11434".to_string(),
            model_name: "llama2".to_string(),
            end_point: "/api/chat".to_string(),
            api_key: "Not required".to_string(),
        }
    }
}

impl Ollama {
    pub fn new(base_url: impl Into<String>, model_name: impl Into<String>) -> Self {
        let base_url = base_url.into();
        Ollama {
            base_url,
            model_name: model_name.into(),
            ..Default::default()
        }
    }
}

impl LLM for Ollama {
    fn call(&self, message: impl Into<String>) -> Result<String> {
        // let message = message.into();
        // let url = format!("{}/{}{}", self.base_url, self.model_name, self.end_point);
        // let client = reqwest::blocking::Client::new();
        // let response = client.post(&url)
        //     .header("Authorization", &self.api_key)
        //     .json(&json!({
        //         "message": message
        //     }))
        //     .send()?;

        // let response = response.text()?;
        // Ok(response)
        todo!("Implement Ollama call method")
    }
}

#[derive(Debug, Default)]
pub struct OllamaBuilder {
    base_url: Option<String>,
    model_name: Option<String>,
}

impl OllamaBuilder {
    pub fn new() -> Self {
        OllamaBuilder::default()
    }

    pub fn base_url(&mut self, base_url: impl Into<String>) -> &mut Self {
        self.base_url.insert(base_url.into());
        self
    }

    pub fn model_name(&mut self, model_name: impl Into<String>) -> &mut Self {
        self.model_name.insert(model_name.into());
        self
    }

    pub fn build(&self) -> Result<Ollama> {
        let base_url = self.base_url.as_ref().ok_or(Error::Generic("Base URL is required".to_string()))?;
        let model_name = self.model_name.as_ref().ok_or(Error::Generic("Model name is required".to_string()))?;

        Ok(
            Ollama::new(
                base_url,
                model_name
            )
        )
    }
}

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
    fn call(&self, message: impl Into<String>) -> Result<String> {
        // let message = message.into();
        // let url = format!("{}/{}{}", self.base_url, self.model_name, self.end_point);
        // let client = reqwest::blocking::Client::new();
        // let response = client.post(&url)
        //     .header("Authorization", &self.api_key)
        todo!("Implement OpenAI call method")
    }
}

#[derive(Debug, Default)]
pub struct OpenAIBuilder {
    model_name: Option<String>,
    api_key: Option<String>,
}

impl OpenAIBuilder {
    pub fn new() -> Self {
        OpenAIBuilder::default()
    }

    pub fn model_name(&mut self, model_name: impl Into<String>) -> &mut Self {
        self.model_name.insert(model_name.into());
        self
    }

    pub fn api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key.insert(api_key.into());
        self
    }

    pub fn build(&self) -> Result<OpenAI> {
        Ok(
            OpenAI::new(
                self.api_key.clone().unwrap(),
                self.model_name.clone().unwrap()
            )
        )
    }
}

#[cfg(test)]
#[path = "_tests/llm.rs"]
mod llm;

