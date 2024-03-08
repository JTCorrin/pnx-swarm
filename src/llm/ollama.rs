use crate::{message::Message, prelude::*};
use super::{transport::Transport, ModelName, NoModelName, LLM};

#[derive(Debug, Clone)]
pub struct Ollama {
    pub base_url: String,
    pub model_name: String,
    api_key: String,
    end_point: String,
    transport: Transport
}

impl Default for Ollama {
    fn default() -> Self {
        Ollama {
            base_url: "http://localhost:11434".to_string(),
            model_name: "llama2".to_string(),
            end_point: "/api/chat".to_string(),
            api_key: "Not required".to_string(),
            transport: Transport::new()
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
    fn call(&self, messages: Vec<Message>) -> Result<String> {
        todo!("Implement Ollama call method")
    }
}

#[derive(Debug, Default)]
pub struct OllamaBuilder<M> {
    base_url: Option<String>,
    model_name: M,
}

impl OllamaBuilder<NoModelName> {
    pub fn new() -> Self {
        OllamaBuilder::default()
    }
}

impl OllamaBuilder<ModelName> {
    pub fn build(&self) -> Result<Ollama> {
        // Get base url is self has one otherwise default to default base url
        let base_url = self.base_url.clone().unwrap_or_else(|| "http://localhost:11434".to_string());

        Ok(
            Ollama {
                base_url,
                model_name: self.model_name.0.clone(),
                ..Default::default()
            }
        )
    }
}

impl<M> OllamaBuilder <M> {

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url.insert(base_url.into());
        self
    }

    pub fn model_name(self, model_name: impl Into<String>) -> OllamaBuilder<ModelName> {
        OllamaBuilder { 
            base_url: self.base_url,
            model_name: ModelName(model_name.into())
        }
    }
}

#[cfg(test)]
#[path = "../_tests/ollama.rs"]
mod tests;

