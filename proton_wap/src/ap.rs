//! Wireless access point abstraction.

use std::{
    io,
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddrV4,
    },
};

use cidr::Ipv4Cidr;

use pnet::{
    packet::{
        ip::IpNextHeaderProtocols,
        ipv4::{
            Ipv4Packet,
            MutableIpv4Packet,
        },
        tcp::{
            TcpPacket,
            MutableTcpPacket,
        },
        Packet,
        MutablePacket,
    },
    transport::{
        TransportChannelType::Layer3,
        transport_channel,
        ipv4_packet_iter,
    },
};

use tokio::task;

use proton_nat::NatTable;

use crate::AccessPointResult;

/// Transport channel buffer size.
pub const TRANSPORT_CHANNEL_BUFFER_SIZE: usize = 4_096;

/// A wireless access point.
pub struct AccessPoint {
    /// Network Address Translation (NAT) table.
    nat: NatTable,

    /// CIDR network range.
    range: Ipv4Cidr,
}

impl AccessPoint {
    /// Constructs a new wireless access point.
    /// 
    /// # Parameters
    /// - `external_ipv4` (`Ipv4Addr`): the external IPv4 address assigned
    /// to this access point
    /// - `range` (`Ipv4Cidr`): the internal network range associated to
    /// this access point
    /// 
    /// # Returns
    /// A new `AccessPoint`.
    pub fn new(
        external_ipv4: Ipv4Addr,
        range: Ipv4Cidr,
    ) -> Self {
        Self {
            nat: NatTable::new(vec![external_ipv4]),
            range,
        }
    }

    /// Continuously route packets, monitoring both the Data Link Layer and
    /// the Transport Layer to ensure both proper NAT and MAC policy enforcement.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `AccessPointResult<()>` indicating an error, if one occurred.
    /// 
    /// This function does not return during nominal operation.
    pub async fn run(&mut self) -> AccessPointResult<()> {
        // Construct new NAT
        let nat = self.nat.clone();

        // Get network range
        let range = self.range;

        let layer_4_task = task::spawn(Self::run_layer_4(nat, range));

        match tokio::join!(layer_4_task) {
            (Ok (Ok (_)),) => Ok (()),
            _ => todo!(),
        }
    }

    /// Continuously route packets on the Transport Layer (OSI Layer 4).
    /// 
    /// # Parameters
    /// - `nat` (`NatTable`): the reference NAT table
    /// - `range` (`Ipv4Cidr`): the network range
    /// 
    /// # Returns
    /// An `AccessPointResult<()>` indicating an error, if one occurred.
    /// 
    /// This function does not return during nominal operation.
    async fn run_layer_4(
        mut nat: NatTable,
        range: Ipv4Cidr,
    ) -> AccessPointResult<()> {
        // Create a network layer protocol with TCP packets
        let protocol = Layer3 (IpNextHeaderProtocols::Tcp);

        // Create a new transport protocol 
        let (mut tx, mut rx) = match transport_channel(TRANSPORT_CHANNEL_BUFFER_SIZE, protocol) {
            Ok ((tx, rx)) => (tx, rx),
            Err (_) => return Err (io::Error::new(io::ErrorKind::Other, "could not open transport channel")),
        };

        // We treat received packets as if they are IPv4 packets
        let mut iter = ipv4_packet_iter(&mut rx);

        println!("Beginning listener...");

        // Continuously iterate through the packets on the receiving line
        loop {
            println!("    Waiting for packet...");
            match iter.next() {
                Ok ((packet, addr)) => {
                    println!("        Received packet: {:#?}", packet);

                    let (translated_packet, translated_addr) = Self::translate_packet(
                        &mut nat,
                        range,
                        packet,
                        addr,
                    )?;

                    println!("        Translated packet: {:#?}", translated_packet);
    
                    // Send the translated packet
                    if tx.send_to(translated_packet, translated_addr).is_err() {
                        println!("        Failed to send packet to address {}", addr);
                    }
                }
                Err (e) => {
                    // If an error occurs, we can handle it here
                    println!("        Failed to route packet: {:#?}", e);
                    continue;
                }
            }
            println!();
        }
    }

