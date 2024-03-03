#[cfg(test)]
mod tests {
    use crate::llm::open_ai::OpenAIBuilder;

    #[test]
    fn test_openai_builder() {
        let openai = OpenAIBuilder::new()
            .model_name("gpt-3")
            .api_key("api_key")
            .build()
            .unwrap();

        assert_eq!(openai.model_name, "gpt-3");
        assert_eq!(openai.api_key, "api_key");
    }

    #[test]
    fn test_openai_call() {
        todo!("Implement OpenAI call method")
    }
}