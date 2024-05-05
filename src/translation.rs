use llm::Model;

fn load_model() -> Result<llm::models::Llama, Box<dyn std::error::Error>> {
    let llama = llm::load::<llm::models::Llama>(
        std::path::Path::new("./models/open_llama_3b-f16.bin"),
        // llm::ModelParameters
        Default::default(),
        // load progress callback
        llm::load_progress_callback_stdout,
    )?;

    return Ok(llama);
}

pub fn ai_translate(text: String, voice: String) -> String {
    if text.is_empty() {
        return "".to_string();
    }

    let llama = llm::load::<llm::models::Llama>(
        std::path::Path::new("./models/open_llama_3b-f16.bin"),
        // llm::ModelParameters
        Default::default(),
        // load progress callback
        llm::load_progress_callback_stdout,
    )
    .unwrap();

    // TODO:
    let message = format!("WIP: response= \"{text}\" in a {voice} voice");
    // let message = format!("{}", text.to_uppercase());
    message
}
