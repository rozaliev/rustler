#![feature(unboxed_closures)]
#![feature(core)]

extern crate rustler;

use rustler::service::{Service,SimpleFilter};
use rustler::future::{Future, Async};
use rustler::pipeline::{InboundHandler,OutboundHandler};
use rustler::pipeline::{InboundHandlerContext, OutboundHandlerContext};
use rustler::pipeline::{Pipeline};

use std::sync::Arc;

fn doubler(x: u64) -> Future<u64, ()> {
	Future::value(x * 2)
}

struct RequestLoggerFilter;

impl SimpleFilter<u64, u64, ()> for RequestLoggerFilter {
    fn filter<S: Service<u64, u64, ()>>(&self, r: u64, s: S) -> Future<u64, ()> {
    	println!("got a request {:?}", r);
    	s.apply(r).then(|v| {
    		println!("doubled: {}", v);
    		Ok(v)
    	})
    }
}

struct StringToIntHandler;

impl InboundHandler for StringToIntHandler {
    type RIn = String;
    type ROut = u64;
    type E = ();

    fn read<WOut: Send + 'static>(&self,
                                  ctx: &mut InboundHandlerContext<String,
                                                                  u64,
                                                                  (),
                                                                  WOut>,
                                  i: String) {
        let r = u64::from_str_radix(&i, 10);
        ctx.fire_read(r.unwrap())
    }
}

impl OutboundHandler for  StringToIntHandler {
    type WIn = u64;
    type WOut = String;
    type E = ();

    fn write(&self,
             ctx: &mut OutboundHandlerContext<u64, String, ()>,
             i: u64)
             -> Future<(), ()> {
        ctx.fire_write(i.to_string());
        Future::value(())
    }
}

struct SimpleServiceDispatcher<F: Fn(u64) -> Future<u64,()> + Send+'static> {
	f: Arc<F>
}

impl<F: Fn(u64) -> Future<u64,()> + Send+'static> SimpleServiceDispatcher<F> {
    fn new(f: F) -> SimpleServiceDispatcher<F> {
    	SimpleServiceDispatcher {
    		f: Arc::new(f)
    	}
    }
}

impl<F: Fn(u64) -> Future<u64,()> + Send+'static> InboundHandler for SimpleServiceDispatcher<F> {
    type RIn = u64;
    type ROut = ();
    type E = ();

    fn read<WOut: Send + 'static>(&self,
                                  ctx: &mut InboundHandlerContext<u64,
                                                                  (),
                                                                  (),
                                                                  WOut>,
                                  i: u64) {
        self.f.call((i,));
    }
}


fn main() {
	let s = |i| RequestLoggerFilter.filter(i, doubler);

	let mut p = Pipeline::new();
	p.inbound(StringToIntHandler).then(SimpleServiceDispatcher::new(move |i|{
		s(i)
	}));
	p.outbound(StringToIntHandler);

	p.read("111".to_string());
}