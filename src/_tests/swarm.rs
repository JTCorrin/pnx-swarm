#[cfg(test)]
mod tests {
    use crate::llm::{ollama::{Ollama, OllamaBuilder}, open_ai::OpenAIBuilder};
    use crate::swarm::{SwarmBuilder, Swarm, Process};
    use crate::task::{TaskBuilder, Task};
    use crate::agent::{AgentBuilder, Agent};
    use crate::tool::{ToolBuilder, Tool};


    #[test]
    fn test_swarm() {
        
        let task = Task::new("Requirement", "Description", "Expected Outcome");
        
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
            .build()
            .unwrap();
        
        assert_eq!(swarm.tasks.len(), 1);
        assert_eq!(swarm.agents.len(), 1);
        assert_eq!(swarm.tools.unwrap().len(), 1);
    }


    fn test_swarm_builder_missing_process() {
        let task = Task::new("Requirement", "Description", "Expected Outcome");
        let agent = AgentBuilder::new()
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
            .tools(vec![tool])
            .build()
            .unwrap();

        assert_eq!(swarm.process, Process::Sequential);
    }
}