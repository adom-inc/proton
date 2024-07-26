//! Device discovery for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod device;
mod manager;
mod socket;

pub use device::Device;

pub use manager::DeviceManager;

pub use socket::NetworkSocket;