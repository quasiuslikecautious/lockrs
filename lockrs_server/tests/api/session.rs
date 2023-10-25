use hyper::StatusCode;
use lockrs_server::{
    api::v1::{
        models::{SessionModel, SessionTokenModel},
        responses::SessionResponse,
        services::{SessionService, SessionTokenService},
    },
    utils::jwt::JwtUtil,
};
use serde_json::json;

use crate::common::helpers::{TestApp, TestUser};

#[tokio::test]
async fn session_create_returns_a_200_for_valid_bearer_auth() {
    // Arrange
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate_stored(&app).await;
    let session_token = SessionTokenService::create_session_token(
        &app.get_state().db_context,
        &*app
            .get_state()
            .repository_container
            .session_token_repository,
        test_user.get_id(),
    )
    .await
    .expect("Unable to create session token.");

    // Act
    let response = app
        .get_client()
        .post(&format!("{}/api/v1/sessions", &app.get_address()))
        .bearer_auth(&session_token.token)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status(),);
    assert!(
        response
            .cookies()
            .any(|cookie| { cookie.name() == JwtUtil::cookie_name() && cookie.value() != "" }),
        "Creating a session should send a non-empty session cookie in the response."
    );

    // Arrange 2: extract body
    let session_response = response
        .json::<SessionResponse>()
        .await
        .expect("Failed to read request body.");

    // Act 2: verify session exists
    SessionService::get_session(
        &app.get_state().db_context,
        &*app.get_state().repository_container.session_repository,
        test_user.get_id(),
        &session_response.id,
    )
    .await
    .expect("Session not created.");

    // Assert 2
    assert_eq!(*test_user.get_id(), session_response.user_id);

    // Arrange 3: verify session token has been consumed
    SessionTokenService::validate_session_token(
        &app.get_state().db_context,
        &*app
            .get_state()
            .repository_container
            .session_token_repository,
        &session_token.token,
    )
    .await
    .expect_err("Session token still exists after used.");

    // cleanup
    SessionService::delete_session(
        &app.get_state().db_context,
        &*app.get_state().repository_container.session_repository,
        test_user.get_id(),
    )
    .await
    .unwrap_or_else(|_| panic!("Session {} not deleted.", &session_response.id));
}

