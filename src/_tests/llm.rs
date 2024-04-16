#[cfg(test)]

mod tests {
    use crate::{llm::{LLMBuilder, LLMType, OllamaResponse}, message::{Message, MessageBuilder}};

    #[tokio::test]
    async fn test_llm_builder() {
        let llm = LLMBuilder::new()
            .base_url("http://192.168.1.73:11434")
            .completion_endpoint("/api/chat")
            .model_name("mistral-openorca")
            .api_key("not_required")
            .llm_type(LLMType::Ollama)
            .build()
            .unwrap();

        assert_eq!(llm.base_url, "http://192.168.1.73:11434");
        assert_eq!(llm.completion_endpoint, "/api/chat");
        assert_eq!(llm.model_name, "mistral-openorca");
        assert_eq!(llm.api_key, "not_required");

        let sys_message = MessageBuilder::new()
            .role("system")
            .content("You are a helpful assistant")
            .build()
            .unwrap();

        let user_message = MessageBuilder::new()
            .role("user")
            .content("Hello, how are you?")
            .build()
            .unwrap();

        let messages = vec![sys_message, user_message];

        let res = llm.call(messages).await.unwrap(); 

        println!("{:?}", res);

    }
}