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

#[derive(Debug)]
/// An error that occurred within the Proton library.
pub enum ProtonError {
    /// The provided interface was not an Ethernet interface, as expected.
    MustBeEthernetInterface,

    /// The program could not find any wireless network interfaces.
    CouldNotFindWirelessInterface,

    /// An error that could not be converted to a native error.
    Other (String),
}

impl Display for ProtonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ProtonError::*;
        let error = match self {
            MustBeEthernetInterface => "must be Ethernet interface",
            CouldNotFindWirelessInterface => "could not find wireless interface",
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