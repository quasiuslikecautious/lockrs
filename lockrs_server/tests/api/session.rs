use hyper::StatusCode;
use lockrs_server::api::v1::{
    models::{SessionTokenModel, SessionModel},
    responses::SessionResponse,
    services::{SessionService, SessionTokenService},
};
use uuid::Uuid;

use crate::common::helpers::{TestApp, TestUser};

#[tokio::test]
async fn session_create_returns_a_200_for_valid_bearer_auth() {
    // Arrange
    let app = TestApp::spawn().await;

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
    let response = app
        .client
        .post(&format!("{}/api/v1/sessions", &app.address))
        .bearer_auth(&session_token.token)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status(),);

    // Arrange 2: extract body
    let session_response = response
        .json::<SessionResponse>()
        .await
        .expect("Failed to read request body.");

    // Act 2: verify session exists
    SessionService::get_session(
        &app.state.db_context,
        &*app.state.repository_container.session_repository,
        &test_user.id,
        &session_response.id,
    )
    .await
    .expect("Session not created.");

    // Assert 2
    assert_eq!(test_user.id, session_response.user_id);

    // Arrange 3: verify session token has been consumed
    SessionTokenService::validate_session_token(
        &app.state.db_context,
        &*app.state.repository_container.session_token_repository,
        &session_token.token,
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
}

#[tokio::test]
async fn session_create_returns_a_400_for_missing_token() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user = TestUser::generate();
    test_user.store(&app).await;

    // Act
    let response = app
        .client
        .post(&format!("{}/api/v1/sessions", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::BAD_REQUEST, response.status(),);
}

#[tokio::test]
async fn session_create_returns_a_401_for_invalid_token() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user = TestUser::generate();
    test_user.store(&app).await;

    let random_token = SessionTokenService::generate_session_token();

    let expired_token = app
        .state
        .repository_container
        .session_token_repository
        .create(
            &app.state.db_context,
            &SessionTokenModel {
                token: SessionTokenService::generate_session_token(),
                user_id: test_user.id.clone(),
                expires_at: chrono::Utc::now().timestamp() - 1,
            },
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
        let response = app
            .client
            .post(&format!("{}/api/v1/sessions", &app.address))
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
}

#[tokio::test]
async fn session_delete_returns_a_200_for_valid_id() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user = TestUser::generate();
    test_user.store(&app).await;
    let auth_info = test_user.login(&app).await;

    // Act
    let response = app
        .client
        .delete(&format!("{}/api/v1/sessions/{}", &app.address, &auth_info.id))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        StatusCode::NO_CONTENT,
        response.status()
    );
}

#[tokio::test]
async fn session_delete_returns_a_404_for_invalid_authorization() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user_1 = TestUser::generate();
    test_user_1.store(&app).await;
    let auth_info_1 = test_user_1.login(&app).await; // create session for user 1
    
    let test_user_2 = TestUser::generate();
    test_user_2.store(&app).await;
    test_user_2.login(&app).await; // overwrite client auth info with new user 

    // Act
    let response = app
        .client
        .delete(&format!("{}/api/v1/sessions/{}", &app.address, &auth_info_1.id))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        StatusCode::NOT_FOUND,
        response.status()
    );
}

#[tokio::test]
async fn session_delete_returns_a_404_for_invalid_session() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user = TestUser::generate();
    test_user.store(&app).await;
    test_user.login(&app).await;

    let test_cases = vec![
        (String::new(), "no session provided"),
        (SessionService::generate_session_id(), "random invalid session"),
    ];

    for (session_id, error_message) in test_cases {
        // Act
        let response = app
            .client
            .get(&format!("{}/api/v1/sessions/{}", &app.address, session_id))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            StatusCode::NOT_FOUND,
            response.status(),
            "The API did not fail with 404 Not Found when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn expired_session_is_unauthorized() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user = TestUser::generate();
    test_user.store(&app).await;
    let auth_info = test_user.login(&app).await;

    let expired_session_model = SessionModel::new(
        &auth_info.id,
        &auth_info.user_id,
        chrono::Utc::now().timestamp_millis(),
    );

    app.state.repository_container.session_repository.update(
        &app.state.db_context,
        &expired_session_model
    )
    .await
    .expect("Failed to expire session");

    // Act
    let response = app
        .client
        .get(format!("{}/api/v1/sessions/{}", &app.address, &auth_info.id))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        StatusCode::UNAUTHORIZED,
        response.status()
    );
}

#[tokio::test]
async fn max_one_session_exists_per_user() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_user = TestUser::generate();
    test_user.store(&app).await;
    let auth_info_1 = test_user.login(&app).await; // create session for user 1
    test_user.login(&app).await; // overwrite client auth info with new session 

    // Act
    let response = app
        .client
        .get(&format!("{}/api/v1/{}", &app.address, &auth_info_1.id))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        StatusCode::NOT_FOUND,
        response.status()
    );
}
