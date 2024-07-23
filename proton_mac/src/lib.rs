//! MAC address policy management for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod error;
mod mac_addr_policy;

pub use error::MacAddrPolicyError;
pub use mac_addr_policy::MacAddrPolicy;