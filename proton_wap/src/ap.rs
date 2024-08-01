//! Wireless access point abstraction.

use cidr::Ipv4Cidr;

use proton_dev::{
    Device,
    DeviceManager,
};

use proton_err::ProtonResult;

// use proton_mac::MacAddr;

/// A wireless access point.
pub struct AccessPoint {
    #[allow(dead_code)]
    /// CIDR network range.
    range: Ipv4Cidr,

    /// Device manager.
    manager: DeviceManager,
}

impl AccessPoint {
    /// Constructs a new wireless access point.
    /// 
    /// # Parameters
    /// - `range` (`Ipv4Cidr`): the internal network range associated to
    /// this access point
    /// - `wlifname` (`&str`): the name of the wireless interface over which
    /// this access point connects to remote devices
    /// 
    /// # Returns
    /// A `ProtonResult<AccessPoint>` containing a new `AccessPoint` if
    /// initialization was successful.
    pub fn new(
        range: Ipv4Cidr,
        wlifname: &str,
    ) -> ProtonResult<Self> {
        Ok (Self {
            range,
            manager: DeviceManager::new(range, wlifname)?,
        })
    }

    /// Get a list of all connected devices.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<Vec<Device>>` wrappping the list of devices, if
    /// the network scan was successful.
    pub async fn scan(&mut self) -> ProtonResult<Vec<Device>> {
        Ok (self.manager.scan().await?)
    }

    // /// Deauthenticate a device by its MAC address.
    // /// 
    // /// # Parameters
    // /// - `mac` (`MacAddr`): the MAC address of the device to be
    // /// deauthenticated
    // ///
    // /// # Returns
    // /// A `ProtonResult<()>` indicating the status of the response.
    // pub fn deauth(&mut self, mac: MacAddr) -> ProtonResult<()> {
    //     self.manager.deauthenticate(mac)
    // } 

    // /// Continuously route packets, monitoring both the Data Link Layer and
    // /// the Transport Layer to ensure both proper NAT and MAC policy enforcement.
    // /// 
    // /// # Parameters
    // /// None.
    // /// 
    // /// # Returns
    // /// A `ProtonResult<()>` indicating an error, if one occurred.
    // /// 
    // /// This function does not return during nominal operation.
    // pub async fn run(&mut self) -> ProtonResult<()> {
    //     todo!()
    // }
}