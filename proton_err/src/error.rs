//! Enumeration of Proton AP management errors.

use std::{
    error::Error,
    fmt::{
        Display,
        Debug,
        Formatter,
        Result,
    },
};

use proton_mac::MacAddr;

#[derive(Debug)]
/// An error that occurred within the Proton library.
pub enum ProtonError {
    /// The provided interface was not an Ethernet interface, as expected.
    MustBeEthernetInterface,

    /// The hotspot has not yet been initialized.
    HotspotNotInitialized,

    /// The program could not find any wireless network interfaces.
    CouldNotFindWirelessInterface,

    /// Could not get device information.
    CouldNotGetDeviceInformation,

    /// Netlink gave no response.
    NoResponseFromNetlink,

    /// Could not activate hotspot after creation.
    CouldNotActivateHotspot,

    /// Could not deauthenticate device by MAC address.
    CouldNotDeauthenticateDevice (MacAddr),

    /// Could not parse into CIDR range.
    CouldNotParseAsCidr (String),

    /// Root permissions required.
    MustHaveRootPermissions,

    /// CIDR range must contain network gateway.
    CidrMustContainGateway {
        /// Provided CIDR network range.
        cidr: String,

        /// Provided gateway IPv4 address.
        gateway: String,
    },

    /// An error that could not be converted to a native error.
    Other (String),
}

impl Display for ProtonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ProtonError::*;
        let error = match self {
            MustBeEthernetInterface => "must be Ethernet interface",
            MustHaveRootPermissions => "must execute with root permissions",
            HotspotNotInitialized => "hotspot not initialized",
            CouldNotFindWirelessInterface => "could not find wireless interface",
            CouldNotGetDeviceInformation => "could not get wireless device information",
            NoResponseFromNetlink => "no response from Netlink",
            CouldNotParseAsCidr (cidr) => &format!("could not parse '{}' into a valid CIDR range", cidr),
            CouldNotDeauthenticateDevice (mac) => &format!("could not deauthenticate device with MAC address {}", mac),
            CouldNotActivateHotspot => "could not activate hotspot",
            CidrMustContainGateway {
                cidr,
                gateway,
            } => &format!("provided CIDR range {} does not contain gateway IPv4 {}", cidr, gateway),
            Other (t) => t.as_str(),
        };

        write!(f, "{}", error)
    }
}

impl<T> From<T> for ProtonError
    where T: Error
{
    fn from(e: T) -> ProtonError {
        let string = if let Some (err) = e.source() {
            err.to_string()
        } else {
            String::new()
        };

        ProtonError::Other (string)
    }
}