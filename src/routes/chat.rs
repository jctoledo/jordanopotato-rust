use axum::{extract::State, http::StatusCode, Json};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

use openai_rust::{
    chat::{ChatArguments, Message},
    Client as OpenAIClient,
};

use super::{user::DEFAULT_PROMPT, AppState};
use crate::{db, models::User};

pub const OPENAI_MODEL_VERSION: &str = "gpt-4o";

#[derive(Deserialize)]
pub struct MessageRequest {
    pub message: String,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub reply: String,
    pub user_id: i32,
}

/// POST /chat
pub async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<MessageRequest>,
) -> Result<Json<MessageResponse>, StatusCode> {
    let pool = &state.pool;

    // 1) Ensure user exists
    let user_opt = db::get_user_by_id(pool, req.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = match user_opt {
        Some(u) => u,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // 2) Current summary from DB
    let existing_summary = db::get_conversation_summary(pool, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or_default();

    // 3) Use user’s custom prompt or fallback
    let user_prompt = user.prompt.as_deref().unwrap_or(DEFAULT_PROMPT);

    // 4) Build the conversation text for the AI
    let conversation_text = format!(
        r#"{user_prompt}

Conversation so far (summarized):
{existing_summary}

User's new message:
"{user_message}"

Assistant, please respond:
"#,
        user_prompt = user_prompt,
        existing_summary = existing_summary,
        user_message = req.message
    );

    // 5) Create the OpenAI client
    dotenv().ok();
    let api_key = env::var("MY_OPENAI_KEY").unwrap_or_default();
    let client = OpenAIClient::new(&api_key);

    // 6) Ask the AI for the assistant reply
    let chat_args = ChatArguments::new(
        OPENAI_MODEL_VERSION,
        vec![Message {
            role: "system".to_owned(),
            content: conversation_text,
        }],
    );

    let res = client
        .create_chat(chat_args)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let llm_reply = match res.choices.get(0) {
        Some(choice) => choice.message.content.clone(),
        None => "(No reply)".to_owned(),
    };

    // 7) Summarize the new exchange
    let summarization_text = format!(
        r#"
Previous summary:
{old_summary}

User's latest message:
"{user_message}"

Assistant's reply:
"{assistant_reply}"

Please provide an updated very detailed summary of these contents.
No less than 10 sentences, use detail
No more than 40 sentences if needed


If repeated themes start occring use this into consideration for your response:
"#,
        old_summary = existing_summary,
        user_message = req.message,
        assistant_reply = llm_reply
    );

    let summ_args = ChatArguments::new(
        OPENAI_MODEL_VERSION,
        vec![Message {
            role: "system".to_owned(),
            content: summarization_text,
        }],
    );

    let summ_res = client
        .create_chat(summ_args)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_summary = match summ_res.choices.get(0) {
        Some(choice) => choice.message.content.clone(),
        None => existing_summary, // fallback
    };

    // 8) Store updated summary in DB
    db::update_conversation_summary(pool, user.id, &new_summary)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 9) Return the AI’s reply
    Ok(Json(MessageResponse {
        reply: llm_reply,
        user_id: user.id,
    }))
}
