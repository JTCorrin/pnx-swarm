#[cfg(test)]
mod tests {
    use crate::{agent::{Agent, AgentBuilder}, llm::{Ollama, OpenAI, OpenAIBuilder}};

    #[test]
    fn test_agent_builder() {
        let ollama_agent: Agent<Ollama> = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(Ollama::default())
            .build()
            .unwrap();

        assert_eq!(ollama_agent.role, "Role");
        assert_eq!(ollama_agent.goal, "Goal");
        assert_eq!(ollama_agent.backstory, "Backstory");
        assert_eq!(ollama_agent.llm.base_url, "http://localhost:11434");
        assert_eq!(ollama_agent.verbose, true);
        assert_eq!(ollama_agent.allow_delegation, true);

        let open_ai_agent: Agent<OpenAI> = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(OpenAI::new("api_key", "model_name"))
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
        let mut agent: Agent<Ollama> = Agent::default();

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
        let open_ai_agent: Agent<OpenAI> = Agent::new("Role", "Goal", "Backstory", true, false, OpenAI::new("api_key", "model_name"));
        let ollama_agent: Agent<Ollama> = Agent::new("Role", "Goal", "Backstory", true, false, Ollama::default());

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
