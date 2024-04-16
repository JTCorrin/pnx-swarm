
#[cfg(test)]
mod tests {
    use serde_derive::Deserialize;

    use crate::transport::Transport;
    use crate::message::{Message, MessageBuilder};
    use crate::error::Error;



    #[derive(Deserialize, Debug)]
    struct LocalResponse {
        model: String,
        created_at: String,
        message: Message,
        done: bool,
        total_duration: u32,
        load_duration: u32,
        prompt_eval_count: Option<u32>,
        prompt_eval_duration: u32,
        eval_count: u32,
        eval_duration: u32
    }

    #[tokio::test]
    async fn test_transport_can_make_calls() {
        let transport = Transport::new();
        let sys_message = MessageBuilder::new()
            .role("system")
            .content("You are a helpful assistant")
            .build()
            .unwrap();

        let user_message = MessageBuilder::new()
            .role("user")
            .content("Hello, how are you?")
            .build()
            .unwrap();

        let messages = vec![sys_message, user_message];

        let output: LocalResponse = transport.post(
            "http://192.168.1.73:11434/api/chat".to_string(), 
            "mistral-openorca".to_string(), 
            messages, 
            "api_key".to_string()
        ).await.unwrap();

        println!("{:?}", output);

        assert_eq!(output.model, "mistral-openorca");
    }

}   
