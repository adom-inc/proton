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
    let mut ap = AccessPoint::new(
        Ipv4Cidr::new(                      // Internal network range
            Ipv4Addr::new(192, 168, 0, 0),  // Network address
            24,                             // Network length
        ).unwrap(),
        "wlp4s0",
    )?;

    println!("Scanning network interface: {}...", "wlp4s0");

    let devices: Vec<Device> = ap.scan().await?;

    println!("Found: {:#?}", devices);

    Ok (())
}