use std::net::TcpListener;

use diesel::{pg::Pg, Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lockrs_server::{
    api::v1::{
        models::{UserAuthModel, UserLoginCredentials, SessionModel},
        responses::{SessionResponse, SessionTokenResponse},
        services::UserAuthService,
    },
    AppConfig, AppState,
};
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct TestApp {
    pub address: String,
    pub state: AppState,
    pub client: reqwest::Client,

    pg_base_url: String,
    pg_db_name: String,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
        let port = listener.local_addr().unwrap().port();

        // configure pg db for test
        let pg_base_url = String::from("postgres://postgres@localhost");
        let pg_db_name = format!("lockrs_test_{}", Uuid::new_v4().as_simple());

        Self::configure_pg(&pg_base_url, &pg_db_name);
        let postgres_url = format!("{}/{}", pg_base_url, pg_db_name);

        let conn = &mut PgConnection::establish(&postgres_url)
            .expect(&format!("Cannot connect to {} database", &pg_db_name));

        Self::run_migrations(conn);

        let test_config = AppConfig {
            postgres_url,
            redis_url: String::from("redis://localhost:6379"),
            auth_interval: chrono::Duration::minutes(10),
            key_interval: chrono::Duration::minutes(11),
        };

        let state = AppState::new(Some(test_config)).await;

        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .expect("Failed to build http client.");

        let server = lockrs_server::run(listener, Some(state.clone()))
            .await
            .expect("Failed to bind address.");

        let _ = tokio::spawn(server);

        TestApp {
            address: format!("http://127.0.0.1:{}", port),
            state,
            client,
            pg_base_url,
            pg_db_name,
        }
    }

    fn configure_pg(base_url: &str, db_name: &str) {
        let pg_url = format!("{}/postgres", base_url);
        let conn =
            &mut PgConnection::establish(&pg_url).expect("Error connecting to postgres database.");

        diesel::sql_query(&format!("CREATE DATABASE {}", db_name))
            .execute(conn)
            .expect(&format!("Could not create database {}.", db_name));
    }

    fn run_migrations(conn: &mut impl MigrationHarness<Pg>) {
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations.");
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        let pg_url = format!("{}/postgres", self.pg_base_url);
        let conn =
            &mut PgConnection::establish(&pg_url).expect("Error connecting to postgres database.");

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = '{}';",
            self.pg_db_name
        );

        diesel::sql_query(&disconnect_users).execute(conn).unwrap();

        diesel::sql_query(&format!("DROP DATABASE {};", self.pg_db_name))
            .execute(conn)
            .expect(&format!("Couldn't drop database {}.", self.pg_db_name));
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

        let user_auth = UserAuthModel::new(&self.id, self.email.as_str(), password_hash.as_str());

        app.state
            .repository_container
            .user_auth_repository
            .create_raw(&app.state.db_context, &user_auth)
            .await
            .expect("Failed to store test user in user database.");
    }

    pub async fn login(&self, app: &TestApp) -> SessionModel {
        let session_token = app
            .client
            .post(&format!("{}/api/v1/auth/login", &app.address))
            .basic_auth(&self.email, Some(self.password.as_str()))
            .send()
            .await
            .expect(&format!("Failed to login user {}.", &self.email))
            .json::<SessionTokenResponse>()
            .await
            .expect(&format!(
                "Failed to parse session token while logging in user {}.",
                &self.email
            ))
            .session_token;

        let session_response = app.client
            .post(&format!("{}/api/v1/sessions", &app.address))
            .bearer_auth(session_token)
            .send()
            .await
            .expect(&format!(
                "Failed to exchange token for session while logging in user {}.",
                &self.email
            ))
            .json::<SessionResponse>()
            .await
            .expect(&format!(
                "Failed to parse session while logging in user {}.",
                &self.email
            ));

        SessionModel::new(
            &session_response.id,
            &session_response.user_id,
            session_response.expires_at
        )
    }
}
