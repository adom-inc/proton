//! ARP Reply functionality.

use pnet::{
    packet::{
        arp::ArpPacket,
        ethernet::{
            EtherTypes,
            EthernetPacket,
        },
        Packet,
    },
};

use tokio::sync::mpsc;

use proton_nif::NetworkInterface;

use crate::ArpCacheEntry;

/// Receive a series of ARP replies.
///
/// # Parameters
/// - `interface` (`NetworkInterface`): the network interface to use
/// - `tx` (`Sender<ArpCacheEntry>`): the cache entry transmitter
/// 
/// # Returns
/// None.
pub async fn listen(
    mut interface: NetworkInterface,
    tx: mpsc::Sender<ArpCacheEntry>,
) {
    // Get interface MAC address
    let mac = interface.mac.unwrap();

    while let Some (packet) = interface.recv().await {
        // Check if the MPSC channel has closed
        // There's no point in continuing if it is because
        //  all future packets will be dropped anyways
        if tx.is_closed() {
            break;
        }

        // Convert to ETH Frame
        let eth_frame = if let Some (f) = EthernetPacket::new(&packet) {
            f
        } else {
            continue;
        };

        // Check ETH Frame Type
        let frame_type = eth_frame.get_ethertype();
        if frame_type != EtherTypes::Arp {
            continue;
        }

        // Convert to ARP Packet
        let arp_packet = if let Some (a) = ArpPacket::new(eth_frame.payload()) {
            a
        } else {
            continue;
        };

        // Drop the frame if it was sent from our own computer
        if arp_packet.get_sender_hw_addr() == mac {
            continue;
        }

        // Construct cache entry
        let entry = ArpCacheEntry::new(
            arp_packet.get_sender_proto_addr(),
            arp_packet.get_sender_hw_addr().into(),
        );

        // Send the reply
        let send = tx.send(entry);

        // When the receiver side of the channel is closed,
        //  this will return, because `tx::send` will return an error
        if send.await.is_err() {
            break;
        }

        // If there are no packets left, the function returns
    }
}