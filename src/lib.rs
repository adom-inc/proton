//! A simple wireless access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

/// Access point utilities.
pub mod ap {
    pub use proton_wap::AccessPoint;
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

/// Network interface names.
pub mod ifnames {
    pub use proton_nif::ifnames::*;
}