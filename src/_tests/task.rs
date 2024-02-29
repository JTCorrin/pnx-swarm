#[cfg(test)]
mod tests {
    use crate::task::{TaskBuilder, Task};

    #[test]
    fn test_task_builder() {
        let task = TaskBuilder::new()
            .requirement("Requirement")
            .description("Description")
            .build()
            .unwrap();

        assert_eq!(task.requirement, "Requirement");
        assert_eq!(task.description, "Description");
    }

    #[test]
    fn test_task_default() {
        let task = Task::default();

        assert_eq!(task.requirement, "Requirement not yet set");
        assert_eq!(task.description, "No description");
    }

    #[test]
    fn test_task_new() {
        let task = Task::new("Requirement", "Description");

        assert_eq!(task.requirement, "Requirement");
        assert_eq!(task.description, "Description");
    }
}