use std::marker::PhantomData;

use mio::*;
use mio::tcp::*;

use pipeline::{Pipeline, PipelineFactory};
use async::EventBase;

pub struct Conn<P: PipelineFactory> {
    sock: TcpStream,
    token: Token,
    p: PhantomData<P>,
}

impl<P: PipelineFactory> Conn<P> {
    pub fn new(sock: TcpStream, token: Token) -> Conn<P> {
        Conn {
            sock: sock,
            token: token,
            p: PhantomData,
        }
    }

    pub fn transport_active(&mut self, event_loop: &mut EventLoop<EventBase<P>>) {
        event_loop.register_opt(&self.sock,
                                self.token,
                                EventSet::all(),
                                PollOpt::edge() | PollOpt::oneshot())
                  .unwrap();
    }
    pub fn readable(&mut self, event_loop: &mut EventLoop<EventBase<P>>) {
    }
    pub fn writable(&mut self, event_loop: &mut EventLoop<EventBase<P>>) {
    }
}
