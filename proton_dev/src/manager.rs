//! Device discovery manager.

use std::time::Duration;

use cidr::Ipv4Cidr;

use neli::err::NlError;

use nl80211::{
    Interface,
    parse_string,
    Socket,
};

use proton_arp::ArpManager;

use crate::{
    Device,
    NetlinkResult,
    NetworkSocket,
};

/// A device manager abstraction.
/// 
/// This structure is responsible for performing network device discovery by
/// interfacing with Linux's `nl80211` public header as well as directly
/// communicating with connected devices via ARP requests.
pub struct DeviceManager {
    /// Network interface socket.
    socket: Socket,

    /// ARP cache manager.
    arp_manager: ArpManager,
}

impl DeviceManager {
    /// Construct a new device manager.
    /// 
    /// # Parameters
    /// - `range` (`Ipv4Cidr`): the CIDR range of the network
    /// - `refresh` (`Duration`): the amount of time after which to refresh
    /// the ARP cache
    /// 
    /// # Returns
    /// The result type `NetlinkResult<DeviceManager>` containing the device
    /// manager, if its initialization was successful.
    pub fn new(range: Ipv4Cidr, refresh: Duration) -> NetlinkResult<Self> {
        Ok (Self {
            socket: Socket::connect()?,
            arp_manager: ArpManager::new(range, refresh),
        })
    }

    /// Get a list of connected devices.
    /// 
    /// # Parameters
    /// - `ifname` (`&str`): the name of the network interface to scan
    /// 
    /// # Returns
    /// The result type `NetlinkResult<Vec<Device>>` containing a list of
    /// connected devices.
    pub fn scan(&mut self, ifname: &str) -> NetlinkResult<Vec<Device>> {
        // Determine Wi-Fi device by name
        let check_wifi_device = |iface: &Interface| parse_string(&iface.name.clone().unwrap_or_default()) == ifname;

        // Get the Wi-Fi device
        let interface = self.socket.get_interfaces_info()?
            .into_iter()
            .find(check_wifi_device)
            .ok_or(NlError::Msg ("no wireless interface available".to_string()))?;

        // Get all stations
        let stations = self.socket.get_all_stations(&interface.index.unwrap())?;

        // Convert each station into a native device structure
        let devices = stations.into_iter()
            .map(|station| Device::from_station(station, &self.arp_manager))
            .collect::<Vec<Device>>();

        Ok (devices)
    }
}