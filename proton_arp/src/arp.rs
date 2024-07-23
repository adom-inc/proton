//! Address resolution manager.

use std::{
    time::Duration,
};

use crate::{
    ArpCache,
    ArpCacheIterator,
    ArpResult,
};

/// An address resolution manager that provides ARP caching
/// and requests/response handling.
pub struct ArpManager {
    /// A correspondence between IPv4 addresses and MAC addresses.
    cache: ArpCache,
}

impl ArpManager {
    /// Construct a new ARP manager.
    /// 
    /// # Parameters
    /// - `refresh` (`Duration`): the amount of time after which ARP
    /// cache entries should be refreshed.
    /// 
    /// # Returns
    /// A new `ArpManager` with an empty cache.
    pub fn new(refresh: Duration) -> Self {
        Self {
            cache: ArpCache::new(refresh),
        }
    }

    /// Scan the network and refresh the ARP cache.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `ArpResult` indicating whether or not the scan was successful.
    pub fn scan(&mut self) -> ArpResult {
        todo!()
    }

    /// Refresh the cache without scanning for new devices.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `ArpResult` indicating whether or not the refresh was successful.
    pub fn refresh(&mut self) -> ArpResult {
        todo!()
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