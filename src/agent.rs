use serde::de::DeserializeOwned;

use crate::{
    message::MessageBuilder, 
    planner::Planner, 
    prelude::*, 
    task::Task, 
    llm::LLM
};


#[derive(Clone)]
pub struct Agent {
    role: String,
    goal: String,
    backstory: String,
    verbose: bool,
    allow_delegation: bool,
    llm: LLM,
    planner: Planner,
    // tools: Vec<Tool>,
}

impl Agent {

    async fn create_plan(&self, task: Task) -> Result<Vec<String>> {
        Ok(self.planner.create_plan(task).await?)
    }

    fn take_step(&self, action: &str) -> Result<String> {
        todo!("Take step: {}", action)
    }

    pub async fn execute(&self, task: Task) -> Result<String> {
        let plan = self.create_plan(task).await?;
        for action in plan {
            self.take_step(&action)?;
        }
        Ok("Task completed".to_string())
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
pub struct WithLLM(LLM);



#[derive(Default)]
pub struct AgentBuilder<R, G, B, L> {
    role: R,
    goal: G,
    backstory: B,
    verbose: Option<bool>,
    allow_delegation: Option<bool>,
    llm: L
    // tools: Option<Vec<Tool>>,
}

impl AgentBuilder<NoRole, NoGoal, NoBackstory, NoLLM> {
    pub fn new() -> Self {
        AgentBuilder::default()
    }
}

impl AgentBuilder<Role, Goal, Backstory, WithLLM> {
    pub fn build(&self) -> Result<Agent> {
        let verbose = self.verbose.clone().unwrap_or_default();
        let allow_delegation = self.allow_delegation.clone().unwrap_or_default();

        Ok(Agent {
            role: self.role.0.clone(),
            goal: self.goal.0.clone(),
            backstory: self.backstory.0.clone(),
            verbose,
            allow_delegation,
            llm: self.llm.0.clone(),
            planner: Planner::new(self.llm.0.clone()),
        })
    }
}

impl<R, G, B, L> AgentBuilder<R, G, B, L> {

    pub fn role(mut self, role: impl Into<String>) -> AgentBuilder<Role, G, B, L> {
        AgentBuilder {
            role: Role(role.into()),
            goal: self.goal,
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm
            // tools: self.tools,
        }
    }

    pub fn goal(mut self, goal: impl Into<String>) -> AgentBuilder<R, Goal, B, L> {
        AgentBuilder {
            role: self.role,
            goal: Goal(goal.into()),
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm
            // tools: self.tools,
        }
    }

    pub fn backstory(mut self, backstory: impl Into<String>) -> AgentBuilder<R, G, Backstory, L> {
        AgentBuilder {
            role: self.role,
            goal: self.goal,
            backstory: Backstory(backstory.into()),
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm
            // tools: self.tools,
        }
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose.insert(verbose);
        self
    }

    pub fn allow_delegation(mut self, allow_delegation: bool) -> Self {
        self.allow_delegation.insert(allow_delegation);
        self
    }

    pub fn llm(mut self, llm: LLM) -> AgentBuilder<R, G, B, WithLLM>{
        AgentBuilder {
            role: self.role,
            goal: self.goal,
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: WithLLM(llm)
            // tools: self.tools,
        }
    }

    pub fn planner(mut self, planner: Planner) -> AgentBuilder<R, G, B, L>{
        AgentBuilder {
            role: self.role,
            goal: self.goal,
            backstory: self.backstory,
            verbose: self.verbose,
            allow_delegation: self.allow_delegation,
            llm: self.llm
            // tools: self.tools,
        }
    }

}


#[cfg(test)]
#[path = "_tests/agent.rs"]
mod tests;