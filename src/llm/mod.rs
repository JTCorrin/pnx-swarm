pub mod ollama;
pub mod open_ai;
pub mod transport;

use crate::prelude::*;

pub trait LLMClone {
    fn clone_box(&self) -> Box<dyn LLM>;
}

impl<T> LLMClone for T
where
    T: 'static + LLM + Clone,
{
    fn clone_box(&self) -> Box<dyn LLM> {
        Box::new(self.clone())
    }
}

pub trait LLM: LLMClone {
    fn call(&self, message: String) -> Result<String>;
}

impl Clone for Box<dyn LLM> {
    fn clone(&self) -> Box<dyn LLM> {
        self.clone_box()
    }
}


// States
#[derive(Default, Clone)]
pub struct NoModelName;

#[derive(Default, Clone)]
pub struct ModelName(String);

#[derive(Default, Clone)]
pub struct NoAPIKey;

#[derive(Default, Clone)]
pub struct APIKey(String);