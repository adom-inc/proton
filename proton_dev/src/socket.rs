//! Socket utilities for device discovery.

use neli::{
    consts::{
        NlmF,
        Nlmsg,
    },
    genl::Genlmsghdr,
    nl::Nlmsghdr,
    nlattr::Nlattr,
    err::NlError,
};

use nl80211::{
    Nl80211Attr,
    Nl80211Cmd,
    ParseNlAttr,
    Socket,
    Station,
    NL_80211_GENL_VERSION,
};

use crate::NetlinkResult;

/// A wireless AP with a number of connected stations.
pub trait NetworkSocket {
    /// Get all stations connected to this AP.
    /// 
    /// # Parameters
    /// - `nlif_index` (`&[u8]`): a Netlink network interface index
    /// 
    /// # Returns
    /// `NetlinkResult<Vec<Station>>` containing a list of stations.
    fn get_all_stations(&mut self, nlif_index: &[u8]) -> NetlinkResult<Vec<Station>>;
}

impl NetworkSocket for Socket {
    fn get_all_stations(
        &mut self,
        nlif_index: &[u8],
    ) -> NetlinkResult<Vec<Station>> {
        // Get the Netlink socket
        let nl80211sock = &mut self.sock;

        let mut attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];
        let new_attr = Nlattr::new(
            None,
            Nl80211Attr::AttrIfindex,
            nlif_index.to_owned(),
        )?;
        attrs.push(new_attr);

        let genlhdr = Genlmsghdr::new(Nl80211Cmd::CmdGetStation, NL_80211_GENL_VERSION, attrs)?;
        
        let len = None;
        let nl_type = self.family_id;
        let flags = vec![NlmF::Request, NlmF::Dump];
        let seq = None;
        let pid = None;
        let payload = genlhdr;
        let nlhdr = Nlmsghdr::new(len, nl_type, flags, seq, pid, payload);

        nl80211sock.send_nl(nlhdr)?;

        let mut results = Vec::new();

        let mut iter = nl80211sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();

        while let Some(Ok(response)) = iter.next() {
            match response.nl_type {
                Nlmsg::Error => return Err (NlError::Msg ("could not get device information".to_string())),
                Nlmsg::Done => break,
                _ => {
                    let handle = response.nl_payload.get_attr_handle();
                    results.push(Station::default().parse(handle));
                },
            };
        }

        Ok(results)
    }
}