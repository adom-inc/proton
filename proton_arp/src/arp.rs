//! Address resolution manager.

use std::{
    time::Duration,
};

use cidr::Ipv4Cidr;

use crate::{
    ArpCache,
    ArpCacheIterator,
    ArpResult,
    scan,
};

/// An address resolution manager that provides ARP caching
/// and ARP request/reply handling.
pub struct ArpManager {
    /// The CIDR range of the network
    range: Ipv4Cidr,

    /// A correspondence between IPv4 addresses and MAC addresses.
    cache: ArpCache,
}

impl ArpManager {
    /// Construct a new ARP manager.
    /// 
    /// # Parameters
    /// - `range` (`Ipv4Cidr`): the CIDR range of the network
    /// - `refresh` (`Duration`): the amount of time after which ARP
    /// cache entries should be refreshed.
    /// 
    /// # Returns
    /// A new `ArpManager` with an empty cache.
    pub fn new(range: Ipv4Cidr, refresh: Duration) -> Self {
        Self {
            range,
            cache: ArpCache::new(refresh),
        }
    }

    /// Scan the network and refresh the ARP cache.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// None.
    pub async fn scan(&mut self) -> ArpResult {
        let mut addresses = Vec::new();

        // Assemble list of addresses to be scanned
        for ipv4 in self.range.iter().addresses() {
            // Skip network address and broadcast address
            if ipv4 != self.range.first_address()
                && ipv4 != self.range.last_address()
            {
                addresses.push(ipv4);
            }
        }

        // Scan the network and update the cache
        self.cache.set(scan(addresses).await?);

        Ok (())
    }

    /// Refresh the cache without scanning for new devices.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `ArpResult` indicating whether or not the refresh was successful.
    pub async fn refresh(&mut self) -> ArpResult {
        let addresses = self.cache.get_stale_ips();

        // Scan the network and update the cache
        self.cache.set(scan(addresses).await?);

        Ok (())
    }

    /// Get an iterator of the cache, without consuming the cache.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `ArpCacheIterator` over all IPv4 addresses and their corresponding MAC addresses.
    pub fn cache(&self) -> ArpCacheIterator {
        self.cache.clone().into_iter()
    }
}