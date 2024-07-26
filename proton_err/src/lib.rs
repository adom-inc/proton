//! Error handling for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod error;

pub use error::ProtonError;

/// Result type for access point operations.
pub type ProtonResult<T> = Result<T, ProtonError>;