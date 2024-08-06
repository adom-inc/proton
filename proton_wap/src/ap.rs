//! Wireless access point abstraction.

use cidr::Ipv4Cidr;

use proton_dev::{
    Device,
    DeviceManager,
};

use proton_err::ProtonResult;

use crate::HotspotConfig;

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

    /// Start a new hotspot on this access point.
    pub async fn start(&mut self, config: HotspotConfig) -> ProtonResult<()> {
        
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
}