use hyper::StatusCode;
use lockrs_server::{
    api::v1::responses::{SessionTokenResponse, UserResponse},
    utils::jwt::JwtUtil,
};

use crate::common::helpers::{TestApp, TestUser};

#[tokio::test]
async fn user_auth_register_returns_a_200_for_valid_json() {
    // Arrange
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();

    // Act
    let body = format!(
        "{{\"email\": \"{}\", \"password\": \"{}\"}}",
        test_user.get_email(),
        test_user.get_password()
    );
    let response = client
        .post(&format!("{}/api/v1/auth/register", &app.get_address()))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert!(
        !response
            .cookies()
            .any(|cookie| { cookie.name() == JwtUtil::cookie_name() }),
        "Auth cookie should not be sent in response of /register."
    );

    // Arrange 2: get newly created user id
    let new_user = response
        .json::<UserResponse>()
        .await
        .expect("Failed extract response body.");

    // Act 2: verify user exists
    lockrs_server::services::UserService::get_user_by_id(
        &app.get_state().db_context,
        &*app.get_state().repository_container.user_repository,
        &new_user.id,
    )
    .await
    .expect("User not created.");

    // Assert 2
    assert_eq!(test_user.get_email(), new_user.email);
}

#[tokio::test]
async fn user_auth_register_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_cases = vec![
        (r#"{"password": "password"}"#, "missing the email"),
        (r#"{"email": "name@example.com"}"#, "missing the password"),
        (r#"{}"#, "missing both the email and password"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app
            .get_client()
            .post(&format!("{}/api/v1/auth/register", app.get_address()))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            StatusCode::UNPROCESSABLE_ENTITY,
            response.status(),
            "The API did not fail with 422 unprocessable entity when the payload was {}.",
            error_message,
        );
    }
}

#[tokio::test]
async fn user_auth_register_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = TestApp::spawn().await;

    let test_cases = vec![
        (r#"{"email": "", "password": "password"}"#, "empty email"),
        (
            r#"{"email": "name@example.com", "password": ""}"#,
            "empty password",
        ),
        (
            r#"{"email": "not-an-email", "password": "password"}"#,
            "invalid email",
        ),
        (
            r#"{"email": "name@example.com", "password": "a"}"#,
            "invalid password",
        ),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app
            .get_client()
            .post(&format!("{}/api/v1/auth/register", &app.get_address()))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn user_auth_register_returns_a_500_for_existing_user() {
    // Arrange
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate_stored(&app).await;

    // Act
    let body = format!(
        "{{\"email\": \"{}\", \"password\": \"{}\"}}",
        test_user.get_email(),
        test_user.get_password()
    );
    let response = app
        .get_client()
        .post(&format!("{}/api/v1/auth/register", app.get_address()))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
}

#[tokio::test]
async fn user_auth_login_returns_a_200_for_valid_authorization_header() {
    // Arrange
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate_stored(&app).await;

    // Act
    let response = app
        .get_client()
        .post(&format!("{}/api/v1/auth/login", &app.get_address()))
        .basic_auth(
            test_user.get_email(),
            Some(test_user.get_password().to_string()),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert!(
        !response
            .cookies()
            .any(|cookie| { cookie.name() == JwtUtil::cookie_name() }),
        "Auth cookie should not be sent in response of /login."
    );

    // Arrange 2: get response body
    let session_token_response = response
        .json::<SessionTokenResponse>()
        .await
        .expect("Failed to read body.");

    let now = chrono::Utc::now().timestamp();

    // Assert 2: confirm body contents
    assert!(session_token_response.expires_at > now);
}

#[tokio::test]
async fn user_auth_login_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate_stored(&app).await;

    let test_cases = vec![
        (
            String::new(),
            Some(test_user.get_password().to_string()),
            "missing email",
        ),
        (test_user.get_email().to_string(), None, "missing password"),
        (String::new(), None, "missing email and password"),
    ];

    for (email, password, error_message) in test_cases {
        // Act
        let response = app
            .get_client()
            .post(&format!("{}/api/v1/auth/login", app.get_address()))
            .basic_auth(email, password)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            error_message,
        );
    }
}

#[tokio::test]
async fn user_auth_login_returns_a_401_for_invalid_credentials() {
    // Arrange
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate_stored(&app).await;

    // Act
    let response = app
        .get_client()
        .post(&format!("{}/api/v1/auth/login", app.get_address()))
        .basic_auth(test_user.get_email(), Some("incorrect_password"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}