    /// Translate an IPv4 packet.
    /// 
    /// # Parameters
    /// - `nat` (`&mut NatTable`): the reference NAT table
    /// - `range` (`Ipv4Cidr`): the network range
    /// - `packet` (`Ipv4Packet`): an IPv4 packet to be translated
    /// - `addr` (`IpAddr`): the destination IP address of the packet,
    /// which will change in the case of Destination NAT (DNAT).
    fn translate_packet<'a>(
        nat: &'a mut NatTable,
        range: Ipv4Cidr,
        packet: Ipv4Packet<'a>,
        addr: IpAddr,
    ) -> AccessPointResult<(Ipv4Packet<'a>, IpAddr)> {
        // Allocate enough space for a new packet
        let mut vec: Vec<u8> = vec![0; packet.packet().len()];
        let mut new_packet = MutableIpv4Packet::new(&mut vec[..]).unwrap();

        // Create a clone of the original packet
        new_packet.clone_from(&packet);

        // Get source IPv4
        let source_ipv4: Ipv4Addr = new_packet.get_source();

        // Construct immutable packet
        let ipv4_packet = new_packet.to_immutable();

        // Set translated address
        let mut translated_addr = addr;

        // Detect NAT type and translate packet
        let output_packet = if range.contains(&source_ipv4) {
            // Source NAT
            Self::translate_outgoing_ipv4_packet(nat, ipv4_packet)
                .ok_or(io::Error::new(io::ErrorKind::Other, "could not perform source NAT"))?
        } else {
            // Destination NAT
            let (packet, new_addr) = Self::translate_incoming_ipv4_packet(nat, ipv4_packet)
                .ok_or(io::Error::new(io::ErrorKind::Other, "could not perform destination NAT"))?;
            
            // Set new destination address
            translated_addr = IpAddr::V4 (*new_addr.ip());

            packet
        };

        // Clone the translated packet
        let vec: Vec<u8> = vec![0; packet.packet().len()];
        let mut translated_packet = MutableIpv4Packet::owned(vec).unwrap();
        translated_packet.clone_from(&output_packet);

        Ok ((translated_packet.consume_to_immutable(), translated_addr))
    }

    /// Translate an outgoing IPv4 packet.
    /// 
    /// # Parameters
    /// - `nat` (`&mut NatTable`): the reference NAT table
    /// - `packet` (`Ipv4Packet`): the IPv4 packet to translate
    /// 
    /// # Returns
    /// An `Option<IPv4Packet>` with a translated source address and port number, if
    /// translation was successful.
    fn translate_outgoing_ipv4_packet<'a>(
        nat: &'a mut NatTable,
        packet: Ipv4Packet<'a>
    ) -> Option<Ipv4Packet<'a>> {
        // Construct a mutable IPv4 packet
        let mut ip_packet = MutableIpv4Packet::owned(packet.packet().to_vec())?;
        
        // Get the source IPv4 address
        let source_ipv4: Ipv4Addr = ip_packet.get_source();

        // Construct an immutable TCP segment
        let tcp_segment = TcpPacket::owned(ip_packet.payload().to_vec())?;

        // Get the source TCP port
        let source_port: u16 = tcp_segment.get_source();

        // Construct the source socket
        let source_socket = SocketAddrV4::new(source_ipv4, source_port);
        
        // Check if the IPv4 address is in the NAT table
        let translated_source_socket = if let Some (i) = nat.translate_source(source_socket) {
            i
        } else {
            // Try to add the address to the NAT table
            nat.add(source_socket)?
        };

        // Extract the IPv4 address and port number from the socket
        let new_ipv4 = translated_source_socket.ip();
        let new_port = translated_source_socket.port();

        // Translate the IPv4 address
        ip_packet.set_source(*new_ipv4);

        // Construct a mutable TCP segment from the IPv4 payload
        let mut tcp_segment = MutableTcpPacket::new(ip_packet.payload_mut())?;

        // Translate the port number
        tcp_segment.set_source(new_port);

        Some (ip_packet.consume_to_immutable())
    }

    /// Translate an incoming IPv4 packet.
    /// 
    /// # Parameters
    /// - `nat` (`&mut NatTable`): the reference NAT table
    /// - `packet` (`Ipv4Packet`): the IPv4 packet to translate
    /// 
    /// # Returns
    /// An `Option<(IPv4Packet, SocketAddrV4)>` with an IPv4 packet with translated destination
    /// address and port number, and the new destination, if translation was successful.
    fn translate_incoming_ipv4_packet<'a>(
        nat: &'a mut NatTable,
        packet: Ipv4Packet<'a>
    ) -> Option<(Ipv4Packet<'a>, SocketAddrV4)> {
        // Construct a mutable IPv4 packet
        let mut ip_packet = MutableIpv4Packet::owned(packet.packet().to_vec())?;
        
        // Get the destination IPv4 address
        let destination_ipv4: Ipv4Addr = ip_packet.get_destination();

        // Construct an immutable TCP segment
        let tcp_segment = TcpPacket::owned(ip_packet.payload().to_vec())?;

        // Get the destination TCP port
        let destination_port: u16 = tcp_segment.get_destination();

        // Construct the destination socket
        let destination_socket = SocketAddrV4::new(destination_ipv4, destination_port);
        
        // Check if the IPv4 address is in the NAT table
        let translated_destination_socket = nat.translate_destination(destination_socket)?;

        // Extract the IPv4 address and port number from the socket
        let new_ipv4 = translated_destination_socket.ip();
        let new_port = translated_destination_socket.port();

        // Translate the IPv4 address
        ip_packet.set_destination(*new_ipv4);

        // Construct a mutable TCP segment from the IPv4 payload
        let mut tcp_segment = MutableTcpPacket::new(ip_packet.payload_mut())?;

        // Translate the port number
        tcp_segment.set_destination(new_port);

        Some ((ip_packet.consume_to_immutable(), translated_destination_socket))
    }
}