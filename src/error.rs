use openai_api_rs::v1::error::APIError;
use serde_json::error::Error as SerdeError;
use thiserror::Error;

use crate::translation;

use translation::{MAX_CHAR_COUNT, MAX_TOKEN_COUNT};

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Could not deserialize payload to proper request type")]
    SerializationError(#[from] SerdeError),
    #[error("Crate http response builder failed somewhere: {0}")]
    BuildingResponseError(#[from] lambda_http::http::Error),
    #[error("The API key used to generate the response was not found.")]
    NoAPIKeyFound,
    #[error("")]
    ChatCompletionCallError(#[from] APIError),
    #[error("The response from the server contained nothing")]
    NoMessageContent,
    #[error("You have exceeded the maximum character count {0}/{MAX_CHAR_COUNT}")]
    MaxCharCountExceeded(usize),
    #[error("The response returned is too long, {0}/{MAX_TOKEN_COUNT}")]
    ResponseMaxTokenCountExceeded(i32),
}

#[allow(clippy::useless_format)]
impl TranslationError {
    pub fn show_details(&self) -> String {
        match self {
            TranslationError::SerializationError(err) => {
                format!("The incoming request to the server could not be deserialized. {err}")
            }
            TranslationError::BuildingResponseError(err) => {
                format!("The implementation's response builder failed in its process. {err}")
            }
            TranslationError::NoAPIKeyFound => {
                format!("Could not load environment variable: \"$OPENAI_API_KEY\"")
            }
            TranslationError::ChatCompletionCallError(err) => {
                format!("The OpenAI API had an error: {}", err.message)
            }
            TranslationError::NoMessageContent => {
                format!("The response to the translation request came back empty.")
            }

            TranslationError::ResponseMaxTokenCountExceeded(tokens) => format!(
                "The number of tokens in this translation request was exceeded ({}/{})",
                tokens, MAX_TOKEN_COUNT
            ),

            TranslationError::MaxCharCountExceeded(chars) => format!(
                "The number of characters in this translation request was exceeded ({}/{})",
                chars, MAX_CHAR_COUNT
            ),
        }
    }
}
