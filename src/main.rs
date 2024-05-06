mod translation;
use translation::ai_translate;

use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

use serde::Deserialize;
use serde_json;
use serde_qs;

// #[derive(Debug, PartialEq, Deserialize, Serialize)]
#[derive(Deserialize, Debug)]
struct TranslationRequest {
    text: String,
    voice: String,
}

async fn function_handler(req: Request) -> Result<Response<Body>, Error> {
    // let (_, body) = req.into_parts();
    let body = req.body();

    println!("[BODY RECEIVED]: \n{body:?}");

    // let TranslationRequest { text, voice } = serde_qs::from_bytes(&body)?;
    let TranslationRequest { text, voice } = serde_json::from_slice(&body)?;
    println!("[DESERIALIZED]: \n text: {text}, voice: {voice}.");

    let message = ai_translate(&text, voice).unwrap_or(text);

    let resp = Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "http://googoltranslate.com")
        // .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS") // Specify allowed methods
        // .header(
        //     "Access-Control-Allow-Headers",
        //     "Content-Type, Authorization",
        // ) // Specify allowed headers
        // .header("Access-Control-Allow-Credentials", "true") // Specify allowed headers
        .header("Access-Control-Max-Age", "86400") // Specify allowed headers
        .header("Content-Type", "text/plain")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
