mod error;
mod response;
mod translation;

use error::TranslationError;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use response::{htmx_format_unwrap, response_wrap};
use translation::{ai_translate, TranslationRequest};

// infallible monadic handler which converts errors to become log messages
fn request_handler(req: Request) -> Result<String, TranslationError> {
    let body = req.body();

    let TranslationRequest { text, voice } = serde_json::from_slice(body)?;

    let message = ai_translate(&text, &voice)?;

    Ok(message)
}

//At this point, there should not be any error, it would have been formatted into the response
async fn function_handler(req: Request) -> Result<Response<Body>, Error> {
    Ok(response_wrap(htmx_format_unwrap(request_handler(req))))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {}
