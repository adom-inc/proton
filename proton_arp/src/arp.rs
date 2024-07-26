//! Address resolution manager.

use std::net::Ipv4Addr;

use cidr::Ipv4Cidr;

use pnet::datalink::MacAddr;

use proton_err::ProtonResult;

use crate::{
    ArpCache,
    ArpCacheIterator,
    scan,
};

/// An address resolution manager that provides ARP caching
/// and ARP request/reply handling.
pub struct ArpManager {
    /// The CIDR range of the network
    range: Ipv4Cidr,

    /// A correspondence between IPv4 addresses and MAC addresses.
    cache: ArpCache,

    /// The name of the network interface to be scanned.
    ifname: String,
}

impl ArpManager {
    /// Construct a new ARP manager.
    /// 
    /// # Parameters
    /// - `range` (`Ipv4Cidr`): the CIDR range of the network
    /// - `ifname` (`&str`): the name of the network interface
    /// 
    /// # Returns
    /// A new `ArpManager` with an empty cache.
    pub fn new(range: Ipv4Cidr, ifname: &str) -> Self {
        Self {
            range,
            cache: ArpCache::new(),
            ifname: ifname.to_string(),
        }
    }

    /// Scan the network and refresh the ARP cache.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// None.
    pub async fn scan(&mut self) -> ProtonResult<()> {
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
        self.cache.set(scan(addresses, &self.ifname).await?);

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

    /// Look up a MAC address, returning its corresponding IPv4 if available.
    /// 
    /// # Parameters
    /// - `mac` (`MacAddr`): the MAC address to look up
    /// 
    /// # Returns
    /// An `Option<Ipv4Addr>` containing to the provided MAC address, if available.
    pub fn lookup_mac(&self, mac: MacAddr) -> Option<Ipv4Addr> {
        for entry in self.cache() {
            if entry.mac == mac {
                return Some (entry.ipv4);
            }
        }

        None
    }
}