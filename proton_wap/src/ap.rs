//! Wireless access point abstraction.

use cidr::Ipv4Cidr;

use crate::AccessPointResult;

#[allow(dead_code)]
/// A wireless access point.
pub struct AccessPoint {
    /// CIDR network range.
    range: Ipv4Cidr,
}

impl AccessPoint {
    /// Constructs a new wireless access point.
    /// 
    /// # Parameters
    /// - `range` (`Ipv4Cidr`): the internal network range associated to
    /// this access point
    /// 
    /// # Returns
    /// A new `AccessPoint`.
    pub fn new(
        range: Ipv4Cidr,
    ) -> Self {
        Self {
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
        loop { }
    }
}