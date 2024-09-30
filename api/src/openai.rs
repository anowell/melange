#![allow(unused)]

use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT_4_TURBO: &str = "gpt-4o";
pub const GPT_4_O: &str = "gpt-4o";
pub const GPT_4_O_MINI: &str = "gpt-4o-mini";

#[derive(Clone)]
pub struct OpenAiClient {
    client: Client<OpenAIConfig>,
}

impl Default for OpenAiClient {
    fn default() -> Self {
        let client = Client::new();
        Self { client }
    }
}

impl OpenAiClient {
    pub fn with_base_url(base_url: &str) -> Self {
        let config = OpenAIConfig::new().with_api_base(base_url);
        let client = Client::with_config(config);
        OpenAiClient { client }
    }

    pub fn inner(&self) -> &Client<OpenAIConfig> {
        &self.client
    }

    pub async fn get_chat_response(
        &self,
        model: &str,
        messages: &[ChatCompletionRequestMessage],
        max_tokens: u16,
    ) -> Result<Option<String>, OpenAIError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .max_tokens(max_tokens)
            .build()?;

        // Call API
        let response = self.client.chat().create(request).await?;

        let answer = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone());
        if let Some(answer) = &answer {
            tracing::trace!("GPT Response: {}", answer);
        }
        Ok(answer)
    }
}

pub fn user_message(msg: &str) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::from(msg),
        name: None,
    })
}

pub fn system_message(msg: &str) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
        content: msg.to_string(),
        name: None,
    })
}
