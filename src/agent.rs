use crate::{message::MessageBuilder, planner::Planner, prelude::*, task::Task};
use super::llm::{LLM, ollama::Ollama};


#[derive(Clone)]
pub struct Agent {
    role: String,
    goal: String,
    backstory: String,
    verbose: bool,
    allow_delegation: bool,
    llm: Box<dyn LLM>,
    planner: Planner,
    // tools: Vec<Tool>,
}

impl Agent {
    pub fn new(
        role: impl Into<String>, 
        goal: impl Into<String>, 
        backstory: impl Into<String>, 
        verbose: bool,
        allow_delegation: bool,
        llm: Box<dyn LLM>,
        planner: Planner
    ) -> Agent {
        Agent {
            role: role.into(),
            goal: goal.into(),
            backstory: backstory.into(),
            verbose,
            allow_delegation,
            llm,
            planner
            // tools: vec![],
        }
    }

    fn create_plan(&self, task: Task) -> Result<Vec<String>> {
        let plan = self.planner.create_plan(task);
        Ok(plan)
    }

    fn take_step(&self, action: &str) -> Result<String> {
        todo!("Take step: {}", action)
    }

    pub fn execute(&self, task: Task) -> Result<String> {
        let plan = self.create_plan(task)?;
        let mut result = String::new();
        for action in plan {
            let step_result = self.take_step(&action)?;
            result.push_str(&step_result);
        }
        Ok(result)
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
            llm: Box::new(Ollama::default()),
            planner: Planner::default(),
            // tools: vec![],
        }
    }
}



// Builder

#[derive(Default, Clone)]
pub struct NoRole;

#[derive(Default, Clone)]
pub struct Role(String);

#[derive(Default, Clone)]
pub struct NoGoal;

#[derive(Default, Clone)]
pub struct Goal(String);

#[derive(Default, Clone)]
pub struct NoBackstory;

#[derive(Default, Clone)]
pub struct Backstory(String);

#[derive(Default, Clone)]
pub struct NoLLM;

#[derive(Clone)]
pub struct WithLLM(Box<dyn LLM>);

impl Default for WithLLM {
    fn default() -> Self {
        WithLLM(Box::new(Ollama::default()))
    }
}

#[derive(Default, Clone)]
pub struct NoPlanner;

#[derive(Default, Clone)]
pub struct WithPlanner(Planner);



#[derive(Default)]
pub struct AgentBuilder<R, G, B, L, P> {
    role: R,
    goal: G,
    backstory: B,
    verbose: Option<bool>,
    allow_delegation: Option<bool>,
    llm: L,
    planner: P,
    // tools: Option<Vec<Tool>>,
}

impl AgentBuilder<NoRole, NoGoal, NoBackstory, NoLLM, NoPlanner> {
    pub fn new() -> Self {
        AgentBuilder::default()
    }
}

impl AgentBuilder<Role, Goal, Backstory, WithLLM, WithPlanner> {
    pub fn build(&self) -> Result<Agent> {
        let verbose = self.verbose.clone().unwrap_or_default();
        let allow_delegation = self.allow_delegation.clone().unwrap_or_default();

        Ok(Agent::new(
            self.role.0,
            self.goal.0,
            self.backstory.0,
            verbose,
            allow_delegation,
            self.llm.0,
            self.planner.0
        ))
    }
}

impl<R, G, B, L, P> AgentBuilder<R, G, B, L, P> {

    pub fn role(&mut self, role: impl Into<String>) -> AgentBuilder<Role, G, B, L, P> {
        AgentBuilder {
            role: Role(role.into()),
            goal: self.goal,
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm,
            planner: self.planner,
            // tools: self.tools,
        }
    }

    pub fn goal(&mut self, goal: impl Into<String>) -> AgentBuilder<R, Goal, B, L, P> {
        AgentBuilder {
            role: self.role,
            goal: Goal(goal.into()),
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm,
            planner: self.planner,
            // tools: self.tools,
        }
    }

    pub fn backstory(&mut self, backstory: impl Into<String>) -> AgentBuilder<R, G, Backstory, L, P> {
        AgentBuilder {
            role: self.role,
            goal: self.goal,
            backstory: Backstory(backstory.into()),
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm,
            planner: self.planner,
            // tools: self.tools,
        }
    }

    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose.insert(verbose);
        self
    }

    pub fn allow_delegation(&mut self, allow_delegation: bool) -> &mut Self {
        self.allow_delegation.insert(allow_delegation);
        self
    }

    pub fn llm(&mut self, llm: Box<dyn LLM>) -> AgentBuilder<R, G, B, WithLLM, P>{
        AgentBuilder {
            role: self.role,
            goal: self.goal,
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: WithLLM(llm),
            planner: self.planner,
            // tools: self.tools,
        }
    }

    pub fn planner(&mut self, planner: Planner) -> AgentBuilder<R, G, B, L, WithPlanner>{
        AgentBuilder {
            role: self.role,
            goal: self.goal,
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm,
            planner: WithPlanner(planner),
            // tools: self.tools,
        }
    }

}


#[cfg(test)]
#[path = "_tests/agent.rs"]
mod tests;