use hyper::StatusCode;
use lockrs_server::api::v1::{services::{SessionTokenService, SessionService}, responses::SessionResponse, models::SessionTokenModel};

use crate::common::helpers::{TestUser, spawn_app};

#[tokio::test]
async fn session_create_returns_a_200_for_valid_bearer_auth() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await;

    let session_token = SessionTokenService::create_session_token(
        &app.state.db_context,
        &*app.state.repository_container.session_token_repository,
        &test_user.id,
    )
    .await
    .expect("Unable to create session token.");

    // Act
    let response = client.post(&format!("{}/api/v1/sessions", &app.address))
        .bearer_auth(&session_token.token)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        StatusCode::OK,
        response.status(),
    );

    // Arrange 2: extract body
    let response_body = response.text().await.expect("Failed to read request body.");
    let session_response = serde_json::from_str::<SessionResponse>(&response_body)
        .expect("Failed to parse body");
    
    // Act 2: verify session exists
    SessionService::get_session(
        &app.state.db_context,
        &*app.state.repository_container.session_repository,
        &test_user.id,
        &session_response.id
    )
    .await
    .expect("Session not created.");

    // Assert 2
    assert_eq!(test_user.id, session_response.user_id);

    // Arrange 3: verify session token has been consumed
    SessionTokenService::validate_session_token(
        &app.state.db_context,
        &*app.state.repository_container.session_token_repository,
        &session_token.token
    )
    .await
    .expect_err("Session token still exists after used.");

    // cleanup
    SessionService::delete_session(
        &app.state.db_context,
        &*app.state.repository_container.session_repository,
        &test_user.id,
    )
    .await
    .expect(&format!("Session {} not deleted.", &session_response.id));

    // cleanup
    test_user.delete(&app).await;
}

#[tokio::test]
async fn session_create_returns_a_400_for_missing_token() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await;

    // Act
    let response = client.post(&format!("{}/api/v1/sessions", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        StatusCode::BAD_REQUEST,
        response.status(),
    );

    test_user.delete(&app).await;
}

#[tokio::test]
async fn session_create_returns_a_401_for_invalid_token() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await;

    let random_token = SessionTokenService::generate_session_token();

    let expired_token = app.state.repository_container.session_token_repository.create(
        &app.state.db_context,
        &SessionTokenModel {
            token: SessionTokenService::generate_session_token(),
            user_id: test_user.id.clone(),
            expires_at: chrono::Utc::now().timestamp() - 1,
        }
    )
    .await
    .expect("Failed to create expired token.")
    .token;

    let used_token = SessionTokenService::create_session_token(
        &app.state.db_context,
        &*app.state.repository_container.session_token_repository,
        &test_user.id,
    )
    .await
    .expect("Failed to create used token.")
    .token;

    SessionTokenService::validate_session_token(
        &app.state.db_context,
        &*app.state.repository_container.session_token_repository,
        &used_token,
    )
    .await
    .expect("Failed to use session token");

    let test_cases = vec![
        (random_token, "invalid random token"),
        (expired_token, "expired token"),
        (used_token, "previously used token"),
    ];

    for (token, error_message) in test_cases {
        // Act
        let response = client.post(&format!("{}/api/v1/sessions", &app.address))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            StatusCode::UNAUTHORIZED,
            response.status(),
            "The API did not fail with 401 Unauthorized when the payload was {}.",
            error_message,
        );
    }

    // cleanup
    test_user.delete(&app).await;
}
