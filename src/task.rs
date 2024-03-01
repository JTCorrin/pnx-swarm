use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Task {
    requirement: String,
    description: String,
}

#[derive(Default)]
pub struct TaskBuilder {
    requirement: Option<String>,
    description: Option<String>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        TaskBuilder::default()
    }

    pub fn requirement(&mut self, requirement: impl Into<String>) -> &mut Self {
        self.requirement.insert(requirement.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description.insert(description.into());
        self
    }

    pub fn build(&self) -> Result<Task> {
        let Some(requirement) = self.requirement.as_ref() else {
            return Err(Error::Generic("Task requirement is required".to_string()));
        };

        let description = self.description.clone().unwrap_or_default();

        Ok(Task {
            requirement: requirement.to_string(),
            description,
        })
    }

}

impl Default for Task {
    fn default() -> Self {
        Self {
            requirement: "Requirement not yet set".to_string(),
            description: "No description".to_string(),
        }
    }

}

impl Task {
    pub fn new(requirement: impl Into<String>, description: impl Into<String>) -> Task {
        Task {
            requirement: requirement.into(),
            description: description.into(),
        }
    }
}


#[cfg(test)]
#[path = "_tests/task.rs"]
mod tests;