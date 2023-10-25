use std::{net::TcpListener, time::Duration};

use diesel::{pg::Pg, Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lockrs_server::{
    api::v1::{
        models::{SessionModel, UserAuthModel},
        responses::{SessionResponse, SessionTokenResponse},
        services::UserAuthService,
    },
    utils::jwt::JwtUtil,
    AppConfig, AppState,
};
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct TestApp {
    address: String,
    state: AppState,
    client: reqwest::Client,

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
            .unwrap_or_else(|_| panic!("Cannot connect to {} database", &pg_db_name));

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

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_state(&self) -> &AppState {
        &self.state
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn configure_pg(base_url: &str, db_name: &str) {
        let pg_url = format!("{}/postgres", base_url);
        let conn =
            &mut PgConnection::establish(&pg_url).expect("Error connecting to postgres database.");

        diesel::sql_query(format!("CREATE DATABASE {}", db_name))
            .execute(conn)
            .unwrap_or_else(|_| panic!("Could not create database {}.", db_name));
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

        diesel::sql_query(disconnect_users).execute(conn).unwrap();

        diesel::sql_query(format!("DROP DATABASE {};", self.pg_db_name))
            .execute(conn)
            .unwrap_or_else(|_| panic!("Couldn't drop database {}.", self.pg_db_name));
    }
}

/// Because neither reqwest::Response nor reqwest::cookie::Cookie impl Clone, and to access the
/// body of a reqwest response requires a move, we have to manually recreate the cookie
/// ourselves...
pub struct TestCookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub max_age: Option<Duration>,
    pub http_only: bool,
    pub same_site_lax: bool,
    pub same_site_strict: bool,
}

impl From<reqwest::cookie::Cookie<'_>> for TestCookie {
    fn from(value: reqwest::cookie::Cookie) -> Self {
        Self {
            name: value.name().to_string(),
            value: value.value().to_string(),
            path: value.path().map(|v| v.to_string()),
            domain: value.domain().map(|d| d.to_string()),
            max_age: value.max_age(),
            http_only: value.http_only(),
            same_site_lax: value.same_site_lax(),
            same_site_strict: value.same_site_strict(),
        }
    }
}

impl ToString for TestCookie {
    fn to_string(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

pub struct TestUserAuthInfo {
    session: SessionModel,
    cookies: Vec<TestCookie>,
}

impl TestUserAuthInfo {
    pub fn new(session: SessionModel, cookies: Vec<TestCookie>) -> Self {
        Self { session, cookies }
    }

    pub fn get_session(&self) -> &SessionModel {
        &self.session
    }

    pub fn get_session_id(&self) -> &str {
        &self.get_session().id
    }

    pub fn get_user_id(&self) -> &Uuid {
        &self.get_session().user_id
    }

    pub fn get_expires_at(&self) -> &i64 {
        &self.get_session().expires_at
    }

    pub fn get_cookies(&self) -> &Vec<TestCookie> {
        &self.cookies
    }

    pub fn get_auth_cookie(&self) -> Option<&TestCookie> {
        self.get_cookies()
            .iter()
            .find(|cookie| cookie.name == JwtUtil::cookie_name())
    }
}

pub struct TestUser {
    id: Uuid,
    email: String,
    password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            id: Uuid::new_v4(),
            email: format!("{}@example.com", Uuid::new_v4()),
            password: Uuid::new_v4().to_string(),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
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

    pub async fn generate_stored(app: &TestApp) -> Self {
        let user = Self::generate();
        user.store(app).await;
        user
    }

    pub async fn login(&self, app: &TestApp) -> TestUserAuthInfo {
        let session_token = app
            .client
            .post(&format!("{}/api/v1/auth/login", &app.address))
            .basic_auth(&self.email, Some(self.password.as_str()))
            .send()
            .await
            .unwrap_or_else(|_| panic!("Failed to login user {}.", &self.email))
            .json::<SessionTokenResponse>()
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse session token while logging in user {}.",
                    &self.email
                )
            })
            .session_token;

        let session_response = app
            .client
            .post(&format!("{}/api/v1/sessions", &app.address))
            .bearer_auth(session_token)
            .send()
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to exchange token for session while logging in user {}.",
                    &self.email
                )
            });

        let cookies = session_response.cookies().map(TestCookie::from).collect();

        let session_data = session_response
            .json::<SessionResponse>()
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse session while logging in user {}.",
                    &self.email
                )
            });

        let session = SessionModel::new(
            &session_data.id,
            &session_data.user_id,
            session_data.expires_at,
        );

        TestUserAuthInfo::new(session, cookies)
    }

    pub async fn generate_logged_in(app: &TestApp) -> (Self, TestUserAuthInfo) {
        let user = Self::generate_stored(app).await;
        let auth_info = user.login(app).await;
        (user, auth_info)
    }
}
