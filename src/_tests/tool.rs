#[cfg(test)]
mod tests {
    use crate::tool::{ToolBuilder, Tool};
    use crate::error::Error;

    #[test]
    fn test_tool_new() {
        let tool = Tool::new(
            "Test Tool", 
            "This is a test tool", 
            true, 
            true,
            true, 
            serde_json::json!({"type": "object"}),
            |_| Ok(serde_json::Value::Null)
        );

        assert_eq!(tool.name, "Test Tool");
        assert_eq!(tool.description, "This is a test tool");
        assert_eq!(tool.requires_response, true);
        assert_eq!(tool.requires_permission, true);
        assert_eq!(tool.return_direct, true);
    }

    #[test]
    fn test_tool_builder() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .description("This is a test tool")
            .requires_response(true)
            .requires_permission(true)
            .return_direct(true)
            .schema(serde_json::json!({"type": "object"}))
            .func(|_| Ok(serde_json::Value::Null))
            .build()
            .unwrap();
        assert_eq!(tool.name, "Test Tool");
        assert_eq!(tool.description, "This is a test tool");
        assert_eq!(tool.requires_response, true);
        assert_eq!(tool.requires_permission, true);
        assert_eq!(tool.return_direct, true);
    }

    #[test]
    fn test_tool_builder_missing_name() {
        let tool = ToolBuilder::new()
            .description("This is a test tool")
            .requires_response(true)
            .requires_permission(true)
            .return_direct(true)
            .func(|_| Ok(serde_json::Value::Null))
            .build();

        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_builder_missing_description() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .requires_response(true)
            .requires_permission(true)
            .return_direct(true)
            .func(|_| Ok(serde_json::Value::Null))
            .build();

        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_builder_missing_requires_response() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .description("This is a test tool")
            .requires_permission(true)
            .return_direct(true)
            .func(|_| Ok(serde_json::Value::Null))
            .build();

        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_builder_missing_requires_permission() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .description("This is a test tool")
            .requires_response(true)
            .return_direct(true)
            .func(|_| Ok(serde_json::Value::Null))
            .build();

        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_builder_missing_return_direct() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .description("This is a test tool")
            .requires_response(true)
            .requires_permission(true)
            .func(|_| Ok(serde_json::Value::Null))
            .build();

        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_builder_missing_func() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .description("This is a test tool")
            .requires_response(true)
            .requires_permission(true)
            .return_direct(true)
            .build();

        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_builder_missing_all() {
        let tool = ToolBuilder::new().build();
        assert!(tool.is_err());
    }

    #[test]
    fn test_tool_call_func() {
        let tool = ToolBuilder::new()
            .name("Test Tool")
            .description("This is a test tool")
            .requires_response(true)
            .requires_permission(true)
            .return_direct(true)
            .schema(serde_json::json!({"type": "object"}))
            .func(|input| {
                Ok(input)
            })
            .build()
            .unwrap();

        let input = serde_json::json!({"test": "test"});
        let result = tool.call(input.clone()).unwrap();
        println!("{}", result);
        assert_eq!(result, input);
    }
}