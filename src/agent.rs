use crate::{llm::{Ollama, LLM}, prelude::*};

#[derive(Debug)]
pub struct Agent<T: LLM> {
    role: String,
    goal: String,
    backstory: String,
    verbose: bool,
    allow_delegation: bool,
    llm: T
    // tools: Vec<Tool>,
}

#[derive(Default)]
pub struct AgentBuilder<T: LLM> {
    role: Option<String>,
    goal: Option<String>,
    backstory: Option<String>,
    verbose: Option<bool>,
    allow_delegation: Option<bool>,
    llm: Option<T>
    // tools: Option<Vec<Tool>>,
}

impl <T> AgentBuilder<T>
where T: LLM + Clone {
    fn new() -> Self {
        Self { 
            role: None,
            goal: None,
            backstory: None,
            verbose: Some(true),
            allow_delegation: Some(true),
            llm: None 
        }
    }

    fn role(&mut self, role: impl Into<String>) -> &mut Self {
        self.role.insert(role.into());
        self
    }

    fn goal(&mut self, goal: impl Into<String>) -> &mut Self {
        self.goal.insert(goal.into());
        self
    }

    fn backstory(&mut self, backstory: impl Into<String>) -> &mut Self {
        self.backstory.insert(backstory.into());
        self
    }

    fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose.insert(verbose);
        self
    }

    fn allow_delegation(&mut self, allow_delegation: bool) -> &mut Self {
        self.allow_delegation.insert(allow_delegation);
        self
    }

    fn llm(&mut self, llm: T) -> &mut Self {
        self.llm.insert(llm);
        self
    }

    fn build(&self) -> Result<Agent<T>> {
        let role = self.role.as_ref().ok_or(Error::Generic("Agent role is required".to_string()))?;
        let goal = self.goal.as_ref().ok_or(Error::Generic("Agent goal is required".to_string()))?;
        let backstory = self.backstory.as_ref().ok_or(Error::Generic("Agent backstory is required".to_string()))?;
        let verbose = self.verbose.clone().unwrap_or_default();
        let allow_delegation = self.allow_delegation.clone().unwrap_or_default();
        let llm = self.llm.as_ref().ok_or(Error::Generic("LLM is required".to_string()))?;
        // let tools = self.tools.clone().unwrap_or_default();

        Ok(Agent {
            role: role.to_string(),
            goal: goal.to_string(),
            backstory: backstory.to_string(),
            verbose,
            allow_delegation,
            llm: llm.clone()
            // tools,
        })
    }
}

impl <T: LLM + Default> Default for Agent<T> {
    fn default() -> Agent<T> {
        Agent {
            role: "Role not yet set".to_string(),
            goal: "Goal not yet set".to_string(),
            backstory: "No backstory".to_string(),
            verbose: false,
            allow_delegation: false,
            llm: T::default()
            // tools: vec![],
        }
    }
}


impl <T: LLM> Agent<T> {
    fn new(
        role: impl Into<String>, 
        goal: impl Into<String>, 
        backstory: impl Into<String>, 
        verbose: bool,
        allow_delegation: bool,
        llm: T
    ) -> Agent<T> {
        Agent {
            role: role.into(),
            goal: goal.into(),
            backstory: backstory.into(),
            verbose,
            allow_delegation,
            llm
            // tools: vec![],
        }
    }
}

#[cfg(test)]
#[path = "_tests/agent.rs"]
mod tests;