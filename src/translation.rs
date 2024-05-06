use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;

use std::env;

pub fn ai_translate(text: &String, voice: String) -> Result<String, Box<dyn std::error::Error>> {
    if text.is_empty() {
        return Ok("".to_string());
    }

    let prompt = format!("Omit all disclaimers. No need for surrounding quotation marks. You are a translator which translates English to \"{voice}\", you will translate the following text: \"{text}\".");

    // let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let client = Client::new("API KEY WAS HERE".to_string());
    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(prompt),
            name: None,
        }],
    )
    .temperature(0.0); // for deterministic language 1-1 translation

    let result = client.chat_completion(req)?;

    let message = result.choices[0].message.content.clone().unwrap();

    // let message = message[1..message.len() - 2].to_string();

    // let message = format!("WIP: response= \"{text}\" in a {voice} voice");
    // let message = format!("{}", text.to_uppercase());

    // let message = format!("{:?}", result);

    Ok(message)
}

#[test]
fn call_openai() {
    let output = ai_translate(
        &"This is the meaning of life, the universe, everything. We are infinite".to_string(),
        "5 year old".to_string(),
    )
    .unwrap();

    println!("OUTPUT: {output:#?}");
}
