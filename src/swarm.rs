use std::os::windows::process;

use crate::{agent::Agent, llm::LLM, prelude::*, task::Task, tool::Tool};

#[derive(Debug, Clone)]
enum Process {
    Sequential,
    Parallel
}

struct Swarm {
    tasks: Vec<Task>, 
    agents: Vec<Agent>,
    verbose: bool,
    process: Process,
    tools: Option<Vec<Tool>> // Not necessarily required
}

impl Swarm {
    fn run() {
        todo!("Implement run method")
    }
}


#[derive(Default)]
pub struct SwarmBuilder {
    tasks: Option<Vec<Task>>,
    agents: Option<Vec<Agent>>,
    verbose: Option<bool>,
    process: Option<Process>,
    tools: Option<Vec<Tool>>
}

impl SwarmBuilder {
    fn new() -> Self {
        Self { 
            tasks: None,
            agents: None,
            verbose: Some(true),
            process: Some(Process::Sequential),
            tools: None
        }
    }

    fn tasks(&mut self, tasks: Vec<Task>) -> &mut Self {
        self.tasks.insert(tasks);
        self
    }

    fn agents(&mut self, agents: Vec<Agent>) -> &mut Self {
        self.agents.insert(agents);
        self
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

    fn build(&self) -> Swarm {
        let tasks = self.tasks.as_ref().ok_or(Error::Generic("Tasks are required".to_string())).unwrap().clone();
        let agents = self.agents.as_ref().ok_or(Error::Generic("Agents are required".to_string())).unwrap().clone();
        let process = self.process.as_ref().ok_or(Error::Generic("Process is required".to_string())).unwrap().clone();

        Swarm {
            tasks: tasks.clone(),
            agents: agents.clone(),
            verbose: self.verbose.unwrap_or_default(),
            process: process.clone(),
            tools: self.tools.clone()
        }
    }
}


#[cfg(test)]
#[path = "_tests/swarm.rs"]
mod tests;