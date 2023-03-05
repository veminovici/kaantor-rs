use actix::prelude::*;

use crate::proxy::Proxy;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum CfgMessage<P>
where
    P: Send,
{
    AddProxy(Proxy<P>),
}
