//! An example of a simple wireless access point managed by Proton.

use std::net::Ipv4Addr;

use cidr::Ipv4Cidr;

use proton::AccessPoint;


#[tokio::main]
async fn main() {
    let mut ap = AccessPoint::new(
        Ipv4Addr::new(10, 0, 0, 1),     // External IPv4 address
        Ipv4Cidr::new(                  // Internal network range
            Ipv4Addr::new(10, 0, 0, 0),     // Network address
            24,                             // Network length
        ).unwrap(),
        Default::default()              // MAC address management policy (default: public)
    );

    let result = ap.run().await;

    println!("{:#?}", result);
}