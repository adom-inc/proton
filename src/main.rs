//! Main executable for Proton.

#![deny(warnings)]

use std::{
    thread::sleep,
    time::{
        Duration,
        Instant,
    },
};

use pnet::{
    packet::{
        ethernet::{
            EtherTypes,
            MutableEthernetPacket,
        },
        Packet,
    },
    datalink::{
        channel,
        Channel,
        interfaces,
        MacAddr,
        NetworkInterface,
    },
};

fn main() {
    // Get interface
    let check_iface = |i: &NetworkInterface| i.name == "wlp4s0";
    let interface = interfaces()
        .into_iter()
        .find(check_iface)
        .unwrap();
    
    // Open Ethernet channel
    let channel = channel(&interface, Default::default()).unwrap();
    let (mut tx, mut rx) = if let Channel::Ethernet (tx, rx) = channel {
        (tx, rx)
    } else {
        todo!();
    };

    let mut frame = MutableEthernetPacket::owned(vec![0u8; 48]).unwrap();

    frame.set_destination(MacAddr::broadcast());
    frame.set_source(interface.mac.unwrap());

    loop {
        tx.send_to(
            frame.packet(),
            None,
        );

        println!("Sent {}", pretty_print(frame.packet()));

        sleep(Duration::from_micros(102400));
    }
}

fn pretty_print(packet: &[u8]) -> String {
    let mut output = String::new();

    for val in packet {
        output.push_str(&format!("{:02x} ", val));
    }

    output
}