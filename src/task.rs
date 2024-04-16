use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Task {
    pub requirement: String,
    pub description: String,
    pub expected_outcome: String,
}

impl Task {
    pub fn new(requirement: impl Into<String>, description: impl Into<String>, expected_outcome: impl Into<String>) -> Task {
        Task {
            requirement: requirement.into(),
            description: description.into(),
            expected_outcome: expected_outcome.into(),
        }
    }
}


#[derive(Default, Clone)]
pub struct NoRequirement;

#[derive(Default, Clone)]
pub struct Requirement(String);

#[derive(Default, Clone)]
pub struct NoDescription;

#[derive(Default, Clone)]
pub struct Description(String);

#[derive(Default, Clone)]
pub struct NoExpectedOutcome;

#[derive(Default, Clone)]
pub struct ExpectedOutcome(String);

#[derive(Default)]
pub struct TaskBuilder<R, D, E>{
    requirement: R, // 1 liner task title
    description: D, // More in depth description of the task
    expected_outcome: E, // What is expected to be produced at task completion
}

impl TaskBuilder<NoRequirement, NoDescription, NoExpectedOutcome>{
    pub fn new() -> Self {
        TaskBuilder::default()
    }
}

impl TaskBuilder<Requirement, Description, ExpectedOutcome> {
    pub fn build(&self) -> Result<Task> {
        Ok(Task::new(
            self.requirement.0.clone(),
            self.description.0.clone(),
            self.expected_outcome.0.clone(),
        ))
    }
}

impl<R, D, E> TaskBuilder<R, D, E> {

    pub fn requirement(mut self, requirement: impl Into<String>) -> TaskBuilder<Requirement, D, E> {
        TaskBuilder {
            requirement: Requirement(requirement.into()),
            description: self.description,
            expected_outcome: self.expected_outcome,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> TaskBuilder<R, Description, E> {
        TaskBuilder {
            requirement: self.requirement,
            description: Description(description.into()),
            expected_outcome: self.expected_outcome,
        }
    }
    
    pub fn expected_outcome(mut self, expected_outcome: impl Into<String>) -> TaskBuilder<R, D, ExpectedOutcome> {
        TaskBuilder {
            requirement: self.requirement,
            description: self.description,
            expected_outcome: ExpectedOutcome(expected_outcome.into()),
        }
    }
}



#[cfg(test)]
#[path = "_tests/task.rs"]
mod tests;