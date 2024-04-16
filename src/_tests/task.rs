#[cfg(test)]
mod tests {
    use crate::task::{TaskBuilder, Task};

    #[test]
    fn test_task_builder() {
        let task = TaskBuilder::new()
            .requirement("Requirement")
            .description("Description")
            .expected_outcome("Expected Outcome")
            .build()
            .unwrap();

        assert_eq!(task.requirement, "Requirement");
        assert_eq!(task.description, "Description");
        assert_eq!(task.expected_outcome, "Expected Outcome");
    }

    #[test]
    fn test_task_new() {
        let task = Task::new("Requirement", "Description", "Expected Outcome");

        assert_eq!(task.requirement, "Requirement");
        assert_eq!(task.description, "Description");
    }
}