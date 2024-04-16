use serde::de::DeserializeOwned;
use serde_derive::Deserialize;

use crate::{message::Message, transport::Transport};
use crate::prelude::*;

#[derive(Debug, Deserialize, Clone)]
struct Choice {
    index: usize,
    message: Message,
    finish_reason: String
}

#[derive(Debug, Deserialize, Clone)]
struct OpenAIUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize
}

#[derive(Debug, Deserialize, Clone)]
struct AnthropicUsage {
    input_tokens: usize,
    output_tokens: usize
}

#[derive(Debug, Deserialize, Clone)]
struct AnthropicContent {
    text: String,
}


#[derive(Debug, Clone)]
pub struct StandardResponse {
    pub message: Message, // Assuming you might have multiple messages
    pub token_count: usize,
}


#[derive(Debug, Deserialize, Clone)]
struct OllamaResponse {
    model: String,
    created_at: String,
    message: Message,
    done: bool,
    total_duration: usize,
    load_duration: usize,
    prompt_eval_count: usize,
    prompt_eval_duration: usize,
    eval_count: usize,
    eval_duration: usize
}

#[derive(Debug, Deserialize, Clone)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: usize,
    model: String,
    system_fingerprint: String,
    choices: Vec<Choice>,
    usage: OpenAIUsage
}

#[derive(Debug, Deserialize, Clone)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    id: String,
    model: String,
    role: String,
    stop_reason: String,
    stop_sequence: Option<String>,
    usage: AnthropicUsage
}

impl OllamaResponse {
    fn to_standard(&self) -> StandardResponse {
        StandardResponse {
            message: Message {
                role: self.message.role.clone(),
                content: self.message.content.clone()
            },
            token_count: self.prompt_eval_count,
        }
    }
}

impl OpenAIResponse {
    fn to_standard(&self) -> StandardResponse {
        StandardResponse {
            message: self.choices[0].message.clone(),
            token_count: self.usage.total_tokens,
        }
    }
}

