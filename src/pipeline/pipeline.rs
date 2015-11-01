use std::marker::PhantomData;

use pipeline::InboundHandler;
use pipeline::InboundHandlerContext;

pub struct Pipeline<I: InboundHandler> {
	i: Option<InboundPipelineChain<I>>
}

impl<I: InboundHandler> Pipeline<I> {
    pub fn new() -> Pipeline<I> {
    	Pipeline {
    		i: None
    	}
    }

    pub fn inbound(&mut self, i: I) -> &mut InboundPipelineChain<I> {
    	self.i = Some(InboundPipelineChain::new(i));
    	self.i.as_mut().unwrap()
    }

    pub fn read(&self, rin: I::RIn) {
    	self.i.as_ref().unwrap().fire_read(rin);
    }
}

pub struct InboundPipelineChain<H: InboundHandler>  {
	h: H,
	n: Option<Box<Next<H::ROut>>>
}

impl<H: InboundHandler> InboundPipelineChain<H> {
    fn new(h: H) -> InboundPipelineChain<H> {
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
    	let mut ctx = InboundHandlerContext::new();
    	self.h.read::<()>(&mut ctx, rin);
    }
}

pub trait Next<RIn> {
	fn fire_read(&self, rin: RIn);
}

impl<H: InboundHandler> Next<H::RIn> for InboundPipelineChain<H> {
	fn fire_read(&self, rin: H::RIn) {
		self.read(rin)
	}
}


#[cfg(test)]
mod tests {
	use std::num::ParseIntError;

    use super::*;
    use pipeline::InboundHandler;
    use pipeline::InboundHandlerContext;

    use testutils::marker;

    struct StringToIntInbound { read_f: Box<Fn(&str)> }

    impl StringToIntInbound {
        fn new<F>(f: F) -> StringToIntInbound 
         where F: Fn(&str)+Send+'static
        {
        	StringToIntInbound {
        		read_f: Box::new(f)
        	}
        }
    }

    impl InboundHandler for  StringToIntInbound{
        type RIn = String;
        type ROut = i64;
        type E = ParseIntError;

        fn read<WOut: Send+'static>(&self, ctx: &mut InboundHandlerContext<String, i64, ParseIntError, WOut>, i: String) {
        	self.read_f.call((&i,))
        }
    }

    struct CaptureIntInbound {
    	read_f: Box<Fn(i64)>
    }

    impl CaptureIntInbound {
        fn new<F>(f: F) -> CaptureIntInbound 
         where F: Fn(i64)+Send+'static
        {
        	CaptureIntInbound {
        		read_f: Box::new(f)
        	}
        }
    }

    impl InboundHandler for CaptureIntInbound{
        type RIn = i64;
        type ROut = ();
        type E = ();

        fn read<WOut: Send+'static>(&self, ctx: &mut InboundHandlerContext<i64, (), (), WOut>, i: i64) {
        	self.read_f.call((i,));
        }
    }

    #[test]
    fn inbound() {
    	let (set_marker, assert_marker) = marker();
    	let s2int = StringToIntInbound::new(move |s| {
    		set_marker();
    		assert_eq!(s, "333");
    	});
    	
    	let mut p = Pipeline::new();
    	p.inbound(s2int);
    	p.read("333".to_owned());
    	assert_marker();
    }

    #[test]
    fn then() {
    	let (set_marker, assert_marker) = marker();
    	let s2int = StringToIntInbound::new(|_|{});
    	let capt = CaptureIntInbound::new(move |r| {
    		assert_eq!(r, 333);
    		set_marker();
    	});

    	let mut p = Pipeline::new();
    	p.inbound(s2int)
    	.then(capt);

    	p.read("333".to_owned());
    }
}