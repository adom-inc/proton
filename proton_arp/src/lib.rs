//! Address Resolution Protocol (ARP) management for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod arp;
mod cache;
mod error;
mod scan;

use std::time::Duration;

pub use arp::ArpManager;

pub use cache::{
    ArpCache,
    ArpCacheEntry,
    ArpCacheIterator,
};

pub use error::{
    ArpError,
};

pub use scan::scan;

/// Result type for ARP management actions.
pub type ArpResult = Result<(), ArpError>;

/// Result type for ARP network scans.
type ScanResult = Result<Vec<ArpCacheEntry>, ArpError>;

/// The default ARP cache entry refresh time (2 minutes).
pub static DEFAULT_ARP_REFRESH_TIME: Duration = Duration::from_secs(120);