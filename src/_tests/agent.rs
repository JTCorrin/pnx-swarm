#[cfg(test)]
mod tests {
    use crate::{agent::{Agent, AgentBuilder}, llm::LLM, planner::Planner, task::TaskBuilder};

    #[test]
    fn test_agent_builder() {
        let ollama_agent: Agent = AgentBuilder::new()
            .role("You are a friendly assistant")
            .goal("To assist the user")
            .backstory("You have been designed to help users with their queries")
            .verbose(true)
            .allow_delegation(true)
            .llm(LLM::new(
                "http://192.168.1.73:11434", 
                "/api/chat", 
                "mistral-openorca", 
                "api_key",
                crate::llm::LLMType::Ollama
            ))
            .build()
            .unwrap();

        assert_eq!(ollama_agent.role, "You are a friendly assistant");

        let task = TaskBuilder::new()
            .requirement("Develop an idea for a simple yet profitable business")
            .description("You are to develop an idea for a simple yet profitable business. The idea should be simple, yet profitable enough to be a viable business.")
            .expected_outcome("A simple yet profitable business idea in a business plan")
            .build()
            .unwrap();

        ollama_agent.execute(task);
        

    }

}
