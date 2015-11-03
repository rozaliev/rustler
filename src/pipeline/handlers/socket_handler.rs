use iobuf::AROIobuf;

use pipeline::{InboundHandler, OutboundHandler};
use pipeline::{InboundHandlerContext, OutboundHandlerContext};
use future::Future;

pub struct SocketHandler;


impl SocketHandler {
    pub fn new() -> SocketHandler {
        SocketHandler
    }
}

impl InboundHandler for SocketHandler {
	type RIn = AROIobuf;
	type ROut = AROIobuf;
	type E = ();

    fn read<WOut: Send + 'static>(&self,
                                  ctx: &mut InboundHandlerContext<Self::RIn,
                                                                  Self::ROut,
                                                                  Self::E,
                                                                  WOut>,
                                  i: Self::RIn) {
        debug!("async socker incomming: {:?}", i);
        ctx.fire_read(i)
    }
}


impl OutboundHandler for SocketHandler {
  type WIn = ();
  type WOut = ();
  type E = ();

    fn write(&self,
             ctx: &mut OutboundHandlerContext<Self::WIn, Self::WOut, Self::E>,
             i: Self::WIn)
             -> Future<(), Self::E> {
        Future::value(())
    }
}