impl AnthropicResponse {
    fn to_standard(&self) -> StandardResponse {
        let token_count = self.usage.input_tokens + self.usage.output_tokens;
        StandardResponse {
            message: Message {
                role: self.role.clone(),
                content: self.content[0].text.clone()
            },
            token_count,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LLMType {
    Ollama,
    OpenAI,
    Anthropic,
}

#[derive(Debug, Deserialize)]
pub enum LLMResponse {
    Ollama(OllamaResponse),
    OpenAI(OpenAIResponse),
    Anthropic(AnthropicResponse),
}

#[derive(Clone)]
pub struct LLM {
    base_url: String,
    completion_endpoint: String,
    model_name: String,
    api_key: String,
    transport: Transport,
    llm_type: LLMType
}

impl LLM {
    pub fn new (
        base_url: impl Into<String>, 
        completion_endpoint: impl Into<String>, 
        model_name: impl Into<String>, 
        api_key: impl Into<String>,
        llm_type: LLMType
    ) -> Self {
        LLM {
            base_url: base_url.into(),
            completion_endpoint: completion_endpoint.into(),
            model_name: model_name.into(),
            api_key: api_key.into(),
            transport: Transport::new(),
            llm_type
        }
    }
    pub async fn call(&self, messages: Vec<Message>) -> Result<StandardResponse> {
        let url = format!("{}{}", self.base_url, self.completion_endpoint);
        let model = self.model_name.clone();
        let api_key = self.api_key.clone();
        let res = match self.llm_type {
            LLMType::Ollama => {
                let res = self.transport.post::<OllamaResponse>(url, model, messages, api_key).await.unwrap();
                res.to_standard()
            },
            LLMType::OpenAI => {
                let res = self.transport.post::<OpenAIResponse>(url, model, messages, api_key).await.unwrap();
                res.to_standard()
            },
            LLMType::Anthropic => {
                let res = self.transport.post::<AnthropicResponse>(url, model, messages, api_key).await.unwrap();
                res.to_standard()
            },
        };

        Ok(res)
    }

    pub async fn complete<T: DeserializeOwned>(&self, prompt: String) -> Result<T> {
        todo!("Complete function llm")
    }
}


// State

#[derive(Default, Clone)]
pub struct NoBaseURL;

#[derive(Default, Clone)]
pub struct BaseURL(String);

#[derive(Default, Clone)]
pub struct NoCompletionEndpoint;

#[derive(Default, Clone)]
pub struct CompletionEndpoint(String);

#[derive(Default, Clone)]
pub struct NoModelName;

#[derive(Default, Clone)]
pub struct ModelName(String);

#[derive(Default, Clone)]
pub struct NoAPIKey;

#[derive(Default, Clone)]
pub struct APIKey(String);

#[derive(Default, Clone)]
pub struct NoLLMType;

#[derive(Clone)]
pub struct WithLLMType(LLMType);


#[derive(Default)]
pub struct LLMBuilder<B, C, M, K, T> {
    base_url: B,
    completion_endpoint: C,
    model_name: M,
    api_key: K,
    llm_type: T
}

impl LLMBuilder <NoBaseURL, NoCompletionEndpoint, NoModelName, NoAPIKey, NoLLMType> {
    pub fn new() -> Self {
        LLMBuilder::default()
    }
}

impl LLMBuilder <BaseURL, CompletionEndpoint, ModelName, APIKey, WithLLMType> {
    pub fn build(&self) -> Result<LLM> {
        let base_url = self.base_url.0.clone();
        let completion_endpoint = self.completion_endpoint.0.clone();
        let model_name = self.model_name.0.clone();
        let api_key = self.api_key.0.clone();
        let llm_type = self.llm_type.0.clone();

        Ok(
            LLM {
                base_url,
                completion_endpoint,
                model_name,
                api_key,
                transport: Transport::new(),
                llm_type
            }
        )
    }
}

impl <B: Clone, C: Clone, M: Clone, K: Clone, T: Clone> LLMBuilder <B, C, M, K, T> {
    pub fn base_url(&mut self, base_url: impl Into<String>) -> LLMBuilder<BaseURL, C, M, K ,T> {
        LLMBuilder {
            base_url: BaseURL(base_url.into()),
            completion_endpoint: self.completion_endpoint.clone(),
            model_name: self.model_name.clone(),
            api_key: self.api_key.clone(),
            llm_type: self.llm_type.clone()
        }
    }

    pub fn completion_endpoint(&mut self, completion_endpoint: impl Into<String>) -> LLMBuilder<B, CompletionEndpoint, M, K, T> {
        LLMBuilder {
            base_url: self.base_url.clone(),
            completion_endpoint: CompletionEndpoint(completion_endpoint.into()),
            model_name: self.model_name.clone(),
            api_key: self.api_key.clone(),
            llm_type: self.llm_type.clone()
        }
    }

    pub fn model_name(&mut self, model_name: impl Into<String>) -> LLMBuilder<B, C, ModelName, K, T> {
        LLMBuilder {
            base_url: self.base_url.clone(),
            completion_endpoint: self.completion_endpoint.clone(),
            model_name: ModelName(model_name.into()),
            api_key: self.api_key.clone(),
            llm_type: self.llm_type.clone()
        }
    }

    pub fn api_key(&mut self, api_key: impl Into<String>) -> LLMBuilder<B, C, M, APIKey, T> {
        LLMBuilder {
            base_url: self.base_url.clone(),
            completion_endpoint: self.completion_endpoint.clone(),
            model_name: self.model_name.clone(),
            api_key: APIKey(api_key.into()),
            llm_type: self.llm_type.clone()
        }
    }

    pub fn llm_type(&mut self, llm_type: LLMType) -> LLMBuilder<B, C, M, K, WithLLMType> {
        LLMBuilder {
            base_url: self.base_url.clone(),
            completion_endpoint: self.completion_endpoint.clone(),
            model_name: self.model_name.clone(),
            api_key: self.api_key.clone(),
            llm_type: WithLLMType(llm_type)
        }
    }

}


#[cfg(test)]
#[path = "_tests/llm.rs"]
mod tests;