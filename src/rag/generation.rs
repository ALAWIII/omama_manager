use rig::{
    completion::CompletionError,
    streaming::{StreamingPrompt, StreamingResult},
};

use crate::OM_CLIENT;

/// returns a streaming object !
pub async fn generate_response(
    model: &str,
    prompt: &str,
) -> Result<StreamingResult, CompletionError> {
    let agent = OM_CLIENT
        .agent(model)
        .preamble("you are a perfect AI assistant in all disciplines.")
        .build();
    agent.stream_prompt(prompt).await
}
