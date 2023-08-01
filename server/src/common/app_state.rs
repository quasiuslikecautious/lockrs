use std::sync::Arc;

use crate::{
    db::{pg::repositories::*, redis::repositories::*, DbContext, RepositoryContainer},
    utils::jwt::{JwtUtil, RotatingKey},
    AppConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub jwt_util: Arc<JwtUtil>,
    pub repository_container: Arc<RepositoryContainer>,
    pub db_context: Arc<DbContext>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = Arc::new(AppConfig::default());

        let key_duration = config.as_ref().key_interval;
        let overlap_duration = config.as_ref().auth_interval;

        let postgres_url = config.as_ref().postgres_url.clone();
        let redis_url = config.as_ref().redis_url.clone();
        let db_context =
            Arc::new(DbContext::new(postgres_url.as_str(), 5, redis_url.as_str(), 5).await);

        let repository_container = RepositoryContainer {
            access_token_repository: Box::new(PgAccessTokenRepository),
            authorization_code_repository: Box::new(PgAuthorizationCodeRepository),
            client_repository: Box::new(PgClientRepository),
            device_authorization_repository: Box::new(PgDeviceAuthorizationRepository),
            redirect_repository: Box::new(PgRedirectUriRepository),
            refresh_token_repository: Box::new(PgRefreshTokenRepository),
            scope_repository: Box::new(PgScopeRepository),
            session_repository: Box::new(RedisSessionRepository),
            session_token_repository: Box::new(RedisSessionTokenRepository),
            user_repository: Box::new(PgUserRepository),
        };

        Self {
            config,
            jwt_util: Arc::new(JwtUtil {
                secret: RotatingKey::new(&key_duration, &overlap_duration),
            }),
            repository_container: Arc::new(repository_container),
            db_context: Arc::clone(&db_context),
        }
    }
}
