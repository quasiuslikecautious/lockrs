
use axum::{
    Json,
    extract::Path, 
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    models::NewUser,
    auth::responses::UserResponse,
    services::{UserService, UserServiceError},
};

pub struct UserController;

impl UserController {
    pub async fn create(
        Json(new_user): Json<NewUser>,
    ) -> impl IntoResponse {
        let login_attempt = UserService::create_user(new_user);
    
        match login_attempt {
            Ok(user) => {
                let signup_response = UserResponse {
                    id: user.id,
                    email: user.email,
                };
    
                Json(signup_response).into_response()
            },
    
            Err(err) => {
                match err {
                    UserServiceError::AlreadyExistsError => (StatusCode::CONFLICT, "An account is already associated with that email address").into_response(),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error proccessing signup request").into_response(),
                }
            }
        }
    }
    
    pub async fn read(
        Path(user_id): Path<Uuid>
    ) -> impl IntoResponse {
        let user = UserService::get_user_by_id(&user_id);
    
        match user {
            Ok(user) => (StatusCode::OK, Json(UserResponse {
                id: user.id,
                email: user.email,
            })).into_response(),
    
            Err(err) => {
                match err {
                    UserServiceError::NotFoundError => (StatusCode::NOT_FOUND, "No user with that id found").into_response(),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unable to access user").into_response(),
                }
            }
        }
    }

    pub async fn update(
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}", user_id))
    }

    pub async fn delete(
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}", user_id))
    }
}

