//! Wireless access point abstraction.

use std::{
    io,
    net::{
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
        TransportChannelType::Layer4,
        TransportProtocol::Ipv4,
        transport_channel,
        ipv4_packet_iter,
    },
};

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
    pub fn new(external_ipv4: Ipv4Addr, range: Ipv4Cidr) -> Self {
        Self {
            nat: NatTable::new(vec![external_ipv4]),
            range,
        }
    }

    /// Continuously route packets on the Transport Layer (OSI Layer 4).
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// An `AccessPointResult` indicating an error, if one occurred.
    /// 
    /// This function does not return if there are no errors.
    pub fn run(&mut self) -> AccessPointResult {
        // Create an IPv4 protocol
        let protocol = Layer4 (Ipv4 (IpNextHeaderProtocols::Ipv4));

        // Create a new transport protocol 
        let (mut tx, mut rx) = match transport_channel(TRANSPORT_CHANNEL_BUFFER_SIZE, protocol) {
            Ok ((tx, rx)) => (tx, rx),
            Err (_) => return Err (io::Error::new(io::ErrorKind::Other, "could not open transport channel")),
        };

        // We treat received packets as if they were IPv4 packets
        let mut iter = ipv4_packet_iter(&mut rx);

        // Continuously iterate through the packets on the receiving line
        loop {
            match iter.next() {
                Ok ((packet, addr)) => {
                    // Allocate enough space for a new packet
                    let mut vec: Vec<u8> = vec![0; packet.packet().len()];
                    let mut new_packet = MutableIpv4Packet::new(&mut vec[..]).unwrap();
    
                    // Create a clone of the original packet
                    new_packet.clone_from(&packet);

                    // Get source IPv4
                    let source_ipv4: Ipv4Addr = new_packet.get_source();

                    // Construct immutable packet
                    let ipv4_packet = new_packet.to_immutable();

                    // Detect NAT type and translate packet
                    let translated_packet = if self.range.contains(&source_ipv4) {
                        // Source NAT
                        self.translate_outgoing_ipv4_packet(ipv4_packet)
                    } else {
                        // Destination NAT
                        self.translate_incoming_ipv4_packet(ipv4_packet)
                    }.ok_or(io::Error::new(io::ErrorKind::Other, "could not perform NAT"))?;
    
                    // Send the translated packet
                    if tx.send_to(translated_packet, addr).is_err() {
                        println!("Failed to send packet to address {}", addr);
                    }
                }
                Err (e) => {
                    // If an error occurs, we can handle it here
                    println!("Failed to route packet: {:#?}", e);
                    continue;
                }
            }
        }
    }

    /// Translate an outgoing IPv4 packet.
    /// 
    /// # Parameters
    /// - `packet` (`Ipv4Packet`): the IPv4 packet to translate
    /// 
    /// # Returns
    /// An `Option<IPv4Packet>` with a translated source address and port number, if
    /// translation was successful.
    fn translate_outgoing_ipv4_packet(&mut self, packet: Ipv4Packet) -> Option<Ipv4Packet> {
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
        let translated_source_socket = if let Some (i) = self.nat.translate_source(source_socket) {
            i
        } else {
            // Try to add the address to the NAT table
            self.nat.add(source_socket)?
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
    /// - `packet` (`Ipv4Packet`): the IPv4 packet to translate
    /// 
    /// # Returns
    /// An `Option<IPv4Packet>` with a translated destination address and port number, if
    /// translation was successful.
    fn translate_incoming_ipv4_packet(&mut self, packet: Ipv4Packet) -> Option<Ipv4Packet> {
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
        let translated_destination_socket = self.nat.translate_destination(destination_socket)?;

        // Extract the IPv4 address and port number from the socket
        let new_ipv4 = translated_destination_socket.ip();
        let new_port = translated_destination_socket.port();

        // Translate the IPv4 address
        ip_packet.set_destination(*new_ipv4);

        // Construct a mutable TCP segment from the IPv4 payload
        let mut tcp_segment = MutableTcpPacket::new(ip_packet.payload_mut())?;

        // Translate the port number
        tcp_segment.set_destination(new_port);

        Some (ip_packet.consume_to_immutable())
    }
}