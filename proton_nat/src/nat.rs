//! Network Address Translation (NAT) table.

use std::{
    cmp::Ordering,
    net::{
        Ipv4Addr,
        SocketAddrV4,
    },
};

use bimap::BiMap;

/// OSI Layer 4 (Transport Layer) port.
type Layer4Port = u16;

/// Minimum Layer 4 port used.
const MINIMUM_NAT_PORT: Layer4Port = 50_000;

/// Maximum Layer 4 port permitted (inclusive).
const MAXIMUM_NAT_PORT: Layer4Port = u16::MAX;

#[derive(Clone)]
/// A Network Address Translation table.
/// 
/// This table provides a bijective mapping (with the help of the external crate `bimap`) to
/// translate between internal IPv4 sockets and external IPv4 sockets.
pub struct NatTable {
    /// A bijective mapping.  The left element indicates the
    /// internal address and the right element indicates the
    /// external address.
    table: BiMap<SocketAddrV4, SocketAddrV4>,

    /// A list of external IPv4 addresses available for NAT.
    ips: Vec<Ipv4Addr>,

    /// The index of the next available external IPv4 address.
    ip_index: usize,

    /// The next available OSI Layer 4 port.
    next_port: Layer4Port,

    /// The number of available sockets.
    available: usize,
}

#[allow(clippy::len_without_is_empty)]
impl NatTable {
    /// Construct a new NAT table.
    /// 
    /// # Parameters
    /// - `ips` (`Vec<IPv4Addr>`): a list of external IPv4 addresses
    ///   available for NAT
    /// 
    /// # Returns
    /// A new `NatTable`.
    pub fn new(ips: Vec<Ipv4Addr>) -> Self {
        Self {
            table: BiMap::new(),
            ip_index: 0,
            next_port: MINIMUM_NAT_PORT,
            available: ips.len() * (MAXIMUM_NAT_PORT - MINIMUM_NAT_PORT + 1) as usize,
            ips,
        }
    }

    /// Get the length of the NAT table.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `usize` of the length of the table.
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Get the number of available sockets.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `usize` with the number of available sockets.
    pub fn available(&self) -> usize {
        self.available
    }

    /// Get the next available socket.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// `Option<SocketAddrV4>` with the next unused external
    /// socket, if available.
    fn get_next_socket(&mut self) -> Option<SocketAddrV4> {
        // Get IPv4 address
        let ipv4 = self.ips[self.ip_index];

        // Get port
        let port = self.next_port;

        // Increment next port
        match self.next_port.cmp(&MAXIMUM_NAT_PORT) {
            Ordering::Less => self.next_port += 1,
            Ordering::Equal => if self.ip_index + 1 < self.ips.len() {
                // We have another IP address
                self.ip_index += 1;
                self.next_port = MINIMUM_NAT_PORT;
            } else {
                // We are out of IP addresses
                return None;
            },
            Ordering::Greater => unreachable!(),
        }

        Some (SocketAddrV4::new(ipv4, port))
    }

    /// Free the last used socket.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// `Option<SocketAddrV4>` with the freed socket,
    /// if available.
    fn free_last_socket(&mut self) -> Option<SocketAddrV4> {
        // Make sure at least one socket has been used,
        // otherwise this function will panic
        if self.ip_index == 0 && self.next_port == MINIMUM_NAT_PORT {
            return None;
        }

        // Get the last used port number
        let port = match self.next_port.cmp(&MINIMUM_NAT_PORT) {
            Ordering::Less => unreachable!(),
            Ordering::Equal => MAXIMUM_NAT_PORT,
            Ordering::Greater => self.next_port - 1,
        };

        // Get the last used IP address
        let ipv4 = if port == MAXIMUM_NAT_PORT {
            // Wrap back around
            // Note: this will never panic because we checked that
            // at least one port was used
            self.ips[self.ip_index - 1]
        } else {
            // Still on the same IPv4
            self.ips[self.ip_index]
        };

        Some (SocketAddrV4::new(ipv4, port))
    }

