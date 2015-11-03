extern crate rustler;
extern crate mio;
extern crate iobuf;
extern crate env_logger;


use rustler::pipeline::{Pipeline, PipelineFactory};
use rustler::pipeline::handlers::SocketHandler;
use rustler::async::EventBase;
use rustler::async::eventbase;
use rustler::pipeline::{InboundHandlerContext, InboundHandler};


use mio::*;
use mio::tcp::*;
use iobuf::{AROIobuf};

struct EchoHandler;

impl InboundHandler for EchoHandler {
    type RIn = AROIobuf;
    type ROut = ();
    type E = ();

    fn read<WOut: Send + 'static>(&self,
                                  ctx: &mut InboundHandlerContext<Self::RIn,
                                                                  Self::ROut,
                                                                  Self::E,
                                                                  WOut>,
                                  i: Self::RIn) {
      println!("echo read: {:?}", i)
    }
}


struct EchoPipeline;

impl PipelineFactory for EchoPipeline {
	type I = SocketHandler;
	type O = SocketHandler;

    fn pipeline() -> Pipeline<SocketHandler, SocketHandler> {
        let mut p = Pipeline::new();
        p.inbound(SocketHandler::new()).then(EchoHandler);
        p.outbound(SocketHandler::new());

        p
    }
}

fn main() {
    let _ = env_logger::init();

    
    let lst = TcpListener::bind(&"0.0.0.0:9999".parse().unwrap()).unwrap();
    let mut event_loop = mio::EventLoop::new().unwrap();

    event_loop.register(&lst, eventbase::SERVER);

    let mut eb = EventBase::new(lst, EchoPipeline);

    event_loop.run(&mut eb);
}