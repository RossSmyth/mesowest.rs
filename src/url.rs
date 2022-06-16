//! Module for the object that represents all Mesowest URLs

/// The Mesowest URL
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
#[repr(transparent)]
pub struct MesowestUrl(String);

/// Crate private constructor for MesowestURL
pub const fn new_url(input: String) -> MesowestUrl {
    MesowestUrl(input)
}

impl MesowestUrl {
    /// Returns a reference to the str representing the URL
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
