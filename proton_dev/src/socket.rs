//! Socket utilities for device discovery.

use neli::{
    consts::{
        NlmF,
        Nlmsg,
    },
    genl::Genlmsghdr,
    nl::Nlmsghdr,
    nlattr::Nlattr,
};

use nl80211::{
    Nl80211Attr,
    Nl80211Cmd,
    ParseNlAttr,
    Socket,
    Station,
    NL_80211_GENL_VERSION,
};

use proton_err::{
    ProtonError,
    ProtonResult,
};

// use proton_mac::MacAddr;

/// A wireless AP with a number of connected stations.
pub trait NetworkSocket {
    /// Get all stations connected to this AP.
    /// 
    /// # Parameters
    /// - `nlif_index` (`&[u8]`): a Netlink network interface index
    /// 
    /// # Returns
    /// `ProtonResult<Vec<Station>>` containing a list of network stations.
    fn get_all_stations(&mut self, nlif_index: &[u8]) -> ProtonResult<Vec<Station>>;
}

impl NetworkSocket for Socket {
    fn get_all_stations(
        &mut self,
        nlif_index: &[u8],
    ) -> ProtonResult<Vec<Station>> {
        // Get the Netlink socket
        let nl80211sock = &mut self.sock;

        // Set Generic Netlink attributes
        let mut attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];
        let new_attr = Nlattr::new(
            None,
            Nl80211Attr::AttrIfindex,
            nlif_index.to_owned(),
        )?;
        attrs.push(new_attr);

        // Construct the Generic Netlink header
        let genlhdr = Genlmsghdr::new(
            Nl80211Cmd::CmdGetStation,
            NL_80211_GENL_VERSION,
            attrs,
        )?;
        
        // Set the Netlink header length
        let len = None;

        // Set the Generic Netlink Family ID
        let nl_type = self.family_id;

        // Set the Netlink flags
        let flags = vec![NlmF::Request, NlmF::Dump];

        // Set the sequence number
        let seq = None;

        // Set the Netlink port ID
        let pid = None;

        // Set the Netlink header payload (contains Generic Netlink header)
        let payload = genlhdr;

        // Construct the Netlink header
        let nlhdr = Nlmsghdr::new(len, nl_type, flags, seq, pid, payload);

        // Send header to the Netlink socket
        nl80211sock.send_nl(nlhdr)?;

        // Read results back from the Netlink socket
        let mut results = Vec::new();
        let mut iter = nl80211sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();

        while let Some(Ok(response)) = iter.next() {
            match response.nl_type {
                Nlmsg::Error => return Err (ProtonError::CouldNotGetDeviceInformation),
                Nlmsg::Done => break,
                _ => {
                    let handle = response.nl_payload.get_attr_handle();
                    results.push(Station::default().parse(handle));
                },
            };
        }

        Ok (results)
    }

    // fn deauthenticate_by_mac(
    //     &mut self,
    //     mac: MacAddr,
    // ) -> ProtonResult<()> {
    //     // Get the Netlink socket
    //     let nl80211sock = &mut self.sock;

    //     // Convert the MAC address into a vector of octets
    //     let mac_vec: Vec<u8> = vec![
    //         mac.0,
    //         mac.1,
    //         mac.2,
    //         mac.3,
    //         mac.4,
    //         mac.5,
    //     ];

    //     // Set Generic Netlink attributes
    //     let mut attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];

    //     // Set MAC attribute (MAC address)
    //     let mac_attr = Nlattr::new(
    //         None,
    //         Nl80211Attr::AttrMac,
    //         mac_vec,
    //     )?;
    //     attrs.push(mac_attr);

    //     // Set Reason Code attribute (reason for deauthentication)
    //     let rc_attr = Nlattr::new(
    //         None,
    //         Nl80211Attr::AttrReasonCode,
    //         vec![1, 0], // unspecified reason
    //     )?;
    //     attrs.push(rc_attr);

    //     // Construct the Generic Netlink header
    //     let genlhdr = Genlmsghdr::new(
    //         Nl80211Cmd::CmdDeauthenticate,
    //         NL_80211_GENL_VERSION,
    //         attrs,
    //     )?;
        
    //     // Set the Netlink header length
    //     let len = None;

    //     // Set the Generic Netlink Family ID
    //     let nl_type = self.family_id;

    //     // Set the Netlink flags
    //     let flags = vec![NlmF::Request];

    //     // Set the sequence number
    //     let seq = None;

    //     // Set the Netlink port ID
    //     let pid = None;

    //     // Set the Netlink header payload (contains Generic Netlink header)
    //     let payload = genlhdr;

    //     // Construct the Netlink header
    //     let nlhdr = Nlmsghdr::new(len, nl_type, flags, seq, pid, payload);

    //     // Send header to the Netlink socket
    //     nl80211sock.send_nl(nlhdr)?;

    //     // Read results back from the Netlink socket
    //     let mut iter = nl80211sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();

    //     if let Some (Ok (response)) = iter.next() {
    //         dbg!(&response.nl_payload);

    //         match response.nl_type {
    //             Nlmsg::Done => Ok (()),
    //             _ => Err (ProtonError::CouldNotDeauthenticateDevice (mac)),
    //         }
    //     } else {
    //         Err (ProtonError::NoResponseFromNetlink)
    //     }
    // }
}