//! Network Interface Card (NIC) abstraction for the Proton access point management library.

#[deny(warnings)]
#[deny(missing_docs)]

mod nif;

pub use nif::NetworkInterface;

#[cfg(all(feature = "cndn", feature = "ethx"))]
compile_error!("features `cndn` and `ethx` are mutually exclusive");

#[cfg(not(any(feature = "cndn", feature = "ethx")))]
compile_error!("you must select exactly one of features `cndn` and `ethx`");

// Feature `cndn` should be used on machines that use Consistent Network Device Naming.
#[cfg(feature = "cndn")]
pub mod ifnames {
    /// The default wireless network interface to use.
    pub const DEFAULT_WIRELESS_INTERFACE: &str = "wlp4s0";

    /// The default wired network interface to use.
    pub const DEFAULT_WIRED_INTERFACE: &str = "eno1";
}

// Feature `ethx` should be used on machines that use the old `ethX` network interface naming system.
#[cfg(feature = "ethx")]
pub mod ifnames {
    /// The default wireless network interface to use.
    pub const DEFAULT_WIRELESS_INTERFACE: &str = "wlan0";

    /// The default wired network interface to use.
    pub const DEFAULT_WIRED_INTERFACE: &str = "eth0";
}