//! MAC address policy errors.

use std::{
    error::Error,
    fmt,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
/// A MAC address policy error.
pub enum MacAddrPolicyError {
    /// The user tried to whitelist a device on a non-whitelist policy.
    NotWhitelistPolicy,

    /// The user tried to blacklist a device on a non-blacklist policy.
    NotBlacklistPolicy,
}

impl fmt::Display for MacAddrPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::NotWhitelistPolicy => "cannot whitelist device on a non-whitelist policy",
            Self::NotBlacklistPolicy => "cannot blacklist device on a non-blacklist policy",
        };

        write!(f, "{}", error)
    }
}

impl Error for MacAddrPolicyError { }