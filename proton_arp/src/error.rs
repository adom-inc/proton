//! ARP management errors.

use std::{
    error::Error,
    fmt,
};

use tokio::task::JoinError;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
/// An ARP management error.
pub enum ArpError {
    /// Could not find Wi-Fi interface.
    CouldNotFindWirelessInterface,

    /// Could not join asynchronous task.
    CouldNotJoinAsyncTask,
}

impl fmt::Display for ArpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ArpError::*;
        let error = match self {
            CouldNotFindWirelessInterface => "could not find wireless interface",
            CouldNotJoinAsyncTask => "could not join asynchronous task",
        };

        write!(f, "{}", error)
    }
}

impl Error for ArpError { }

impl From<JoinError> for ArpError {
    fn from(_: JoinError) -> Self {
        Self::CouldNotJoinAsyncTask
    }
}