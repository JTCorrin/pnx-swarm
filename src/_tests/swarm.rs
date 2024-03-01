#[cfg(test)]
mod tests {
    use crate::llm::{Ollama, OpenAI, OpenAIBuilder};
    use crate::swarm::{SwarmBuilder, Swarm, Process};
    use crate::task::{TaskBuilder, Task};
    use crate::agent::{AgentBuilder, Agent};
    use crate::tool::{ToolBuilder, Tool};
use std::default::Default;

    #[test]
    fn test_swarm() {
        
        let task = Task::new("Requirement", "Description");
        let agent: Agent = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(Box::new(Ollama::default()))
            .build()
            .unwrap();

        let tool = Tool::new("Tool", "Description", false, false, false, serde_json::Value::Null, |value| Ok(value));
        
        let swarm = SwarmBuilder::new()
            .tasks(vec![task])
            .agents(vec![agent])
            .process(Process::Sequential)
            .tools(vec![tool])
            .build();
        
        assert_eq!(swarm.tasks.len(), 1);
        assert_eq!(swarm.agents.len(), 1);
        assert_eq!(swarm.tools.unwrap().len(), 1);
    }

    #[test]
    fn test_swarm_builder_missing_tasks() {
        let open_ai = OpenAIBuilder::new()
            .model_name("gpt-3")
            .api_key("api_key")
            .build()
            .unwrap();

        let ollama_ai = Ollama::default();

        let open_ai_agent: Agent = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(Box::new(open_ai))
            .build()
            .unwrap();
        
        let ollama_agent: Agent = AgentBuilder::new()
            .role("Role")
            .goal("Goal")
            .backstory("Backstory")
            .verbose(true)
            .allow_delegation(true)
            .llm(Box::new(ollama_ai))
            .build()
            .unwrap();

        let tool = Tool::new("Tool", "Description", false, false, false, serde_json::Value::Null, |value| Ok(value));
        let swarm = SwarmBuilder::new()
            .agents(vec![open_ai_agent, ollama_agent])
            .process(Process::Sequential)
            .tools(vec![tool])
            .build();

        
    }

    #[test]
    fn test_swarm_builder_missing_agents() {
        let task = Task::new("Requirement", "Description");
        let tool = Tool::new("Tool", "Description", false, false, false, serde_json::Value::Null, |value| Ok(value));
        let swarm = SwarmBuilder::new()
            .tasks(vec![task])
            .process(Process::Sequential)
            .tools(vec![tool])
            .build();
    }

    #[test]
    fn test_swarm_builder_missing_process() {
        let task = Task::new("Requirement", "Description");
        let agent = Agent::new("role", "goal", "backstory", true, true, Box::new(Ollama::default()));
        let tool = Tool::new("Tool", "Description", false, false, false, serde_json::Value::Null, |value| Ok(value));
        let swarm = SwarmBuilder::new()
            .tasks(vec![task])
            .agents(vec![agent])
            .tools(vec![tool])
            .build();

    }
}