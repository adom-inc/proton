//! Wireless access point utilities for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod ap;

use std::io;

pub use ap::AccessPoint;

/// Result type for access point operations.
pub type AccessPointResult = Result<(), io::Error>;