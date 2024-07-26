//! Example code to list wireless devices connected to an access point.

use std::net::Ipv4Addr;

use cidr::Ipv4Cidr;

use proton::{
    ap::AccessPoint,
    device::Device,
    error::ProtonResult,
};

#[tokio::main]
async fn main() -> ProtonResult<()> {
    let ifname = "wlp4s0";

    let mut ap = AccessPoint::new(
        Ipv4Cidr::new(                      // Internal network range
            Ipv4Addr::new(192, 168, 0, 0),  // Network address
            24,                             // Network length
        ).unwrap(),
        ifname,
    )?;

    println!("Scanning network interface: {}...", ifname);

    let devices: Vec<Device> = ap.scan().await?;

    println!("Found: {:#?}", devices);

    Ok (())
}