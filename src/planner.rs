use crate::{llm::{ollama::Ollama, LLM}, message::{Message, MessageBuilder}, task::Task};

#[derive(Clone)]
pub struct Planner {
    llm: Box<dyn LLM>,
    system_message: Message,
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner {
    pub fn new() -> Self {

        // TODO Include the use of tools
        let message = "You've been given a task which must be completed. \
        Let's first understand the task and devise a plan to successfully complete it. \
        Your instructions for devising the plan are as follows: \
        1. Start the with the header 'Plan:' \
        2. The plan should be a numbered list of steps (much like these instructions) \
        3. The plan should be no more than the minimum number of steps required to complete the task \
        4. Write only the plan at this time. Do not include any other information or commentary";

        let system_message = MessageBuilder::new()
            .role("system")
            .content(message)
            .build()
            .unwrap();

        Planner {
            llm: Box::new(Ollama::default()),
            system_message
        }
    }

    pub fn create_plan(&self, task: Task) -> Vec<String> {
        let message = format!("Task: {}\nDescription: {}\nExpected Outcome: {}", task.requirement, task.description, task.expected_outcome);
        let user_message = MessageBuilder::new()
            .role("user")
            .content(message)
            .build()
            .unwrap();


        // Create messages and pass to llm 
        let mut messages = vec![self.system_message.clone(), user_message];

        // Get response from llm
        let response = self.llm.call(messages).unwrap();

        // Parse response
        self.parse_plan(response)
    }

    fn parse_plan(&self, plan: String) -> Vec<String> {
        plan.split("\n").map(|s| s.to_string()).collect()
    }
}
