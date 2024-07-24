//! Wireless access point abstraction.

use std::net::{
    Ipv4Addr,
    SocketAddrV4,
};

use pnet::packet::{
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
};

use proton_nat::NatTable;

/// A wireless access point.
pub struct AccessPoint {
    /// Network Address Translation (NAT) table.
    nat: NatTable,
}

impl AccessPoint {
    /// Constructs a new wireless access point.
    /// 
    /// # Parameters
    /// - `external_ipv4` (`Ipv4Addr`): the external IPv4 address assigned
    /// to this access point.
    /// 
    /// # Returns
    /// A new `AccessPoint`.
    pub fn new(external_ipv4: Ipv4Addr) -> Self {
        Self {
            nat: NatTable::new(vec![external_ipv4]),
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
    pub fn translate_outgoing_ipv4_packet(&mut self, packet: Ipv4Packet) -> Option<Ipv4Packet> {
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
    pub fn translate_incoming_ipv4_packet(&mut self, packet: Ipv4Packet) -> Option<Ipv4Packet> {
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