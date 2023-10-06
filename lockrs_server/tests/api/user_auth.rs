use hyper::StatusCode;
use lockrs_server::api::v1::responses::{SessionTokenResponse, UserResponse};

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
        test_user.email, test_user.password
    );
    let response = client
        .post(&format!("{}/api/v1/auth/register", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status());

    // Arrange 2: get newly created user id
    let response_body = response
        .text()
        .await
        .expect("Failed extract response body.");

    let new_user = serde_json::from_str::<UserResponse>(&response_body)
        .expect("Failed to parse response body");

    // Act 2: verify user exists
    lockrs_server::services::UserService::get_user_by_id(
        &app.state.db_context,
        &*app.state.repository_container.user_repository,
        &new_user.id,
    )
    .await
    .expect("User not created.");

    // Assert 2
    assert_eq!(test_user.email, new_user.email);
}

#[tokio::test]
async fn user_auth_register_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        (r#"{"password": "password"}"#, "missing the email"),
        (r#"{"email": "name@example.com"}"#, "missing the password"),
        (r#"{}"#, "missing both the email and password"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/api/v1/auth/register", &app.address))
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
    let client = reqwest::Client::new();

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
        let response = client
            .post(&format!("{}/api/v1/auth/register", &app.address))
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
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await; // add to db

    // Act
    let body = format!(
        "{{\"email\": \"{}\", \"password\": \"{}\"}}",
        test_user.email, test_user.password
    );
    let response = client
        .post(&format!("{}/api/v1/auth/register", &app.address))
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
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await; // add to db

    // Act
    let response = client
        .post(&format!("{}/api/v1/auth/login", &app.address))
        .basic_auth(&test_user.email, Some(test_user.password.clone()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status(),);

    // Arrange 2: get response body
    let response_body = response.text().await.expect("Failed to read body.");
    let session_token_response = serde_json::from_str::<SessionTokenResponse>(&response_body)
        .expect("Failed to parse body.");

    let now = chrono::Utc::now().timestamp();

    // Assert 2: confirm body contents
    assert!(session_token_response.expires_at > now);
}

#[tokio::test]
async fn user_auth_login_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await;

    let test_cases = vec![
        (
            String::new(),
            Some(test_user.password.clone()),
            "missing email",
        ),
        (test_user.email.clone(), None, "missing password"),
        (String::new(), None, "missing email and password"),
    ];

    for (email, password, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/api/v1/auth/login", &app.address))
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
    let client = reqwest::Client::new();
    let test_user = TestUser::generate();
    test_user.store(&app).await;

    // Act
    let response = client
        .post(&format!("{}/api/v1/auth/login", app.address))
        .basic_auth(&test_user.email, Some("incorrect_password"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}
