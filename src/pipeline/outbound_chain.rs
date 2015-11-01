use pipeline::{OutboundHandler};
use pipeline::{OutboundHandlerContext};
use future::Future;

pub struct OutboundPipelineChain<H: OutboundHandler>  {
	h: H,
	n: Option<Box<NextOutbound<H::WOut>>>
}

impl<H: OutboundHandler> OutboundPipelineChain<H> {
    pub fn new(h: H) -> OutboundPipelineChain<H> {
    	OutboundPipelineChain {
    		h: h,
    		n: None
    	}
    }

    pub fn then<N: OutboundHandler<WIn=H::WOut>+'static>(&mut self, n: N) -> &mut Box<OutboundPipelineChain<N>> {
    	use std::mem::transmute;

    	self.n = Some(Box::new(OutboundPipelineChain::new(n)));
    	unsafe { transmute(self.n.as_mut().unwrap()) }
    }

    pub fn write(&self, win: H::WIn) -> Future<(),H::E> {
    	let mut ctx = OutboundHandlerContext::new(&self.n);
    	self.h.write(&mut ctx, win);
        Future::value(())
    }
}

pub trait NextOutbound<WIn> {
	fn fire_write(&self, win: WIn);
}

impl<H: OutboundHandler> NextOutbound<H::WIn> for OutboundPipelineChain<H> {
	fn fire_write(&self, win: H::WIn) {
		self.write(win);
	}
}