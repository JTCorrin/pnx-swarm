use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Tool {
    name: String,
    description: String,
    requires_response: bool,
    requires_permission: bool,
    return_direct: bool,
    schema: serde_json::Value,
    func: fn(serde_json::Value) -> Result<serde_json::Value>,
}

impl Tool {
    pub fn new(
        name: &str, 
        description: &str, 
        requires_response: bool, 
        requires_permission: bool, 
        return_direct: bool, 
        schema: serde_json::Value,
        func: fn(serde_json::Value) -> Result<serde_json::Value>
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            requires_response,
            requires_permission,
            return_direct,
            schema,
            func,
        }
    }

    pub fn call(&self, input: serde_json::Value) -> Result<serde_json::Value> {
        (self.func)(input)
    }
}


pub struct ToolBuilder {
    name: Option<String>,
    description: Option<String>,
    requires_response: Option<bool>,
    requires_permission: Option<bool>,
    return_direct: Option<bool>,
    schema: Option<serde_json::Value>,
    func: fn(serde_json::Value) -> Result<serde_json::Value>,
}

impl ToolBuilder {
    pub fn new() -> Self {
        ToolBuilder {
            name: None,
            description: None,
            requires_response: None,
            requires_permission: None,
            return_direct: None,
            schema: None,
            func: |_| Ok(serde_json::Value::Null),
        }
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name.insert(name.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description.insert(description.into());
        self
    }

    pub fn requires_response(&mut self, requires_response: bool) -> &mut Self {
        self.requires_response.insert(requires_response);
        self
    }

    pub fn requires_permission(&mut self, requires_permission: bool) -> &mut Self {
        self.requires_permission.insert(requires_permission);
        self
    }

    pub fn return_direct(&mut self, return_direct: bool) -> &mut Self {
        self.return_direct.insert(return_direct);
        self
    }

    pub fn schema(&mut self, schema: serde_json::Value) -> &mut Self {
        self.schema.insert(schema);
        self
    }

    pub fn func(&mut self, func: fn(serde_json::Value) -> Result<serde_json::Value>) -> &mut Self {
        self.func = func;
        self
    }

    pub fn build(&self) -> Result<Tool> {
        let name = self.name.as_ref().ok_or(Error::Generic("Tool name is required".to_string()))?;
        let description = self.description.as_ref().ok_or(Error::Generic("Tool description is required".to_string()))?;
        let requires_response = self.requires_response.ok_or(Error::Generic("Tool requires_response is required".to_string()))?;
        let requires_permission = self.requires_permission.ok_or(Error::Generic("Tool requires_permission is required".to_string()))?;
        let return_direct = self.return_direct.ok_or(Error::Generic("Tool return_direct is required".to_string()))?;
        let schema = self.schema.as_ref().ok_or(Error::Generic("Tool schema is required".to_string()))?;
        
        Ok(Tool {
            name: name.to_string(),
            description: description.to_string(),
            requires_response,
            requires_permission,
            return_direct,
            schema: schema.clone(),
            func: self.func,
        })
    }
}

#[cfg(test)]
#[path = "_tests/tool.rs"]
mod tests;
