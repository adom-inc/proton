//! Network scanning for the ARP manager.

use std::net::Ipv4Addr;

use tokio::task;

use crate::ArpCacheEntry;

/// Scan the provided list of IPv4 addresses and return all ARP responses.
/// 
/// # Parameters
/// - `ips` (`Vec<Ipv4Addr>`): the IPv4 addresses to scan
/// 
/// # Returns
/// A `Vec<ArpCacheEntry>` containing the ARP responses received.
pub async fn scan(ips: Vec<Ipv4Addr>) -> Vec<ArpCacheEntry> {
    // Begin listening for ARP replies
    let rx_task = task::spawn(request(ips));

    // Begin making ARP requests
    let tx_task = task::spawn(listen());

    let (_rx_result, _tx_result) = tokio::join!(rx_task, tx_task);

    todo!()
}

/// Make a series of ARP requests to the provided IPv4 addresses.
/// 
/// # Parameters
/// - `ips` (`Ipv4Addr`): the IPv4 addresses to scan
/// 
/// # Returns
/// TODO
pub async fn request(_ips: Vec<Ipv4Addr>) {
    todo!()
}

/// Receive a series of ARP replies.
///
/// # Parameters
/// TODO
/// 
/// # Returns
/// TODO
pub async fn listen() {
    todo!()
}