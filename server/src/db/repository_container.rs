use crate::db::repositories::*;

pub struct RepositoryContainer {
    pub access_token_repository: Box<dyn AccessTokenRepository>,
    pub authorization_code_repository: Box<dyn AuthorizationCodeRepository>,
    pub client_repository: Box<dyn ClientRepository>,
    pub device_authorization_repository: Box<dyn DeviceAuthorizationRepository>,
    pub redirect_repository: Box<dyn RedirectUriRepository>,
    pub refresh_token_repository: Box<dyn RefreshTokenRepository>,
    pub scope_repository: Box<dyn ScopeRepository>,
    pub session_repository: Box<dyn SessionRepository>,
    pub session_token_repository: Box<dyn SessionTokenRepository>,
    pub user_repository: Box<dyn UserRepository>,
}
