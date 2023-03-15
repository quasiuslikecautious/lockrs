use serde::Deserialize;
use url::Url;

pub trait GrantParam {
    fn get_scope(&self) -> String;
}

#[derive(Debug, Deserialize)]
pub struct AuthorizationCodeParams {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: Url, // url to enforce absolute url instead of relative path
    pub code_verifier: String, // require pkce for best practices/draft of oauth 2.1
    pub scope: String,
}

impl GrantParam for AuthorizationCodeParams {
    fn get_scope(&self) -> String {
        self.scope.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientCredentialsParams {
    pub grant_type: String,
    pub scope: String,
}

impl GrantParam for ClientCredentialsParams {
    fn get_scope(&self) -> String {
        self.scope.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeParams {
    pub grant_type: String,
    pub device_code: String,
    pub scope: String,
}

impl GrantParam for DeviceCodeParams {
    fn get_scope(&self) -> String {
        self.scope.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenParams {
    pub grant_type: String,
    pub refresh_token: String,
    pub scope: String,
}

impl GrantParam for RefreshTokenParams {
    fn get_scope(&self) -> String {
        self.scope.clone()
    }
}

