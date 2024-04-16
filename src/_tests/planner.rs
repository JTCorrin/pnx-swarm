#[cfg(test)]
mod tests {
    use crate::{llm::{LLMBuilder, LLMType}, message::Message, planner::Planner, task::TaskBuilder};

    #[tokio::test]
    async fn test_planner_builder() {
        let llm = LLMBuilder::new()
            .base_url("http://192.168.1.73:11434")
            .completion_endpoint("/api/chat")
            .model_name("mistral-openorca")
            .api_key("not_required")
            .llm_type(LLMType::Ollama)
            .build()
            .unwrap();
        
        let task = TaskBuilder::new()
            .requirement("Develop an idea for a simple yet profitable business")
            .description("You are to develop an idea for a simple yet profitable business. The idea should be simple, yet profitable enough to be a viable business.")
            .expected_outcome("A simple yet profitable business idea in a business plan")
            .build()
            .unwrap();

        let planner = Planner::new(llm);
        
        let plan = planner.create_plan(task).await.unwrap();

        println!("{:?}", plan);

    }
}