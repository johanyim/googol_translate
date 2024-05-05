use crate::error::TranslationError;
use lambda_http::{Body, Response};

// WARN: Do not use reqwest crate StatusCode. See Cargo.toml
const STATUS_OK: u16 = 200;
const DOMAIN_URL: &str = "http://googoltranslate.com";
const MAX_AGE: u32 = 86400;

pub fn response_wrap(message: String) -> Response<Body> {
    Response::builder()
        .status(STATUS_OK)
        .header("Access-Control-Allow-Origin", DOMAIN_URL)
        .header("Access-Control-Max-Age", &MAX_AGE.to_string()) // Specify allowed headers
        .header("Content-Type", "text/plain")
        .body(message.into())
        .map_err(TranslationError::BuildingResponseError)
        .unwrap()
}

pub fn htmx_format_unwrap(result: Result<String, TranslationError>) -> String {
    //htmx formatted string
    match result {
        Ok(message) => message,
        Err(err) => format!(
            r#"<details class="error">
  <summary class="error">{err}</summary>
  {err:#?}: {}
</details>"#,
            err.show_details()
        ),
    }
}
