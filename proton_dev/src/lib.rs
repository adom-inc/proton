//! Device discovery for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod device;
mod manager;
mod socket;

use neli::err::NlError;

pub use device::Device;

pub use manager::DeviceManager;

pub use socket::NetworkSocket;

/// Result type for Netlink operations.
pub type NetlinkResult<T> = Result<T, NlError>;