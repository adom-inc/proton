//! Connected device data.

use std::net::Ipv4Addr;

use nl80211::{
    parse_i8,
    parse_u32,
    Station,
};

use serde::Serialize;

use proton_arp::ArpManager;

#[derive(Serialize, Clone, Copy, Debug)]
/// Information about a connected network device.
pub struct Device {
    /// MAC address of the device.
    pub mac: [u8; 6],

    /// IPv4 address of the device.
    pub ipv4: Ipv4Addr,

    /// Signal strength of last received signal, in dBm.
    pub signal_strength: i8,

    /// Time since the device was connected, in seconds.
    pub connection_time: u32,
}

impl Device {
    /// Convert a `Station` into a `Device` by checking the ARP cache.
    pub fn from_station(station: Station, arp: &ArpManager) -> Self {
        // Get hardware address of the station
        let mac: [u8; 6] = station.bssid
            .unwrap_or_default()
            .try_into()
            .unwrap_or([0; 6]);

        // Get IPv4 address of the station
        let ipv4: Ipv4Addr = arp.lookup_mac(mac.into())
            .unwrap_or(Ipv4Addr::new(0, 0, 0, 0));

        // Get signal strength of this station
        let signal_strength: i8 = parse_i8(&station.signal.unwrap_or_default());

        // Get connection time of this station
        let connection_time: u32 = parse_u32(&station.connected_time.unwrap_or_default());

        Self {
            mac,
            ipv4,
            signal_strength,
            connection_time,
        }
    }
}