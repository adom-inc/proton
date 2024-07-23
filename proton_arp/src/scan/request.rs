//! ARP Request functionality.

use std::net::Ipv4Addr;

use pnet::{
    datalink::MacAddr,
    packet::{
        arp::{
            ArpHardwareTypes,
            ArpOperations,
            MutableArpPacket,
        },
        ethernet::{
            EtherTypes,
            MutableEthernetPacket,
        },
        MutablePacket,
    },
};

use tokio::{
    sync::mpsc,
    time,
};

use proton_nif::NetworkInterface;

use crate::ArpCacheEntry;

use super::ARP_LISTENER_DELAY;

/// Make a series of ARP requests to the provided IPv4 addresses.
/// 
/// # Parameters
/// - `interface` (`NetworkInterface`): the network interface to use
/// - `ips` (`Ipv4Addr`): the IPv4 addresses to scan
/// - `rx` (`Receiver<ArpCacheEntry>`): the cache entry receiver, to be closed after transmission
/// 
/// # Returns
/// `Receiver<ArpCacheEntry>` to be read after channel closure.
pub async fn request(
    mut interface: NetworkInterface,
    ips: Vec<Ipv4Addr>,
    mut rx: mpsc::Receiver<ArpCacheEntry>,
) -> mpsc::Receiver<ArpCacheEntry> {
    // Get MAC address of interface
    let interface_mac = interface.mac.unwrap();

    // Get IPv4 address of interface
    let interface_ipv4 = interface.ipv4.unwrap();

    // Iterate over IPv4 addresses
    for ipv4 in ips {
        // Construct ETH frame
        let mut eth_frame = MutableEthernetPacket::owned(vec![0u8; 48]).unwrap();
        eth_frame.set_ethertype(EtherTypes::Arp);
        eth_frame.set_source(interface_mac);
        eth_frame.set_destination(MacAddr::broadcast());

        // Construct ARP Packet
        let mut arp_packet = MutableArpPacket::new(eth_frame.payload_mut()).unwrap();
        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(ArpOperations::Request);
        arp_packet.set_sender_hw_addr(interface_mac);
        arp_packet.set_sender_proto_addr(interface_ipv4);
        arp_packet.set_target_hw_addr(MacAddr::zero());
        arp_packet.set_target_proto_addr(ipv4);

        interface.send(eth_frame.to_immutable()).await;
    }

    // Wait
    time::sleep(ARP_LISTENER_DELAY).await;

    // Close the channel
    // This stops the receiver
    rx.close();

    rx
}