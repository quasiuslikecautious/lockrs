use std::sync::Arc;

use crate::{
    db::{pg::repositories::*, redis::repositories::*, DbContext, RepositoryContainer},
    utils::jwt::{JwtUtil, RotatingKey},
    AppConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub jwt_util: Arc<JwtUtil>,
    pub repository_container: Arc<RepositoryContainer>,
    pub db_context: Arc<DbContext>,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppState {{ jwt_util: {:?}, }}", self.jwt_util)
    }
}

impl AppState {
    pub async fn new(config: Option<AppConfig>) -> Self {
        let config = config.unwrap_or_default();

        let key_duration = config.key_interval;
        let overlap_duration = config.auth_interval;
        let key = RotatingKey::new(&key_duration, &overlap_duration);
        let jwt_util = JwtUtil::new(key);

        let postgres_url = config.postgres_url.clone();
        let redis_url = config.redis_url.clone();

        let repository_container = RepositoryContainer {
            access_token_repository: Box::new(PgAccessTokenRepository),
            authorization_code_repository: Box::new(PgAuthorizationCodeRepository),
            client_repository: Box::new(PgClientRepository),
            client_auth_repository: Box::new(PgClientAuthRepository),
            device_authorization_repository: Box::new(PgDeviceAuthorizationRepository),
            redirect_repository: Box::new(PgRedirectUriRepository),
            refresh_token_repository: Box::new(PgRefreshTokenRepository),
            scope_repository: Box::new(PgScopeRepository),
            session_repository: Box::new(RedisSessionRepository),
            session_token_repository: Box::new(RedisSessionTokenRepository),
            user_auth_repository: Box::new(PgUserAuthRepository),
            user_repository: Box::new(PgUserRepository),
        };

        let db_context = DbContext::new(postgres_url.as_str(), 5, redis_url.as_str(), 5).await;

        AppState {
            config,
            jwt_util: Arc::new(jwt_util),
            repository_container: Arc::new(repository_container),
            db_context: Arc::new(db_context),
        }
    }
}
