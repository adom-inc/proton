//! MAC address policy.

use pnet::datalink::MacAddr;

use crate::MacAddrPolicyError;

/// Result type for MAC address policy actions.
pub type PolicyResult = Result<(), MacAddrPolicyError>;

/// A MAC address policy.
/// 
/// This defines the policy by which MAC addresses (hardware addresses
/// on Layer 2 of the OSI Model) are permitted to join the wireless access
/// point and send and receive traffic.
pub enum MacAddrPolicy {
    /// All MAC addresses may join the access point.
    Public,

    /// Only a specified list of MAC addresses may join the access point.
    Whitelist (Vec<MacAddr>),

    /// All MAC addresses except a specified list may join the access point.
    Blacklist (Vec<MacAddr>),
}

impl MacAddrPolicy {
    /// Create a new public MAC address policy.
    ///
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new `MacAddrPolicy`.
    pub fn public() -> Self {
        Self::Public
    }

    /// Create a new MAC address whitelist policy.
    ///
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new `MacAddrPolicy` with an empty whitelist.
    pub fn whitelist() -> Self {
        Self::Whitelist (Vec::new())
    }

    /// Create a new MAC address blacklist policy.
    ///
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A new `MacAddrPolicy` with an empty blacklist.
    pub fn blacklist() -> Self {
        Self::Blacklist (Vec::new())
    }
    
    /// Add a MAC address to the whitelist.  The policy must be initialized
    /// as a whitelist.
    /// 
    /// # Parameters
    /// - `device` (`MacAddr`): the MAC address of the device allowed
    /// 
    /// # Returns
    /// A `PolicyResult` indicating whether or not the whitelisting was successful.
    /// This method will return an error if the policy was not a whitelist policy.
    pub fn allow(&mut self, device: MacAddr) -> PolicyResult {
        if let Self::Whitelist (wl) = self {
            wl.push(device);
            Ok (())
        } else {
            Err (MacAddrPolicyError::NotWhitelistPolicy)
        }
    }

    /// Add a MAC address to the blacklist.  The policy must be initialized
    /// as a blacklist.
    /// 
    /// # Parameters
    /// - `device` (`MacAddr`): the MAC address of the device denied
    /// 
    /// # Returns
    /// A `PolicyResult` indicating whether or not the blacklisting was successful.
    /// This method will return an error if the policy was not a blacklist policy.
    pub fn deny(&mut self, device: MacAddr) -> PolicyResult {
        if let Self::Blacklist (bl) = self {
            bl.push(device);
            Ok (())
        } else {
            Err (MacAddrPolicyError::NotBlacklistPolicy)
        }
    }

    /// Check if a MAC address is permitted by the policy.
    /// 
    /// # Parameters
    /// - `address` (`MacAddr`): the MAC address to be checked
    /// 
    /// # Returns
    /// A `bool` indicating whether or not the MAC address is permitted
    /// by the policy.
    pub fn check(&self, address: MacAddr) -> bool {
        match self {
            // A public policy permits all MAC addresses
            Self::Public => true,

            // A whitelist allows only some MAC addresses
            Self::Whitelist (wl) => wl.contains(&address),

            // A blacklist denies only some MAC addresses
            Self::Blacklist (bl) => !bl.contains(&address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_public_mac_policy() {
        // Create a public MAC address policy
        let policy = MacAddrPolicy::public();

        // Define arbitrary MAC addresses
        let mac1 = MacAddr::new(32, 103, 244, 102, 34, 1);
        let mac2 = MacAddr::new(250, 33, 39, 3, 48, 73);
        let mac3 = MacAddr::new(49, 123, 86, 38, 20, 67);
        
        // Check MAC addresses
        assert!(policy.check(mac1));
        assert!(policy.check(mac2));
        assert!(policy.check(mac3));
    }

    #[test]
    fn create_whitelist_mac_policy() {
        // Create a whitelist MAC address policy
        let mut policy = MacAddrPolicy::whitelist();

        // Define arbitrary MAC addresses
        let mac1 = MacAddr::new(32, 103, 244, 102, 34, 1);
        let mac2 = MacAddr::new(250, 33, 39, 3, 48, 73);
        let mac3 = MacAddr::new(49, 123, 86, 38, 20, 67);

        // Allow one address on the policy
        let result = policy.allow(mac1);

        // Check for successful whitelisting
        assert_eq!(result, Ok (()));
        
        // Check MAC addresses
        assert!(policy.check(mac1));
        assert!(!policy.check(mac2));
        assert!(!policy.check(mac3));
    }

    #[test]
    fn create_blacklist_mac_policy() {
        // Create a blacklist MAC address policy
        let mut policy = MacAddrPolicy::blacklist();

        // Define arbitrary MAC addresses
        let mac1 = MacAddr::new(32, 103, 244, 102, 34, 1);
        let mac2 = MacAddr::new(250, 33, 39, 3, 48, 73);
        let mac3 = MacAddr::new(49, 123, 86, 38, 20, 67);

        // Deny one address on the policy
        let result = policy.deny(mac1);

        // Check for successful blacklisting
        assert_eq!(result, Ok(()));
        
        // Check MAC addresses
        assert!(!policy.check(mac1));
        assert!(policy.check(mac2));
        assert!(policy.check(mac3));
    }
}