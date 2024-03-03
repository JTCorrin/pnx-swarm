use std::vec;

use crate::{agent::Agent, llm::{open_ai::OpenAIBuilder, LLM}, prelude::*, task::Task, tool::Tool};

// States
#[derive(Default, Clone)]
pub struct NoTasks;

#[derive(Default, Clone)]
pub struct Tasks(Vec<Task>);

#[derive(Default, Clone)]
pub struct NoAgents;

#[derive(Default, Clone)]
pub struct Agents(Vec<Agent>);



#[derive(Debug, Clone, PartialEq)]
enum Process {
    Sequential,
    Parallel
}

struct Swarm {
    tasks: Vec<Task>, 
    agents: Vec<Agent>,
    verbose: bool,
    process: Process,
    tools: Option<Vec<Tool>>, // Not necessarily required
    complete: bool
}

impl Swarm {
    fn run() {
        todo!("Implement run method")
    }
}


#[derive(Default)]
pub struct SwarmBuilder<T, A> {
    tasks: T,
    agents: A,
    verbose: Option<bool>,
    process: Option<Process>,
    tools: Option<Vec<Tool>>
}

impl SwarmBuilder<NoTasks, NoAgents> {
    fn new() -> Self {
        SwarmBuilder::default()
    }
}

impl SwarmBuilder<Tasks, Agents> {
    fn build(&self) -> Result<Swarm> {
        let process = self.process.clone().unwrap_or(Process::Sequential);

        Ok(
            Swarm {
                tasks: self.tasks.0.clone(),
                agents: self.agents.0.clone(),
                verbose: self.verbose.unwrap_or_default(),
                process: process.clone(),
                tools: self.tools.clone(),
                complete: false
            }
        )
    }
}

impl <T: Clone, A: Clone>SwarmBuilder<T, A> {
    fn tasks(&mut self, tasks: Vec<Task>) -> SwarmBuilder<Tasks, A> {
        SwarmBuilder {
            tasks: Tasks(tasks),
            agents: self.agents.clone(),
            verbose: self.verbose.clone(),
            process: self.process.clone(),
            tools: self.tools.clone()
        }
    }

    fn agents(&mut self, agents: Vec<Agent>) -> SwarmBuilder<T, Agents> {
        SwarmBuilder {
            tasks: self.tasks.clone(),
            agents: Agents(agents),
            verbose: self.verbose.clone(),
            process: self.process.clone(),
            tools: self.tools.clone()
        }
    }

    fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose.insert(verbose);
        self
    }

    fn process(&mut self, process: Process) -> &mut Self {
        self.process.insert(process);
        self
    }

    fn tools(&mut self, tools: Vec<Tool>) -> &mut Self {
        self.tools.insert(tools);
        self
    }
}


#[cfg(test)]
#[path = "_tests/swarm.rs"]
mod tests;