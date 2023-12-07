use url::Url;

use super::secret::Secret;

#[derive(Clone, serde::Deserialize, Debug, Default)]

pub struct AuthConfiguration {
    pub handler: AuthHandlers,
}

#[derive(Clone, serde::Deserialize, Debug, Default, PartialEq)]
#[serde(tag = "type")]
pub enum AuthHandlers {
    #[default]
    #[serde(rename = "no_op")]
    NoOp,
    #[serde(rename = "oauth2")]
    OAuth2(Oauth2Configuration),
}

fn default_callback() -> String {
    "/auth/callback".to_string()
}

fn default_scopes() -> Vec<String> {
    vec!["openid".to_string()]
}

#[derive(Clone, serde::Deserialize, Debug, PartialEq)]
pub struct Oauth2Configuration {
    pub base_url: Url,
    #[serde(default = "default_callback")]
    pub callback: String,
    pub client_id: String,
    pub client_secret: Secret<String>,
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    pub well_known_url: Url,
}
