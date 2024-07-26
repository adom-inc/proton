//! MAC address type.

use std::fmt::{
    Display,
    Debug,
    Formatter,
    Result,
};

use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Clone, Copy)]
/// A hardware (MAC) address consisting of six octets.
pub struct MacAddr (pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl Display for MacAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
        )
    }
}

impl Debug for MacAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(octets: [u8; 6]) -> Self {
        Self (
            octets[0],
            octets[1],
            octets[2],
            octets[3],
            octets[4],
            octets[5],
        )
    }
}

impl From<pnet::datalink::MacAddr> for MacAddr {
    fn from(mac: pnet::datalink::MacAddr) -> Self {
        Self (
            mac.0,
            mac.1,
            mac.2,
            mac.3,
            mac.4,
            mac.5,
        )
    }
}