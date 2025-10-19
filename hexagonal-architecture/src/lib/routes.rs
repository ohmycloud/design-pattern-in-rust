use std::sync::Arc;

use anyhow::{Context, anyhow};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sqlite: Arc<SqlitePool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateAuthorRequestBody {
    name: String,
}

/// Generic response structure shared by all API response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

pub struct ApiSuccess<T: Serialize>((StatusCode, Json<ApiResponseBody<T>>));
pub type ErrorResponseBody = ApiResponseBody<ApiErrorData>;
pub type ErrorResponse = (StatusCode, ErrorResponseBody);

impl<T: Serialize> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        Self((status, Json(ApiResponseBody::new(status, data))))
    }
}

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(anyhow::Error),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            ApiError::UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateAuthorResponseData {
    id: String,
}

const UNIQUE_CONSTRAINT_VIOLATION: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION {
                return true;
            }
        }
    }
    false
}

async fn save_author(tx: &mut Transaction<'_, Sqlite>, name: &str) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let id_as_string = id.to_string();
    let query = sqlx::query!(
        "INSERT INTO authors (id, name) VALUES ($1, $2)",
        id_as_string,
        name
    );

    tx.execute(query).await?;
    Ok(id)
}

pub async fn create_author(
    State(state): State<AppState>,
    Json(author): Json<CreateAuthorRequestBody>,
) -> Result<ApiSuccess<CreateAuthorResponseData>, ApiError> {
    if author.name.is_empty() {
        return Err(ApiError::UnprocessableEntity(
            "author name cannot be empty".to_string(),
        ));
    }

    let mut tx = state
        .sqlite
        .begin()
        .await
        .context("failed to start transaction")?;

    let author_id = save_author(&mut tx, &author.name).await.map_err(|e| {
        if is_unique_constraint_violation(&e) {
            ApiError::UnprocessableEntity(format!(
                "author with name {} already exists",
                &author.name
            ))
        } else {
            anyhow!(e).into()
        }
    })?;

    tx.commit().await.context("failed to commit transaction")?;

    Ok(ApiSuccess::new(
        StatusCode::CREATED,
        CreateAuthorResponseData {
            id: author_id.to_string(),
        },
    ))
}
