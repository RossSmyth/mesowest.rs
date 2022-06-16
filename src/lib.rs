#![deny(
    unsafe_code,
    missing_docs,
    missing_debug_implementations,
    clippy::missing_docs_in_private_items
)]
#![warn(
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::print_stdout,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn
)]
#![doc = include_str!("../README.md")]
pub mod builder;
pub mod key;

mod url;

pub use builder::MesowestBuilder;
pub use url::MesowestUrl;
