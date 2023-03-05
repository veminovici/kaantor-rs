use actix::prelude::*;

use crate::proxy::Proxy;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum CfgMessage<M>
where
    M: Message + Send,
    M::Result: Send,
{
    AddProxy(Proxy<M>),
}
