use std::marker::PhantomData;

use pipeline::{InboundHandler, OutboundHandler};
use pipeline::{InboundHandlerContext, OutboundHandlerContext};
use pipeline::{InboundPipelineChain, OutboundPipelineChain};
use pipeline::{NextInbound, NextOutbound};

pub trait PipelineFactory {
    type I: InboundHandler;
    type O: OutboundHandler;

    fn pipeline() -> Pipeline<Self::I, Self::O>;
}

pub struct Pipeline<I: InboundHandler, O: OutboundHandler> {
    i: Option<InboundPipelineChain<I, O::WIn>>,
    o: Option<OutboundPipelineChain<O>>,
}

impl<I: InboundHandler, O: OutboundHandler> Pipeline<I, O> {
    pub fn new() -> Pipeline<I, O> {
        Pipeline { i: None, o: None }
    }

    pub fn inbound(&mut self, i: I) -> &mut InboundPipelineChain<I, O::WIn> {
        self.i = Some(InboundPipelineChain::new(i));
        self.i.as_mut().unwrap()
    }

    pub fn outbound(&mut self, o: O) -> &mut OutboundPipelineChain<O> {
        self.o = Some(OutboundPipelineChain::new(o));
        self.o.as_mut().unwrap()
    }

    pub fn read(&self, rin: I::RIn) {
        self.i.as_ref().unwrap().fire_read(rin);
    }

    pub fn write(&self, win: O::WIn) {
        self.o.as_ref().unwrap().fire_write(win);
    }

    pub fn transport_active(&self) {}
}




#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::*;
    use future::Future;
    use pipeline::{InboundHandler, OutboundHandler};
    use pipeline::{InboundHandlerContext, OutboundHandlerContext};

    use testutils::marker;

    struct StringToInt {
        cb: Box<Fn(&str)>,
    }

    impl StringToInt {
        fn new<F>(f: F) -> StringToInt
            where F: Fn(&str) + Send + 'static
        {
            StringToInt { cb: Box::new(f) }
        }
    }

    impl InboundHandler for  StringToInt {
        type RIn = String;
        type ROut = i64;
        type E = ParseIntError;

        fn read<WOut: Send + 'static>(&self,
                                      ctx: &mut InboundHandlerContext<String,
                                                                      i64,
                                                                      ParseIntError,
                                                                      WOut>,
                                      i: String) {
            self.cb.call((&i,));
            let r = i64::from_str_radix(&i, 10);
            ctx.fire_read(r.unwrap())
        }
    }

    impl OutboundHandler for  StringToInt {
        type WIn = i64;
        type WOut = String;
        type E = ();

        fn write(&self,
                 ctx: &mut OutboundHandlerContext<i64, String, ()>,
                 i: i64)
                 -> Future<(), ()> {
            self.cb.call((&i.to_string(),));
            ctx.fire_write(i.to_string());
            Future::value(())
        }
    }

    struct CaptureInt {
        cb: Box<Fn(i64)>,
    }

    impl CaptureInt {
        fn new<F>(f: F) -> CaptureInt
            where F: Fn(i64) + Send + 'static
        {
            CaptureInt { cb: Box::new(f) }
        }
    }

    impl InboundHandler for CaptureInt{
        type RIn = i64;
        type ROut = i64;
        type E = ();

        fn read<WOut: Send + 'static>(&self,
                                      ctx: &mut InboundHandlerContext<i64, i64, (), WOut>,
                                      i: i64) {
            self.cb.call((i,));
            ctx.fire_read(i);
        }
    }

    impl OutboundHandler for CaptureInt {
        type WIn = i64;
        type WOut = i64;
        type E = ();

        fn write(&self, ctx: &mut OutboundHandlerContext<i64, i64, ()>, i: i64) -> Future<(), ()> {
            self.cb.call((i,));
            ctx.fire_write(i);
            Future::value(())
        }
    }

    #[test]
    fn inbound() {
        let (set_marker, assert_marker) = marker();
        let s2int = StringToInt::new(move |s| {
            set_marker();
            assert_eq!(s, "333");
        });

        let mut p = Pipeline::new();
        p.inbound(s2int);
        p.outbound(StringToInt::new(|_| {}));
        p.read("333".to_owned());
        assert_marker();
    }

    #[test]
    fn inbound_then() {
        let (set_marker, assert_marker) = marker();
        let s2int = StringToInt::new(|_| {});
        let capt = CaptureInt::new(move |r| {
            assert_eq!(r, 333);
            set_marker();
        });

        let mut p = Pipeline::new();
        p.outbound(StringToInt::new(|_| {}));
        p.inbound(s2int).then(capt);

        p.read("333".to_owned());
        assert_marker();
    }

    #[test]
    fn outbound() {
        let (set_marker, assert_marker) = marker();
        let s2int = StringToInt::new(move |s| {
            set_marker();
            assert_eq!(s, "333");
        });

        let mut p = Pipeline::new();
        p.inbound(StringToInt::new(|_| {}));
        p.outbound(s2int);

        p.write(333);
        assert_marker();
    }

    #[test]
    fn outbound_then() {
        let (set_marker, assert_marker) = marker();
        let s2int = StringToInt::new(move |s| {
            set_marker();
            assert_eq!(s, "333");
        });
        let capt = CaptureInt::new(move |_| {});

        let mut p = Pipeline::new();
        p.inbound(StringToInt::new(|_| {}));
        p.outbound(capt).then(s2int);

        p.write(333);
        assert_marker();
    }
}
