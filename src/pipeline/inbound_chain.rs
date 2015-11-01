use std::marker::PhantomData;
use pipeline::{InboundHandler};
use pipeline::{InboundHandlerContext};

pub struct InboundPipelineChain<H: InboundHandler, WIn: Send+'static>  {
	h: H,
    win: PhantomData<WIn>,
	n: Option<Box<NextInbound<H::ROut>>>
}

impl<H: InboundHandler, WIn: Send+'static> InboundPipelineChain<H,WIn> {
    pub fn new(h: H) -> InboundPipelineChain<H,WIn> {
    	InboundPipelineChain {
    		h: h,
            win: PhantomData,
    		n: None
    	}
    }

    pub fn then<N: InboundHandler<RIn=H::ROut>+'static>(&mut self, n: N) -> &mut Box<InboundPipelineChain<N,WIn>> {
    	use std::mem::transmute;

    	self.n = Some(Box::new(InboundPipelineChain::<_,WIn>::new(n)));
    	unsafe { transmute(self.n.as_mut().unwrap()) }
    }

    pub fn read(&self, rin: H::RIn) {
    	let mut ctx = InboundHandlerContext::new(&self.n);
    	self.h.read::<WIn>(&mut ctx, rin);
    }
}

pub trait NextInbound<RIn> {
	fn fire_read(&self, rin: RIn);
}

impl<H: InboundHandler, WIn: Send+'static> NextInbound<H::RIn> for InboundPipelineChain<H, WIn> {
	fn fire_read(&self, rin: H::RIn) {
		self.read(rin)
	}
}