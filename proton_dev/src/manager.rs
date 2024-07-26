//! Device discovery manager.

use cidr::Ipv4Cidr;

use nl80211::{
    Interface,
    parse_string,
    Socket,
};

use proton_arp::ArpManager;

use proton_err::{
    ProtonError,
    ProtonResult,
};

use crate::{
    Device,
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

    /// Wireless interface name.
    wlifname: String,

    /// ARP cache manager.
    arp_manager: ArpManager,
}

impl DeviceManager {
    /// Construct a new device manager.
    /// 
    /// # Parameters
    /// - `range` (`Ipv4Cidr`): the CIDR range of the network
    /// - `wlifname` (`&str`): the name of the wireless interface
    /// over which to scan for connected devices
    /// 
    /// # Returns
    /// The result type `ProtonResult<DeviceManager>` containing the device
    /// manager, if its initialization was successful.
    pub fn new(range: Ipv4Cidr, wlifname: &str) -> ProtonResult<Self> {
        Ok (Self {
            socket: Socket::connect()?,
            wlifname: wlifname.to_string(),
            arp_manager: ArpManager::new(range, wlifname),
        })
    }

    /// Get a list of connected devices.
    /// 
    /// # Parameters
    /// - `ifname` (`&str`): the name of the network interface to scan
    /// 
    /// # Returns
    /// The result type `ProtonResult<Vec<Device>>` containing a list of
    /// connected devices.
    pub async fn scan(&mut self) -> ProtonResult<Vec<Device>> {
        // Perform an ARP scan of the network to get IPs
        self.arp_manager.scan().await?;

        // Determine Wi-Fi device by name
        let check_wifi_device = |iface: &Interface| parse_string(&iface.name.clone().unwrap_or_default()) == self.wlifname;

        // Get the Wi-Fi device
        let interface = self.socket.get_interfaces_info()?
            .into_iter()
            .find(check_wifi_device)
            .ok_or(ProtonError::CouldNotFindWirelessInterface)?;

        // Get all stations
        let stations = self.socket.get_all_stations(&interface.index.unwrap())?;

        // Convert each station into a native device structure
        let devices = stations.into_iter()
            .map(|station| Device::from_station(station, &self.arp_manager))
            .collect::<Vec<Device>>();

        Ok (devices)
    }
}