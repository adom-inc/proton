//! A simple wireless access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

/// Access point utilities.
pub mod ap {
    pub use proton_wap::{
        AccessPoint,
        HotspotConfig,
    };
}

/// CIDR network range structure.
pub mod cidr {
    pub use cidr::Ipv4Cidr;
}

/// Device management functionality.
pub mod device {
    pub use proton_dev::Device;
}

/// Error handling functionality.
pub mod error {
    pub use proton_err::{
        ProtonError,
        ProtonResult,
    };
}

/// Native MAC address structure.
pub mod mac {
    pub use proton_mac::MacAddr;
}