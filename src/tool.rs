use crate::prelude::*;

#[derive(Debug)]
pub struct Tool {
    name: String,
    description: String,
    requires_response: bool,
    requires_permission: bool,
    return_direct: bool,
    schema: Option<serde_json::Value>,
}