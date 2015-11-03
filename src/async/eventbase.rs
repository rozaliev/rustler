use std::sync::Arc;
use mio::*;
use mio::tcp::*;
use mio::util::Slab;

use iobuf::AROIobuf;


use pipeline::{Pipeline, PipelineFactory};
use pipeline::{InboundHandler};

use async::Conn;

const MAX_CONNS_PER_LOOP: usize = 100_000;
const MAX_ACCEPTOR_THREADS: usize = 10;

pub const SERVER: Token = Token(0);

pub struct EventBase<P: PipelineFactory> 
where 
    P::I: InboundHandler<RIn=AROIobuf> 
{
    lst: TcpListener,
    conns: Slab<Conn<P>>,
    pipeline: Arc<Pipeline<P::I, P::O>>,
}

impl<P: PipelineFactory> EventBase<P> 
where 
    P::I: InboundHandler<RIn=AROIobuf> 
{
    pub fn new(lst: TcpListener, factory: P) -> EventBase<P> {
        EventBase {
            lst: lst,
            conns: Slab::new_starting_at(Token(MAX_ACCEPTOR_THREADS), MAX_CONNS_PER_LOOP),
            pipeline: Arc::new(P::pipeline()),
        }
    }
}

impl<P: PipelineFactory> Handler for EventBase<P> 
where 
    P::I: InboundHandler<RIn=AROIobuf> 
{
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<EventBase<P>>, token: Token, events: EventSet) {
        match token {
            SERVER => {
                if !events.is_readable() {
                    return;
                }

                match self.lst.accept() {
                    Ok(Some(sock)) => {
                        debug!("incomming connection from: {:?}", sock.peer_addr());

                        let token = self.conns
                                        .insert_with(|token| {  Conn::new(sock, token) })
                                        .unwrap();
                        self.conns[token].transport_active(event_loop, &self.pipeline);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        event_loop.shutdown();
                    }
                }
            }
            _ => {
                if self.conns.contains(token) {
                    if events.is_readable() {
                        self.conns[token].readable(event_loop, &self.pipeline);
                    }

                    if events.is_writable() {
                        self.conns[token].writable(event_loop, &self.pipeline)
                    }

                    if self.conns[token].is_closed() {
                        self.conns.remove(token);    
                    }
                }
            }	
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mio;
    use mio::*;
    use mio::tcp::*;

    use pipeline::{Pipeline, PipelineFactory};
    use pipeline::handlers::SocketHandler;

    struct TestPipeline;

    impl PipelineFactory for TestPipeline {
    	type I = SocketHandler;
    	type O = SocketHandler;

        fn pipeline() -> Pipeline<SocketHandler, SocketHandler> {
            let mut p = Pipeline::new();
            p.inbound(SocketHandler::new());
            p.outbound(SocketHandler::new());

            p
        }
    }

    #[test]
    fn init() {
        let lst = TcpListener::bind(&"0.0.0.0:9898".parse().unwrap()).unwrap();
        let mut event_loop = mio::EventLoop::new().unwrap();

        event_loop.register(&lst, SERVER);

        let mut eb = EventBase::new(lst, TestPipeline);

        event_loop.run_once(&mut eb);
    }
}
