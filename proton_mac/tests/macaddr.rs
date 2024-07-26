//! Testing the MAC address implementation.

use pnet::datalink::MacAddr as PnetMac;

use proton_mac::MacAddr;

#[test]
fn display_mac_addr() {
    // Construct a MAC address
    let mac: MacAddr = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab].into();

    // Call `Display::fmt` on this address
    let mac_string = format!("{}", mac);

    assert_eq!(&mac_string, "12:34:56:78:90:ab");
}

#[test]
fn debug_mac_addr() {
    // Construct a MAC address
    let mac: MacAddr = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab].into();

    // Call `Debug::fmt` on this address
    let mac_string = format!("{:#?}", mac);

    assert_eq!(&mac_string, "12:34:56:78:90:ab");
}

#[test]
fn from_pnet_mac() {
    // Construct a `libpnet` MAC address
    let mac: PnetMac = PnetMac::new(0x12, 0x34, 0x56, 0x78, 0x90, 0xab);

    // Convert this into a native MAC address
    let native_mac: MacAddr = mac.into();

    let ref_mac: MacAddr = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab].into();

    assert_eq!(native_mac, ref_mac);
}