#[tokio::test]
async fn session_create_returns_a_400_for_missing_token() {
    // Arrange
    let app = TestApp::spawn().await;
    let _test_user = TestUser::generate_stored(&app).await;

    // Act
    let response = app
        .get_client()
        .post(&format!("{}/api/v1/sessions", app.get_address()))
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
    let test_user = TestUser::generate_stored(&app).await;
    let random_token = SessionTokenService::generate_session_token();
    let expired_token = app
        .get_state()
        .repository_container
        .session_token_repository
        .create(
            &app.get_state().db_context,
            &SessionTokenModel {
                token: SessionTokenService::generate_session_token(),
                user_id: *test_user.get_id(),
                expires_at: chrono::Utc::now().timestamp() - 1,
            },
        )
        .await
        .expect("Failed to create expired token.")
        .token;

    let used_token = SessionTokenService::create_session_token(
        &app.get_state().db_context,
        &*app
            .get_state()
            .repository_container
            .session_token_repository,
        test_user.get_id(),
    )
    .await
    .expect("Failed to create used token.")
    .token;

    SessionTokenService::validate_session_token(
        &app.get_state().db_context,
        &*app
            .get_state()
            .repository_container
            .session_token_repository,
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
            .get_client()
            .post(&format!("{}/api/v1/sessions", app.get_address()))
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
async fn session_update_returns_a_200_for_valid_no_refresh() {
    // Arrange
    let app = TestApp::spawn().await;
    let (_test_user, auth_info) = TestUser::generate_logged_in(&app).await;
    let original_expiry = auth_info.get_expires_at();

    // Act
    let response = app
        .get_client()
        .put(&format!(
            "{}/api/v1/sessions/{}",
            app.get_address(),
            auth_info.get_session_id()
        ))
        .json(&json!({ "refresh": false }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert 1
    assert_eq!(StatusCode::OK, response.status());

    // Arrange 2
    let session_response = response
        .json::<SessionModel>()
        .await
        .expect("Unable to parse response from /session/update.");

    let session_stored = app
        .get_state()
        .repository_container
        .session_repository
        .get_by_hash(
            &app.get_state().db_context,
            auth_info.get_session_id(),
            auth_info.get_user_id(),
        )
        .await
        .expect("Failed to find session in repository");

    // Assert 2: expiry is unchanged in response and in stored value
    assert_eq!(*original_expiry, session_response.expires_at);
    assert_eq!(session_response.expires_at, session_stored.expires_at);
}

#[tokio::test]
async fn session_update_returns_a_200_for_valid_refresh() {
    // Arrange
    let app = TestApp::spawn().await;
    let (_test_user, auth_info) = TestUser::generate_logged_in(&app).await;
    let original_expiry = auth_info.get_expires_at();

    // Act
    let response = app
        .get_client()
        .put(&format!(
            "{}/api/v1/sessions/{}",
            app.get_address(),
            &auth_info.get_session().id
        ))
        .json(&json!({ "refresh": true }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status());

    // Arrange 2
    let session_response = response
        .json::<SessionModel>()
        .await
        .expect("Unable to parse response from /session/update.");

    let session_stored = app
        .get_state()
        .repository_container
        .session_repository
        .get_by_hash(
            &app.get_state().db_context,
            auth_info.get_session_id(),
            auth_info.get_user_id(),
        )
        .await
        .expect("Failed to find session in repository");

    // Assert 2
    assert!(*original_expiry < session_response.expires_at);
    assert_eq!(session_response.expires_at, session_stored.expires_at);
}

#[tokio::test]
async fn session_delete_returns_a_200_for_valid_id() {
    // Arrange
    let app = TestApp::spawn().await;
    let (_, auth_info) = TestUser::generate_logged_in(&app).await;

    // Act
    let response = app
        .get_client()
        .delete(&format!(
            "{}/api/v1/sessions/{}",
            app.get_address(),
            auth_info.get_session_id()
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::NO_CONTENT, response.status());
}

#[tokio::test]
async fn session_returns_a_404_for_invalid_authorization() {
    // Arrange
    let app = TestApp::spawn().await;
    let (_, auth_info) = TestUser::generate_logged_in(&app).await;
    TestUser::generate_logged_in(&app).await; // login with new user to overwrite auth info

    let test_cases = vec![
        (reqwest::Method::GET, "GET"),
        (reqwest::Method::PUT, "PUT"),
        (reqwest::Method::DELETE, "DELETE"),
    ];

    for (method, error_message) in test_cases {
        // Act
        let response = app
            .get_client()
            .request(
                method,
                &format!(
                    "{}/api/v1/sessions/{}",
                    app.get_address(),
                    auth_info.get_session_id()
                ),
            )
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            StatusCode::NOT_FOUND,
            response.status(),
            "The API did not fail with 404 Not Found when the method was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn session_returns_a_404_for_invalid_session() {
    // Arrange
    let app = TestApp::spawn().await;
    TestUser::generate_logged_in(&app).await; // generate valid auth info to avoid unauthorized
                                              // error conflict
    let test_methods = vec![
        (reqwest::Method::GET, "GET"),
        (reqwest::Method::PUT, "PUT"),
        (reqwest::Method::DELETE, "DELETE"),
    ];

    let test_cases = vec![
        (String::new(), "no session provided"),
        (
            SessionService::generate_session_id(),
            "random invalid session",
        ),
    ];

    for (method, method_message) in test_methods {
        for (session_id, error_message) in test_cases.clone() {
            // Act
            let response = app
                .get_client()
                .request(
                    method.clone(),
                    &format!("{}/api/v1/sessions/{}", app.get_address(), session_id),
                )
                .send()
                .await
                .expect("Failed to execute request.");

            // Assert
            assert_eq!(
                StatusCode::NOT_FOUND,
                response.status(),
                "The API did not fail with 404 Not Found when the method was {} and payload was {}.",
                method_message,
                error_message
            );
        }
    }
}

#[tokio::test]
async fn expired_session_is_unauthorized() {
    // Arrange
    let app = TestApp::spawn().await;
    let (_, auth_info) = TestUser::generate_logged_in(&app).await;

    let expired_session_model = SessionModel::new(
        auth_info.get_session_id(),
        auth_info.get_user_id(),
        chrono::Utc::now().timestamp_millis(),
    );

    app.get_state()
        .repository_container
        .session_repository
        .update(&app.get_state().db_context, &expired_session_model)
        .await
        .expect("Failed to expire session");

    // Act
    let response = app
        .get_client()
        .get(format!(
            "{}/api/v1/sessions/{}",
            app.get_address(),
            auth_info.get_session_id()
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}

#[tokio::test]
async fn max_one_session_exists_per_user() {
    // Arrange
    let app = TestApp::spawn().await;

    let (test_user, auth_info) = TestUser::generate_logged_in(&app).await;
    test_user.login(&app).await; // overwrite client auth info with new session

    // Act
    let response = app
        .get_client()
        .get(&format!(
            "{}/api/v1/{}",
            app.get_address(),
            auth_info.get_session_id()
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
