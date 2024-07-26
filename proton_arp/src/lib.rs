//! Address Resolution Protocol (ARP) management for the Proton access point management library.

#![deny(warnings)]
#![deny(missing_docs)]

mod arp;
mod cache;
mod scan;

use std::time::Duration;

pub use arp::ArpManager;

pub use cache::{
    ArpCache,
    ArpCacheEntry,
    ArpCacheIterator,
};

pub use scan::scan;

/// The default ARP cache entry refresh time (2 minutes).
pub static DEFAULT_ARP_REFRESH_TIME: Duration = Duration::from_secs(120);