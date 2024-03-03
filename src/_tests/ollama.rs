#[cfg(test)]
mod tests {
    use crate::llm::ollama::{OllamaBuilder, Ollama};

    #[test]
    fn test_ollama_builder() {

        let builder = OllamaBuilder::new();
        assert_eq!(builder.base_url, None);

        let model = builder
            .base_url("http://localhost:11434")
            .model_name("llama2")
            .build()
            .unwrap();

        assert_eq!(model.model_name, "llama2");

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