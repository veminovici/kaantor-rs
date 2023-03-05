use futures::future::join_all;

use crate::protocol::ProtocolMsg;
use crate::proxy::Proxy;
use crate::ActorId;

use super::CfgMessage;

/// A collection of `Proxy` instances which
/// can handle protocol message with a given `P` payload.
pub struct Proxies<P>
where
    P: Send,
{
    proxies: Vec<Proxy<ProtocolMsg<P>>>,
}

impl<P> Default for Proxies<P>
where
    P: Send,
{
    fn default() -> Self {
        Self {
            proxies: Default::default(),
        }
    }
}

impl<P> Proxies<P>
where
    P: Send,
{
    /// Creates a new instance of the collection.
    pub fn new() -> Self {
        Self {
            proxies: Default::default(),
        }
    }

    /// Adds a new proxy to the internal collection.
    #[inline]
    pub fn add_proxy(&mut self, proxy: Proxy<ProtocolMsg<P>>) {
        self.proxies.push(proxy)
    }

    /// Implements capabilities to handle a configuration message
    /// received by the actor.
    #[inline]
    pub fn handle_msg(&mut self, msg: CfgMessage<ProtocolMsg<P>>) {
        match msg {
            CfgMessage::AddProxy(pxy) => self.add_proxy(pxy),
        }
    }

    /// Sends a message to all neighbours except the ones from the list.
    pub async fn send_all_except(&mut self, sid: &ActorId, msg: ProtocolMsg<P>, except: &[ActorId])
    where
        P: Clone,
    {
        let futures = self
            .proxies
            .iter_mut()
            .filter(|pxy| !except.contains(pxy.aid()))
            .map(|pxy| pxy.send(sid, msg.clone()));
        let _ = join_all(futures).await;
    }

    /// Tries to send a message to all neighbours except the ones from the list.
    pub fn try_send_all_except(&mut self, sid: &ActorId, msg: ProtocolMsg<P>, except: &[ActorId])
    where
        P: Clone,
    {
        let _: Vec<_> = self
            .proxies
            .iter_mut()
            .filter(|pxy| !except.contains(pxy.aid()))
            .map(|pxy| pxy.try_send(sid, msg.clone()))
            .collect();
    }

    /// Does send a message to all neighbours except the ones from the list.
    pub fn do_send_all_except(&mut self, sid: &ActorId, msg: ProtocolMsg<P>, except: &[ActorId])
    where
        P: Clone,
    {
        let _: Vec<_> = self
            .proxies
            .iter_mut()
            .filter(|pxy| !except.contains(pxy.aid()))
            .map(|pxy| pxy.do_send(sid, msg.clone()))
            .collect();
    }
}
