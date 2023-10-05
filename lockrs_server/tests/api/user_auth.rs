use hyper::StatusCode;
use lockrs_server::api::v1::responses::UserResponse;

use crate::common::helpers::{spawn_app, TestUser};

#[tokio::test]
async fn user_auth_register_returns_a_200_for_valid_json() {
    // Arrange
    let app = spawn_app().await;
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
    .expect("User not created");

    // Assert 2
    assert_eq!(test_user.email, new_user.email);

    // Act 3: clear side effects of test
    lockrs_server::services::UserService::delete_user_by_id(
        &app.state.db_context,
        &*app.state.repository_container.user_repository,
        &new_user.id,
    )
    .await
    .expect(&format!("User {} not deleted", &new_user.id));
}

#[tokio::test]
async fn user_auth_register_returns_a_400_for_invalid_json() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        (r#"{"email": "", "password": "password"}"#, "empty email"),
        (r#"{"email": "name@example.com", "password": ""}"#, "empty password"),
        (r#"{"email": "not-an-email", "password": "password"}"#, "invalid email"),
        (r#"{"email": "name@example.com", "password": "a"}"#, "invalid password"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = client.post(&format!("{}/api/v1/auth/register", &app.address))
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
    let app = spawn_app().await;
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

    // Act 2: clear side effects of test
    lockrs_server::services::UserService::delete_user_by_id(
        &app.state.db_context,
        &*app.state.repository_container.user_repository,
        &test_user.id
    )
    .await
    .expect(&format!("User {} not deleted.", &test_user.id));
}
