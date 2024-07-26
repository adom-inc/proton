//! ARP caching utilities.

use std::{
    vec::IntoIter,
    net::Ipv4Addr,
    time::{
        Duration,
        Instant,
    },
};

use proton_mac::MacAddr;

#[derive(Clone)]
/// An address resolution cache.
/// 
/// This cache stores IPv4 addresses, their corresponding MAC addresses, and
/// the time that they were cached.
pub struct ArpCache {
    /// Cache entries consisting of IPv4 addresses, MAC addresses, and the times of caching.
    cache: Vec<ArpCacheEntry>,
}

impl ArpCache {
    #[allow(clippy::new_without_default)]
    /// Construct a new ARP cache.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new, empty `ArpCache`.
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
        }
    }

    /// Add an entry to the ARP cache.
    ///
    /// # Parameters
    /// - `ipv4` (`Ipv4Addr`): the IPv4 address of the device
    /// - `mac` (`MacAddr`): the MAC address of the device
    /// 
    /// # Returns
    /// None.
    pub fn add(&mut self, ipv4: Ipv4Addr, mac: MacAddr) {
        // Create a cache entry
        let entry = ArpCacheEntry::new(ipv4, mac);

        // Add the entry to the cache
        self.cache.push(entry);
    }

    /// Update the ARP cache.
    /// 
    /// # Parameters
    /// - `cache` (`Vec<ArpCacheEntry>`): a new cache list
    /// 
    /// # Returns
    /// None.
    pub fn set(&mut self, cache: Vec<ArpCacheEntry>) {
        self.cache = cache;
    }
}

impl IntoIterator for ArpCache {
    type Item = ArpCacheEntry;
    type IntoIter = ArpCacheIterator;

    fn into_iter(self) -> Self::IntoIter {
        ArpCacheIterator::from(self.cache)
    }
}

/// An iterator over all values in the ARP cache.
pub struct ArpCacheIterator {
    /// IPv4 addresses and their corresponding MAC addresses.
    iter: IntoIter<ArpCacheEntry>,
}

impl ArpCacheIterator {
    /// Construct an iterator from an ARP cache.
    /// 
    /// # Parameters
    /// - `cache` (`Vec<ArpCacheEntry>`): the ARP cache list
    ///
    /// # Returns
    /// A new `ArpCacheIterator`.
    pub fn from(cache: Vec<ArpCacheEntry>) -> Self {
        Self {
            iter: cache.into_iter(),
        }
    }
}

impl Iterator for ArpCacheIterator {
    type Item = ArpCacheEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Clone, Copy)]
/// An address resolution cache entry.
/// 
/// An ARP cache entry stores a device's IPv4 and MAC addresses as well as the time
/// that the entry was created.
pub struct ArpCacheEntry {
    /// The IPv4 address of the device.
    pub ipv4: Ipv4Addr,

    /// The MAC address of the device.
    pub mac: MacAddr,

    /// The time that this entry was created.
    created: Instant,
}

impl ArpCacheEntry {
    /// Construct a new ARP cache entry.
    /// 
    /// # Parameters
    /// - `ipv4` (`Ipv4Addr`): the IPv4 address of the device
    /// - `mac` (`MacAddr`): the MAC address of the device
    /// 
    /// # Returns
    /// A new `ArpCacheEntry` corresponding to the provided MAC address.
    pub fn new(ipv4: Ipv4Addr, mac: MacAddr) -> Self {
        Self {
            ipv4,
            mac,
            created: Instant::now(),
        }
    }

    /// Check if this entry needs to be refreshed (as of call time).
    /// 
    /// # Parameters
    /// - `refresh` (`Duration`): the amount of time after which this entry
    /// should be refreshed.
    /// 
    /// # Returns
    /// A `bool` indicating whether or not this entry should be refreshed.
    pub fn check(&self, refresh: Duration) -> bool {
        // Check the time
        let now = Instant::now();

        now - self.created >= refresh
    }
}