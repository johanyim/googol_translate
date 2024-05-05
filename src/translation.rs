use crate::error::{self, TranslationError};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;

use serde::Deserialize;

pub const MAX_CHAR_COUNT: usize = 300;
pub const MAX_TOKEN_COUNT: i64 = 300;
pub const API_KEY: &str = std::env!("OPENAI_API_KEY");

#[derive(Deserialize, Debug)]
pub struct TranslationRequest {
    pub text: String,
    pub voice: String,
}

pub fn ai_translate(text: &str, voice: &str) -> Result<String, error::TranslationError> {
    if text.trim().is_empty() | voice.trim().is_empty() {
        return Ok("".to_string());
    }

    if text.len() > MAX_CHAR_COUNT {
        return Err(TranslationError::MaxCharCountExceeded(text.len()));
    }

    let client = Client::new(API_KEY.to_string());
    let prompt = format!(
        r#"Omit disclaimers and surrounding quotation marks in your response. You are a translator for English to "{voice}". You will translate the following text in literal form."#
    );

    // IDEA: Possible to use Logit Bias to control the output of the model to omit disclaimers etc
    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(prompt),
                name: None,
            },
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(text.to_string()),
                name: None,
            },
        ],
    )
    .temperature(0.0) // for deterministic language 1-1 translation
    .seed(0)
    .max_tokens(MAX_TOKEN_COUNT);

    let mut completion = client.chat_completion(req)?;

    let tokens = completion.usage.completion_tokens;
    if tokens > MAX_TOKEN_COUNT as i32 {
        return Err(TranslationError::ResponseMaxTokenCountExceeded(tokens));
    };

    let content = completion.choices[0]
        .message
        .content
        .take()
        .ok_or(TranslationError::NoMessageContent)?;

    Ok(content)
}

//Unit testing
#[cfg(test)]
mod tests {
    use crate::ai_translate;
    use lambda_http::tracing;

    #[test]
    fn empty_input() {
        let text = "";
        let voice = "";
        let openai_req = ai_translate(text, voice);
        assert!(openai_req.is_ok());
        assert!(openai_req.unwrap().is_empty());
    }

    #[test]
    #[ignore] // don't incur request charges on CI testing steps
    fn call_openai() {
        tracing::warn!(
            "This test will incur charges on the OpenAI account, it is better to ignore this test"
        );
        let openai_req = ai_translate(
            "This is the meaning of life, the universe, everything. We are infinite",
            "5 year old",
        );
        println!("{openai_req:#?}");
        assert!(openai_req.is_ok());
    }

    #[test]
    #[ignore] // don't incur request charges on CI testing steps
    fn deterministic_output() {
        tracing::warn!(
            "This test will incur charges on the OpenAI account, it is better to ignore this test"
        );
        let text = "Even bad code can function. But if code isn’t clean, it can bring a development organization to its knees. Every year, countless hours and significant resources are lost because of poorly written code. But it doesn’t have to be that way.";
        let voice = "caveman";
        let response1 = ai_translate(text, voice).ok();
        let response2 = ai_translate(text, voice).ok();

        assert_eq!(response1, response2);
    }
}
