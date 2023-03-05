use futures::future::join_all;

use crate::message::Message as Msg;
use crate::proxy::Proxy;
use crate::ActorId;

use super::CfgMessage;

pub struct Proxies<P>
where
    P: Send,
{
    proxies: Vec<Proxy<Msg<P>>>,
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
    pub fn new() -> Self {
        Self {
            proxies: Default::default(),
        }
    }

    #[inline]
    pub fn add_proxy(&mut self, proxy: Proxy<Msg<P>>) {
        self.proxies.push(proxy)
    }

    pub fn handle_msg(&mut self, msg: CfgMessage<Msg<P>>) {
        match msg {
            CfgMessage::AddProxy(pxy) => self.add_proxy(pxy),
        }
    }

    pub async fn send_all_except(&mut self, sid: &ActorId, msg: Msg<P>, except: &[ActorId])
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

    pub fn try_send_all_except(&mut self, sid: &ActorId, msg: Msg<P>, except: &[ActorId])
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

    pub fn do_send_all_except(&mut self, sid: &ActorId, msg: Msg<P>, except: &[ActorId])
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
