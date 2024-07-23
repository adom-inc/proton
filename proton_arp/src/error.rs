//! ARP management errors.

use std::{
    error::Error,
    fmt,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
/// An ARP management error.
pub enum ArpError {
    
}

impl fmt::Display for ArpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            _ => "unrecognized error",
        };

        write!(f, "{}", error)
    }
}

impl Error for ArpError { }