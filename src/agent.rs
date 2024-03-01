use crate::{llm::{Ollama, LLM}, prelude::*};

#[derive(Clone)]
pub struct Agent {
    role: String,
    goal: String,
    backstory: String,
    verbose: bool,
    allow_delegation: bool,
    llm: Box<dyn LLM>
    // tools: Vec<Tool>,
}

#[derive(Default)]
pub struct AgentBuilder {
    role: Option<String>,
    goal: Option<String>,
    backstory: Option<String>,
    verbose: Option<bool>,
    allow_delegation: Option<bool>,
    llm: Option<Box<dyn LLM>>
    // tools: Option<Vec<Tool>>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self { 
            role: None,
            goal: None,
            backstory: None,
            verbose: Some(true),
            allow_delegation: Some(true),
            llm: None 
        }
    }

    pub fn role(&mut self, role: impl Into<String>) -> &mut Self {
        self.role.insert(role.into());
        self
    }

    pub fn goal(&mut self, goal: impl Into<String>) -> &mut Self {
        self.goal.insert(goal.into());
        self
    }

    pub fn backstory(&mut self, backstory: impl Into<String>) -> &mut Self {
        self.backstory.insert(backstory.into());
        self
    }

    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose.insert(verbose);
        self
    }

    pub fn allow_delegation(&mut self, allow_delegation: bool) -> &mut Self {
        self.allow_delegation.insert(allow_delegation);
        self
    }

    pub fn llm(&mut self, llm: Box<dyn LLM>) -> &mut Self {
        self.llm.insert(llm);
        self
    }

    pub fn build(&self) -> Result<Agent> {
        let role = self.role.as_ref().ok_or(Error::Generic("Agent role is required".to_string()))?;
        let goal = self.goal.as_ref().ok_or(Error::Generic("Agent goal is required".to_string()))?;
        let backstory = self.backstory.as_ref().ok_or(Error::Generic("Agent backstory is required".to_string()))?;
        let verbose = self.verbose.clone().unwrap_or_default();
        let allow_delegation = self.allow_delegation.clone().unwrap_or_default();
        let llm = self.llm.as_ref().ok_or(Error::Generic("LLM is required".to_string()))?.clone();
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

impl Default for Agent {
    fn default() -> Agent {
        Agent {
            role: "Role not yet set".to_string(),
            goal: "Goal not yet set".to_string(),
            backstory: "No backstory".to_string(),
            verbose: false,
            allow_delegation: false,
            llm: Box::new(Ollama::default())
            // tools: vec![],
        }
    }
}


impl Agent {
    pub fn new(
        role: impl Into<String>, 
        goal: impl Into<String>, 
        backstory: impl Into<String>, 
        verbose: bool,
        allow_delegation: bool,
        llm: Box<dyn LLM>
    ) -> Agent {
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