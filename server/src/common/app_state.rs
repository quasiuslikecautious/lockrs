use std::sync::Arc;

use crate::{
    pg::repositories::*,
    redis::repositories::*,
    utils::jwt::{JwtUtil, RotatingKey},
    AppConfig, DbContext, RepositoryContainer,
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
        let config = Arc::new(AppConfig::new());

        let key_duration = config.as_ref().key_interval;
        let overlap_duration = config.as_ref().auth_interval;

        let postgres_url = config.as_ref().postgres_url.clone();
        let redis_url = config.as_ref().redis_url.clone();
        let db_context =
            Arc::new(DbContext::new(postgres_url.as_str(), 5, redis_url.as_str(), 5).await);

        let repository_container = RepositoryContainer {
            access_token_repository: Box::new(PgAccessTokenRepository::new(&db_context)),
            authorization_code_repository: Box::new(PgAuthorizationCodeRepository::new(
                &db_context,
            )),
            client_repository: Box::new(PgClientRepository::new(&db_context)),
            device_authorization_repository: Box::new(PgDeviceAuthorizationRepository::new(
                &db_context,
            )),
            redirect_repository: Box::new(PgRedirectUriRepository::new(&db_context)),
            refresh_token_repository: Box::new(PgRefreshTokenRepository::new(&db_context)),
            scope_repository: Box::new(PgScopeRepository::new(&db_context)),
            session_repository: Box::new(RedisSessionRepository::new(&db_context)),
            session_token_repository: Box::new(RedisSessionTokenRepository::new(&db_context)),
            user_repository: Box::new(PgUserRepository::new(&db_context)),
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
