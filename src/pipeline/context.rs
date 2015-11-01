use std::marker::PhantomData;

use pipeline::{NextInbound, NextOutbound};

pub struct PipelineContext;

pub struct InboundHandlerContext<'a, RIn, ROut, E, WIn>
    where RIn: Send + 'static,
          ROut: Send + 'static,
          E: Send + 'static,
          WIn: Send + 'static
{
    n: &'a Option<Box<NextInbound<ROut>>>,
    phantom: PhantomData<(RIn, ROut, E, WIn)>,
}

impl<'a, RIn,ROut,E,WIn> InboundHandlerContext<'a, RIn,ROut,E,WIn>
where RIn: Send+'static,
	  ROut: Send+'static,
	  E: Send+'static,
	  WIn: Send+'static {
    pub fn new(n: &'a Option<Box<NextInbound<ROut>>>) -> InboundHandlerContext<'a, RIn, ROut, E, WIn> {
        InboundHandlerContext {
            n: n,
            phantom: PhantomData,
        }
    }

    pub fn fire_read(&self, rout: ROut) {
        if let &Some(ref n) = self.n {
            n.fire_read(rout);
        }
    }
}

pub struct OutboundHandlerContext<'a, WIn, WOut, E>
    where WIn: Send + 'static,
          WOut: Send + 'static,
          E: Send + 'static
{
    n: &'a Option<Box<NextOutbound<WOut>>>,
    phantom: PhantomData<(WIn, WOut, E)>,
}

impl<'a, WIn,WOut,E> OutboundHandlerContext<'a, WIn,WOut,E>
where WIn: Send+'static,
	  WOut: Send+'static,
	  E: Send+'static
	 {
    pub fn new(n: &'a Option<Box<NextOutbound<WOut>>>) -> OutboundHandlerContext<'a, WIn, WOut, E> {
        OutboundHandlerContext {
            n: n,
            phantom: PhantomData,
        }
    }

    pub fn fire_write(&self, wout: WOut) {
        if let &Some(ref n) = self.n {
            n.fire_write(wout);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
