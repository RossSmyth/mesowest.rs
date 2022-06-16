//! Module to do with authentification with the API
use crate::{MesowestBuilder, MesowestUrl};
use std::fmt;

/// Object holding the key for accessing auth API calls
/// For general API calls please use or create an `ApiToken` instead.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct ApiKey<'a>(&'a str);

impl<'a> ApiKey<'a> {
    /// Constructs a new Key from a str
    pub const fn new(token: &'a str) -> Self {
        Self(token)
    }

    /// Get a reference to the string slice representing the API key itself.
    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    /// Builds URL for creating a new token
    pub fn build_new_token(&self, expiration: Option<&str>) -> MesowestUrl {
        let nonexpired = MesowestBuilder::default().auth(*self).create_token();

        if let Some(expiration_date) = expiration {
            nonexpired.token_expiration(expiration_date).build()
        } else {
            nonexpired.build()
        }
    }

    /// Builds URL for getting the list of token associated with the key
    pub fn build_get_tokens(&self) -> MesowestUrl {
        MesowestBuilder::default().auth(*self).list_tokens().build()
    }

    /// Builds URL for deleting a token
    pub fn build_delete_token(&self, tok: &ApiToken) -> MesowestUrl {
        MesowestBuilder::default()
            .auth(*self)
            .delete_token(tok)
            .build()
    }

    /// Creates a new token
    /// TODO: Take in optional Timestamp for when it should expire.
    pub fn create_token(&self) -> ApiToken<'a> {
        todo!("How to do web requests & add optional expiration timestamp");
    }

    /// Gets a list of tokens that are associatd with the API key
    pub fn get_tokens(&self) -> Vec<ApiToken<'a>> {
        todo!("How to do web requests");
    }

    /// Deletes a token permanently from the API key
    pub fn delete_token(&self, _tok: ApiToken) {
        todo!("How to do web requests");
    }
}

/// Object for accessing general API calls
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct ApiToken<'a>(&'a str);

impl<'a> ApiToken<'a> {
    /// Constructs a new Token from a str
    pub const fn new(token: &'a str) -> Self {
        Self(token)
    }

    /// Get a reference to the string slice representing the API key itself.
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

/// Will expose your token
impl<'a> fmt::Display for ApiToken<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
