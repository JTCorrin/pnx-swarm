use regex::Regex;
use serde::de::DeserializeOwned;

use crate::{llm::{self, LLM}, message::{Message, MessageBuilder}, task::Task, prelude::*};

#[derive(Debug)]
pub struct Plan {
    steps: Vec<String>
}

#[derive(Clone)]
pub struct Planner {
    llm: LLM
}

impl Planner {

    pub fn new(llm: LLM) -> Self {
        Planner {
            llm
        }
    }

    // TODO now I can use the standard response to integrate parse
    pub async fn create_plan(&self, task: Task) -> Result<Vec<String>> {
        // TODO Include the use of tools
        let message = "You've been given a task which must be completed. \
        Let's first understand the task and devise a plan to successfully complete it. \
        Your instructions for devising the plan are as follows: \
        1. The plan should be a numbered list of steps (much like these instructions) \
        2. The plan should be no more than the minimum number of steps required to complete the task \
        3. The plan should be the steps required to complete the task, not the actual completion of the task \
        3. Write only the plan at this time (a numbered list of steps). Do not include any other information or commentary";

        let system_message = MessageBuilder::new()
            .role("system")
            .content(message)
            .build()
            .unwrap();

        let message = format!("Task: {}\nDescription: {}\nExpected Outcome: {}", task.requirement, task.description, task.expected_outcome);
        
        let user_message = MessageBuilder::new()
            .role("user")
            .content(message)
            .build()
            .unwrap();


        // Create messages and pass to llm 
        let mut messages = vec![system_message, user_message];

        // Get response from llm
        let res = self.llm.call(messages).await.unwrap();

        let parsed_plan = self.parse_plan(res.message.content.trim().to_string());

        Ok(parsed_plan)

    }

    fn parse_plan(&self, plan: String) -> Vec<String> {
        let re = Regex::new(r"\d+\.\s*").unwrap();
        let steps: Vec<&str> = re.split(plan.as_str())
                              .filter(|&x| !x.is_empty()) // Filter out any empty strings
                              .collect();
        steps.into_iter().map(|s| s.trim().to_string()).collect()
    }
}


#[cfg(test)]
#[path ="_tests/planner.rs"]
mod tests;