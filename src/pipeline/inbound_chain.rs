use pipeline::{InboundHandler};
use pipeline::{InboundHandlerContext};

pub struct InboundPipelineChain<H: InboundHandler>  {
	h: H,
	n: Option<Box<NextInbound<H::ROut>>>
}

impl<H: InboundHandler> InboundPipelineChain<H> {
    pub fn new(h: H) -> InboundPipelineChain<H> {
    	InboundPipelineChain {
    		h: h,
    		n: None
    	}
    }

    pub fn then<N: InboundHandler<RIn=H::ROut>+'static>(&mut self, n: N) -> &mut Box<InboundPipelineChain<N>> {
    	use std::mem::transmute;

    	self.n = Some(Box::new(InboundPipelineChain::new(n)));
    	unsafe { transmute(self.n.as_mut().unwrap()) }
    }

    pub fn read(&self, rin: H::RIn) {
    	let mut ctx = InboundHandlerContext::new(&self.n);
    	self.h.read::<()>(&mut ctx, rin);
    }
}

pub trait NextInbound<RIn> {
	fn fire_read(&self, rin: RIn);
}

impl<H: InboundHandler> NextInbound<H::RIn> for InboundPipelineChain<H> {
	fn fire_read(&self, rin: H::RIn) {
		self.read(rin)
	}
}