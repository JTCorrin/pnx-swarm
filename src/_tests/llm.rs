#[cfg(test)]
mod tests {
    use crate::llm::{OpenAIBuilder, OllamaBuilder, Ollama};

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

    #[test]
    fn test_ollama_builder() {
        let ollama = OllamaBuilder::new()
            .base_url("http://localhost:11434")
            .model_name("llama2")
            .build()
            .unwrap();

        assert_eq!(ollama.model_name, "llama2");
        assert_eq!(ollama.base_url, "http://localhost:11434");
    }

    #[test]
    fn test_ollama_new() {
        let ollama = Ollama::new("http://192.168.1.73:11434", "llama2");
        assert_eq!(ollama.model_name, "llama2");
        assert_eq!(ollama.base_url, "http://192.168.1.73:11434");
    }

    #[test]
    fn test_ollama_default() {
        let ollama = Ollama::default();
        assert_eq!(ollama.model_name, "llama2");
        assert_eq!(ollama.base_url, "http://localhost:11434");
        assert_eq!(ollama.end_point, "/api/chat")
    }

    #[test]
    fn test_ollama_call() {
        todo!("Implement Ollama call method")
    }
}