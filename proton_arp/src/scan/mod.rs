//! Network scanning utility for the ARP manager.

mod reply;
mod request;

use std::{
    net::Ipv4Addr,
    time::Duration,
};

use tokio::{
    sync::mpsc,
    task,
};

use proton_err::ProtonResult;

use proton_nif::{
    ifnames::DEFAULT_WIRELESS_INTERFACE,
    NetworkInterface,
};

use crate::ArpCacheEntry;

use reply::listen;
use request::request;

/// Buffer size for the asynchronous communication channel for ARP replies.
pub const ARP_CHANNEL_BUFFER_SIZE: usize = 256;

/// Default delay to wait before closing the ARP reply listener.
pub static ARP_LISTENER_DELAY: Duration = Duration::from_millis(2_500);

/// Scan the provided list of IPv4 addresses and return all ARP replies.
/// 
/// # Parameters
/// - `ips` (`Vec<Ipv4Addr>`): the IPv4 addresses to scan
/// 
/// # Returns
/// A `ProtonResult<Vec<ArpCacheEntry>>` containing the ARP responses
/// received, if the scan was successful.
pub async fn scan(ips: Vec<Ipv4Addr>) -> ProtonResult<Vec<ArpCacheEntry>> {
    // Get the wireless network interface
    let interface = NetworkInterface::new(DEFAULT_WIRELESS_INTERFACE)?;

    // Create an asynchronous communication channel for received replies
    let (reply_tx, reply_rx) = mpsc::channel::<ArpCacheEntry>(ARP_CHANNEL_BUFFER_SIZE);

    // Begin listening for ARP replies
    let rx_task = task::spawn(listen(interface.clone(), reply_tx));

    // Begin making ARP requests
    let tx_task = task::spawn(request(interface, ips, reply_rx));

    // Await the transmitter
    // After completing it will pass back the async channel receiver
    let mut reply_rx = tx_task.await?;

    // Await the listener
    rx_task.await?;

    // Construct a list of entries
    let mut entries = Vec::new();

    // Extract each entry
    while let Some (entry) = reply_rx.recv().await {
        entries.push(entry)
    }

    Ok (entries)
}