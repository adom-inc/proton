//! Network interface abstraction.

use std::{
    net::{
        IpAddr,
        Ipv4Addr,
    },
    sync::Arc,
};

use pnet::{
    datalink::{
        self,
        Channel,
        channel,
        DataLinkSender,
        DataLinkReceiver,
        interfaces,
        MacAddr,
    },
    packet::{
        ethernet::EthernetPacket,
        Packet,
    },
};

use tokio::sync::Mutex;

#[derive(Clone)]
/// An OSI Layer 2 (Data Link Layer) network interface.
pub struct NetworkInterface {
    /// Transmitter line (talks to network interface).
    tx: Arc<Mutex<Box<dyn DataLinkSender>>>,

    /// Receiver line (listens to network interface).
    rx: Arc<Mutex<Box<dyn DataLinkReceiver>>>,

    /// MAC address of the interface.
    pub mac: Option<MacAddr>,

    /// IPv4 address of the interface.
    pub ipv4: Option<Ipv4Addr>,
}

impl<'a> NetworkInterface {
    /// Constructs a new network interface.
    /// 
    /// # Parameters
    /// - `iface_name` (`&str`): the name of the network interface
    ///
    /// # Returns
    /// `Option<Self>`, containing the network interface, if it was found.
    pub fn new(iface_name: &str) -> Option<Self> {
        // Get network interface by name
        let all_interfaces = interfaces();
        let check_wireless = |iface: &datalink::NetworkInterface| iface.name == iface_name;
        let interface = all_interfaces.into_iter()
            .find(check_wireless)?;

        // Get MAC (hardware) address
        let mac = interface.mac;

        // Get IPv4 (protocol) address
        let ipv4 = interface.ips
            .iter()
            .find(|ip| ip.is_ipv4())
            .map(|i| if let IpAddr::V4 (ipv4) = i.ip() {
                ipv4
            } else {
                unreachable!()
            });

        // Open channel on the Data Link Layer (Layer 2)
        let channel = channel(
            &interface,         // Network interface
            Default::default(), // Configuration info
        ).ok()?;

        // Destructure channel into TX and RX lines
        // Note: the `Channel` enumeration is documented as non-exhaustive
        if let Channel::Ethernet (tx, rx) = channel {
            Some (Self {
                tx: Arc::new(Mutex::new(tx)),
                rx: Arc::new(Mutex::new(rx)),
                mac,
                ipv4,
            })
        } else {
            None
        }
    }

    /// Yield the next Ethernet frame from the receiver.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `Option<Vec<u8>>` containing the received frame, if
    /// it was available.
    pub async fn recv(&'a mut self) -> Option<Vec<u8>> {
        let mut rx_lock = self.rx.lock().await;

        Some (rx_lock.next().ok()?.to_vec())
    }

    /// Send an Ethernet frame to this interface.
    /// 
    /// # Parameters
    /// - `packet` (`EthernetPacket`): the Ethernet frame to send
    /// 
    /// # Returns
    /// None.
    pub async fn send(&'a mut self, packet: EthernetPacket<'a>) {
        let mut tx_lock = self.tx.lock().await;

        tx_lock.send_to(packet.packet(), None);
    }
}