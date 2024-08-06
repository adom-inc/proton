//! Define an abstraction over hotspot configuration data.

use std::net::Ipv4Addr;

use cidr::Ipv4Cidr;

use proton_err::{
    ProtonError,
    ProtonResult,
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
/// Define a structure holding a hotspot configuration.
pub struct HotspotConfig {
    /// SSID of the hotspot.
    pub ssid: String,

    /// Password of the hotspot.
    pub pass: String,

    /// Security type of the hotspot.
    pub security: String,

    /// Frequency band.
    pub band: String,

    /// IPv4 address of the access point (gateway address).
    pub gateway: Ipv4Addr,

    /// IPv4 CIDR address range of the network.
    pub cidr: Ipv4Cidr,
}

impl From<(String, String, String, String, String, String)> for HotspotConfig {
    fn from(config: (String, String, String, String, String, String)) -> Self {
        let (ssid, pass, cidr, gateway, security, band) = config;

        // Parse CIDR
        // Note: it's okay to use `Result::unwrap` here because we pass
        // `Ipv4Cidr::new` static arguments.
        let cidr = match parse_cidr(&cidr) {
            Ok (c) => c,
            Err (_) => Ipv4Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 24).unwrap(),
        };

        // Parse IPv4 gateway
        let gateway = match str::parse::<Ipv4Addr>(&gateway) {
            Ok (g) => g,
            Err (_) => Ipv4Addr::new(192, 168, 0, 1),
        };

        // Parse band
        let band = match band.as_str() {
            "2.4" => "bg",
            "5" => "a",
            _ => "bg", // default to 2.4 GHz
        }.to_string();

        Self {
            ssid,
            pass,
            security,
            gateway,
            cidr,
            band,
        }
    }
}

/// Parses an IPv4 CIDR.
fn parse_cidr(cidr: &str) -> ProtonResult<Ipv4Cidr> {
    // Split by slash
    let mut parts = cidr.split('/');

    // Get `Ipv4Addr`
    let ipv4 = str::parse::<Ipv4Addr>(
        parts.next().ok_or(ProtonError::CouldNotParseAsCidr (cidr.to_string()))?
    )?;

    // Get network length
    let length = str::parse::<u8>(
        parts.next().ok_or(ProtonError::CouldNotParseAsCidr (cidr.to_string()))?
    )?;

    Ok (Ipv4Cidr::new(ipv4, length)?)
}