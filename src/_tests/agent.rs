#[cfg(test)]
mod tests {
    use crate::{agent::{Agent, AgentBuilder}, llm::{Ollama, OpenAI, OpenAIBuilder}};

    #[test]
    fn test_agent_builder() {
        let ollama_agent: Agent = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(Box::new(Ollama::default()))
            .build()
            .unwrap();

        assert_eq!(ollama_agent.role, "Role");
        assert_eq!(ollama_agent.goal, "Goal");
        assert_eq!(ollama_agent.backstory, "Backstory");
        assert_eq!(ollama_agent.verbose, true);
        assert_eq!(ollama_agent.allow_delegation, true);

        let open_ai_agent: Agent = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(Box::new(OpenAI::new("api_key", "model_name")))
            .build()
            .unwrap();

        assert_eq!(open_ai_agent.role, "Role");
        assert_eq!(open_ai_agent.goal, "Goal");
        assert_eq!(open_ai_agent.backstory, "Backstory");
        assert_eq!(open_ai_agent.verbose, true);
        assert_eq!(open_ai_agent.allow_delegation, true);

    }

    #[test]
    fn test_agent_default() {
        let mut agent: Agent = Agent::default();

        assert_eq!(agent.role, "Role not yet set");
        assert_eq!(agent.goal, "Goal not yet set");
        assert_eq!(agent.backstory, "No backstory");
        assert_eq!(agent.verbose, false);
        assert_eq!(agent.allow_delegation, false);

        agent.role = "Role".to_string();
        assert_eq!(agent.role, "Role");

    }

    #[test]
    fn test_agent_new() {
        let open_ai_agent: Agent = Agent::new("Role", "Goal", "Backstory", true, false, Box::new(OpenAI::new("api_key", "model_name")));
        let ollama_agent: Agent = Agent::new("Role", "Goal", "Backstory", true, false, Box::new(Ollama::default()));

        assert_eq!(open_ai_agent.role, "Role");
        assert_eq!(open_ai_agent.goal, "Goal");
        assert_eq!(open_ai_agent.backstory, "Backstory");
        assert_eq!(open_ai_agent.verbose, true);
        assert_eq!(open_ai_agent.allow_delegation, false);
        
        assert_eq!(ollama_agent.role, "Role");
        assert_eq!(ollama_agent.goal, "Goal");
        assert_eq!(ollama_agent.backstory, "Backstory");
        assert_eq!(ollama_agent.verbose, true);
        assert_eq!(ollama_agent.allow_delegation, false);
    }
}
