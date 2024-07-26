//! Wireless access point abstraction.

use cidr::Ipv4Cidr;

use proton_dev::DeviceManager;

use proton_err::ProtonResult;

#[allow(dead_code)]
/// A wireless access point.
pub struct AccessPoint {
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
    /// 
    /// # Returns
    /// A `ProtonResult<AccessPoint>` containing a new `AccessPoint` if
    /// initialization was successful.
    pub fn new(
        range: Ipv4Cidr,
    ) -> ProtonResult<Self> {
        Ok (Self {
            range,
            manager: DeviceManager::new(range)?,
        })
    }

    /// Continuously route packets, monitoring both the Data Link Layer and
    /// the Transport Layer to ensure both proper NAT and MAC policy enforcement.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `ProtonResult<()>` indicating an error, if one occurred.
    /// 
    /// This function does not return during nominal operation.
    pub async fn run(&mut self) -> ProtonResult<()> {
        loop { }
    }
}