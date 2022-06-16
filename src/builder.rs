//! All the URLs for formatting

use crate::key::{ApiKey, ApiToken};
use crate::{url::new_url, MesowestUrl};

/// Base API URL
pub const BASE: &str = "https://api.synopticdata.com/v2/";

/// Base builder for building URLs
#[derive(Debug)]
#[repr(transparent)]
pub struct MesowestBuilder(String);

/// This is the output of builders that have no further states.
#[derive(Debug)]
#[repr(transparent)]
pub struct FinalBuilder(String);

/// Builder for creating Auth URLs
#[derive(Debug)]
#[repr(transparent)]
pub struct AuthApiBuilder(String);

/// Builder for creating URLs for creating new tokens
#[derive(Debug)]
#[repr(transparent)]
pub struct AuthApiCreateTokenBuilder(String);

impl Default for MesowestBuilder {
    fn default() -> Self {
        MesowestBuilder(BASE.into())
    }
}

impl MesowestBuilder {
    /// Creates a new builder
    pub fn new() -> MesowestBuilder {
        Self::default()
    }

    /// Creates an Auth api builder
    pub fn auth(self, api_key: ApiKey) -> AuthApiBuilder {
        AuthApiBuilder(self.0 + "auth?apikey=" + api_key.as_str())
    }
}

impl FinalBuilder {
    /// Build the URL
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> MesowestUrl {
        new_url(self.0)
    }
}

impl AuthApiBuilder {
    /// Create a new API token
    #[allow(clippy::missing_const_for_fn)]
    pub fn create_token(self) -> AuthApiCreateTokenBuilder {
        AuthApiCreateTokenBuilder(self.0)
    }

    /// List all tokens associatedw with an API key
    pub fn list_tokens(self) -> FinalBuilder {
        FinalBuilder(self.0 + "&list=1")
    }

    /// Delete a token associated with an API key
    pub fn delete_token(self, tok: &ApiToken) -> FinalBuilder {
        FinalBuilder(self.0 + "&disableToken=" + tok.as_str())
    }
}

impl AuthApiCreateTokenBuilder {
    /// Build
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> MesowestUrl {
        new_url(self.0)
    }

    /// If the token should expire, add this.
    pub fn token_expiration(self, expiration: &str) -> FinalBuilder {
        FinalBuilder(self.0 + "&expire=" + expiration)
    }
}

#[cfg(test)]
mod test {
    use super::{ApiKey, ApiToken, MesowestBuilder, BASE};

    #[test]
    fn test_auth() {
        const KEY: &str = "ABC123";
        const TOKEN: &str = "AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH";
        const EXPIRE: &str = "YYYYmmddHHMM";

        let api_key = ApiKey::new(KEY);
        let api_token = ApiToken::new(TOKEN);

        let create = MesowestBuilder::default()
            .auth(api_key)
            .create_token()
            .build();
        let expire = MesowestBuilder::default()
            .auth(api_key)
            .create_token()
            .token_expiration(EXPIRE)
            .build();
        let list = MesowestBuilder::default()
            .auth(api_key)
            .list_tokens()
            .build();
        let delete = MesowestBuilder::default()
            .auth(api_key)
            .delete_token(&api_token)
            .build();

        assert_eq!(
            create.as_str(),
            format!("{}{}{}", BASE, "auth?apikey=", KEY)
        );
        assert_eq!(
            expire.as_str(),
            format!("{}{}{}{}{}", BASE, "auth?apikey=", KEY, "&expire=", EXPIRE)
        );
        assert_eq!(
            list.as_str(),
            format!("{}{}{}{}", BASE, "auth?apikey=", KEY, "&list=1")
        );
        assert_eq!(
            delete.as_str(),
            format!(
                "{}{}{}{}{}",
                BASE,
                "auth?apikey=",
                KEY,
                "&disableToken=",
                api_token.as_str()
            )
        )
    }
}
