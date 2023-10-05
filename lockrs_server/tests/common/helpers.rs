use std::net::TcpListener;

use lockrs_server::{
    api::v1::{models::UserAuthModel, services::UserAuthService},
    AppState,
};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub state: AppState,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let state = AppState::new().await;

    let server = lockrs_server::run(listener, Some(state.clone()))
        .await
        .expect("Failed to bind address.");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        state,
    }
}

pub struct TestUser {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            id: Uuid::new_v4(),
            email: format!("{}@example.com", Uuid::new_v4()),
            password: Uuid::new_v4().to_string(),
        }
    }

    pub async fn store(&self, app: &TestApp) {
        let password_hash = UserAuthService::hash_password(self.password.as_str())
            .expect("Failed to hash password of test user.");

        let user_auth =
            UserAuthModel::new(&self.id, self.email.as_str(), password_hash.as_str());

        app.state
            .repository_container
            .user_auth_repository
            .create_raw(&app.state.db_context, &user_auth)
            .await
            .expect("Failed to store test user in user database.");
    }
}
