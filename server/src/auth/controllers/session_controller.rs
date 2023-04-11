use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    auth::requests::NewSessionRequest,
    auth::services::{UserAuthService, UserAuthServiceError}, 
};

pub struct SessionController;

impl SessionController {
    pub async fn read_all(
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}/sessions", user_id))
    }

    pub async fn create(
        Path(user_id): Path<Uuid>,
        Json(new_session): Json<NewSessionRequest>,
    ) -> impl IntoResponse {
        let login_attempt = UserAuthService::login(&new_session.email, &new_session.password);
    
        match login_attempt {
            Ok(login_response) => return Json(login_response).into_response(),
            Err(err) => {
                match err {
                    UserAuthServiceError::NotFoundError => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error proccessing login request").into_response(),
                }
            }
        }
    }

    pub async fn read(
        Path((user_id, session_id)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}/sessions/{}", user_id, session_id))
    }

    pub async fn update(
        Path((user_id, session_id)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}/sessions/{}", user_id, session_id))
    }

    pub async fn delete(
        Path((user_id, session_id)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        let logout_attempt = UserAuthService::logout(&session_id);
    
        match logout_attempt {
            Ok(()) => (StatusCode::OK, "Successfully logged out").into_response(),
            Err(err) => {
                match err {
                    UserAuthServiceError::NotFoundError => return (StatusCode::BAD_REQUEST, "Session token not found").into_response(),
                    _ => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete session token").into_response()
                }
            }
        }
    }
}

