//! A simple wireless access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

/// Proton error handling functionality.
pub mod error {
    pub use proton_err::ProtonError;
}

/// Proton access point utilities.
pub mod ap {
    pub use proton_wap::AccessPoint;
}