use futures::future::join_all;

use crate::message::{ActorId, Message as Msg};
use crate::proxy::Proxy;

use super::CfgMessage;

pub struct Proxies<P>
where
    P: Send,
{
    proxies: Vec<Proxy<P>>,
}

impl<P> Default for Proxies<P>
where
    P: Send,
{
    fn default() -> Self {
        Self::new()
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
    pub fn add_proxy(&mut self, proxy: Proxy<P>) {
        self.proxies.push(proxy)
    }

    pub fn handle_msg(&mut self, msg: CfgMessage<P>) {
        match msg {
            CfgMessage::AddProxy(pxy) => self.add_proxy(pxy),
        }
    }

    pub async fn send_all_except(&mut self, msg: Msg<P>, except: &[ActorId])
    where
        P: Clone,
    {
        let futures = self
            .proxies
            .iter_mut()
            .filter(|pxy| !except.contains(pxy.aid()))
            .map(|pxy| pxy.send(msg.clone()));
        let _ = join_all(futures).await;
    }

    pub fn try_send_all_except(&mut self, msg: Msg<P>, except: &[ActorId])
    where
        P: Clone,
    {
        let _: Vec<_> = self
            .proxies
            .iter_mut()
            .filter(|pxy| !except.contains(pxy.aid()))
            .map(|pxy| pxy.try_send(msg.clone()))
            .collect();
    }

    pub fn do_send_all_except(&mut self, msg: Msg<P>, except: &[ActorId])
    where
        P: Clone,
    {
        let _: Vec<_> = self
            .proxies
            .iter_mut()
            .filter(|pxy| !except.contains(pxy.aid()))
            .map(|pxy| pxy.do_send(msg.clone()))
            .collect();
    }
}
