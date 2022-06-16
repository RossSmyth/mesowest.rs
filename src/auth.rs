//! Module to do with authentification with the API
use std::fmt;

/// Object holding the key for accessing auth API calls
/// For general API calls please use or create an `ApiToken` instead.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
#[repr(transparent)]
pub struct ApiKey<'a>(&'a str);

impl<'a> ApiKey<'a> {
    /// Constructs a new Key from a str
    pub const fn new(token: &'a str) -> Self {
        Self(token)
    }

    /// Get a reference to the string slice representing the API key itself.
    pub const fn get_str(self) -> &'a str {
        self.0
    }
}

/// Display trait returns "SECRET" unconditionally. If you really want to expose
/// your API key, then get the string slice with `get_str`
impl<'a> fmt::Display for ApiKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SECRET")
    }
}

/// Object for accessing general API calls
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
#[repr(transparent)]
pub struct ApiToken<'a>(&'a str);

impl<'a> ApiToken<'a> {
    /// Constructs a new Token from a str
    pub const fn new(token: &'a str) -> Self {
        Self(token)
    }

    /// Get a reference to the string slice representing the API key itself.
    pub const fn get_str(self) -> &'a str {
        self.0
    }
}

impl<'a> fmt::Display for ApiToken<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