    /// Add a NAT entry.
    ///
    /// # Parameters
    /// - `internal` (`SocketAddrV4`): the internal IPv4 socket address
    ///
    /// # Returns
    /// `Option<SocketAddrV4>` containing the external IPv4 socket
    /// address, if another port was available and addition was successful.
    pub fn add(&mut self, internal: SocketAddrV4) -> Option<SocketAddrV4> {
        // Get the next available socket
        let external = self.get_next_socket();

        if let Some (socket) = external {
            // Insert the addresses into the NAT table
            self.table.insert(internal, socket);
        } else {
            return None;
        }

        // Reduce the number of available sockets
        self.available -= 1;

        external
    }

    /// Delete a NAT entry, freeing the external address for later use.
    /// 
    /// # Parameters
    /// - `internal` (`SocketAddrV4`): the internal IPv4 socket address
    ///
    /// # Returns
    /// `Option<SocketAddrV4>` of the external address that was freed,
    /// if it existed.
    pub fn delete(&mut self, internal: SocketAddrV4) -> Option<SocketAddrV4> {
        // Remove the address from the table, if it exists
        let external = self.table.remove_by_left(&internal);

        let socket = if external.is_some() {
            // Free the last used socket, returning the freed address
            self.free_last_socket()
        } else {
            // The internal address doesn't exist
            return None;
        };

        // Increase the number of available sockets
        self.available += 1;

        socket
    }

    /// Look up an external address by internal address.
    /// This is used for Source Network Address Translation (SNAT).
    /// 
    /// # Parameters
    /// - `internal` (`SocketAddrV4`): the internal IPv4 socket address
    /// 
    /// # Returns
    /// None.
    pub fn translate_source(&self, internal: SocketAddrV4) -> Option<SocketAddrV4> {
        self.table.get_by_left(&internal).copied()
    }

    /// Look up an internal address by external address.
    /// This is used for Destination Network Address Translation (DNAT).
    /// 
    /// # Parameters
    /// - `external` (`SocketAddrV4`): the external IPv4 socket address
    /// 
    /// # Returns
    /// None.
    pub fn translate_destination(&self, external: SocketAddrV4) -> Option<SocketAddrV4> {
        self.table.get_by_right(&external).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_nat_table() {
        let ips = vec![Ipv4Addr::new(10, 0, 0, 1)];
        let nat = NatTable::new(ips);

        // Table length
        assert_eq!(nat.len(), 0);

        // Port availability
        let available = (MAXIMUM_NAT_PORT - MINIMUM_NAT_PORT + 1) as usize;
        assert_eq!(nat.available(), available);
    }

    #[test]
    fn add_nat_entry() {
        let ips = vec![Ipv4Addr::new(10, 0, 0, 1)];
        let mut nat = NatTable::new(ips);

        // Table length
        assert_eq!(nat.len(), 0);

        // Create an HTTPS entry
        let device_ipv4 = Ipv4Addr::new(192, 168, 0, 1);
        let device_port = 443;
        let device_socket = SocketAddrV4::new(device_ipv4, device_port);
        let external = nat.add(device_socket);

        // Test entry creation
        let ref_external_ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ref_external_socket = SocketAddrV4::new(ref_external_ipv4, MINIMUM_NAT_PORT);
        assert_eq!(external, Some (ref_external_socket));

        // Test Source NAT
        let source = device_socket;
        let translated_source = nat.translate_source(source);
        assert_eq!(translated_source, Some(ref_external_socket));

        // Test Destination NAT
        let destination = ref_external_socket;
        let translated_destination = nat.translate_destination(destination);
        assert_eq!(translated_destination, Some (device_socket));
    }

    #[test]
    fn delete_nat_entry() {
        let ips = vec![Ipv4Addr::new(10, 0, 0, 1)];
        let mut nat = NatTable::new(ips);

        // Table length
        assert_eq!(nat.len(), 0);

        // Create an HTTPS entry
        let device_ipv4 = Ipv4Addr::new(192, 168, 0, 1);
        let device_port = 443;
        let device_socket = SocketAddrV4::new(device_ipv4, device_port);
        let external = nat.add(device_socket);

        // Test entry creation
        let ref_external_ipv4 = Ipv4Addr::new(10, 0, 0, 1);
        let ref_external_socket = SocketAddrV4::new(ref_external_ipv4, MINIMUM_NAT_PORT);
        assert_eq!(external, Some (ref_external_socket));

        // Test entry deletion
        let freed_socket = nat.delete(device_socket);
        assert_eq!(freed_socket, Some (ref_external_socket));
    }
}