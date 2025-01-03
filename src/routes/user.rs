use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use super::AppState;
use crate::db;
use crate::models::User;

pub const DEFAULT_PROMPT: &str = r#"The following is a structured and deep conversation between a human and an AI psychologist.
The AI psychologist is empathetic, insightful, and uses ideas from Jordan Peterson's psychological frameworks and the Self-Authoring program.
The AI's goal is to help the human achieve greater clarity, personal growth, and an understanding of their values, goals, and narratives.
The AI provides specific exercises, asks thought-provoking questions, and gives practical advice where appropriate.
If the AI does not have enough context to answer fully, it encourages further reflection or gathering more information.

Guiding principles for the AI psychologist:
1. **Empathy and Validation**: Acknowledge the emotional and psychological state of the human with warmth and understanding.
2. **Narrative Focus**: Help the human identify and refine their personal narrative, connecting past, present, and future into a coherent story.
3. **Goal Clarification**: Encourage the human to define and structure their goals in alignment with their values.
4. **Cognitive Restructuring**: Gently challenge distorted thinking patterns and suggest healthier alternatives.
5. **Practical Exercises**: Provide structured writing exercises, reflection prompts, or actionable steps inspired by the Self-Authoring program.
6. **Accountability**: Motivate the human to take responsibility for their actions and their role in shaping their life.


make sure to answer as if you were jordan peterson

Conversation history (detailed):
{history}
Human: {input}
AI Psychologist:
"#;

// --------------------------------------------------
// DTOs
// --------------------------------------------------
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub summary: Option<String>,
}

#[derive(Deserialize)]
pub struct PromptUpdateRequest {
    pub new_prompt: String,
}

#[derive(Serialize)]
pub struct SummaryResponse {
    pub summary: String,
}

// --------------------------------------------------
// Handlers
// --------------------------------------------------

/// POST /login
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let pool = &state.pool;

    // 1) Create or retrieve user
    let user = db::get_or_create_user(pool, &payload.username, DEFAULT_PROMPT)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2) Grab existing summary
    let summary = db::get_conversation_summary(pool, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        user_id: user.id,
        summary,
    }))
}

/// GET /prompt/:user_id
pub async fn get_prompt_handler(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<String, StatusCode> {
    let pool = &state.pool;
    let user = db::get_user_by_id(pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => Ok(u.prompt.unwrap_or_default()),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /prompt/:user_id
pub async fn update_prompt_handler(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PromptUpdateRequest>,
) -> Result<Json<UpdatePromptResponse>, StatusCode> {
    let pool = &state.pool;
    let updated_ok = db::update_user_prompt(pool, user_id, &payload.new_prompt)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if updated_ok {
        Ok(Json(UpdatePromptResponse {
            prompt: payload.new_prompt,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Serialize)]
pub struct UpdatePromptResponse {
    pub prompt: String,
}

/// GET /summary/:user_id
pub async fn get_summary_handler(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<SummaryResponse>, StatusCode> {
    let pool = &state.pool;
    let summary_opt = db::get_conversation_summary(pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match summary_opt {
        Some(summary) => Ok(Json(SummaryResponse { summary })),
        None => Err(StatusCode::NOT_FOUND),
    }
}
