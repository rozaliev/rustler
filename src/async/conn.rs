use std::marker::PhantomData;
use std::io::Read;

use mio::*;
use mio::tcp::*;

use pipeline::InboundHandler;

use iobuf::{AppendBuf, Iobuf, AROIobuf};

const READ_BUF_SIZE: usize = 2048;

use pipeline::{Pipeline, PipelineFactory};
use async::EventBase;

pub struct Conn<P: PipelineFactory> 
where 
    P::I: InboundHandler<RIn=AROIobuf> 
{
    sock: TcpStream,
    token: Token,
    p: PhantomData<P>,
    buf: AppendBuf<'static>,
    is_closed: bool
}

impl<P: PipelineFactory> Conn<P>
where 
    P::I: InboundHandler<RIn=AROIobuf> {
    pub fn new(sock: TcpStream, token: Token) -> Conn<P> {
        Conn {
            sock: sock,
            token: token,
            p: PhantomData,
            buf: AppendBuf::new(READ_BUF_SIZE),
            is_closed: false
        }
    }

    pub fn transport_active(&mut self, event_loop: &mut EventLoop<EventBase<P>>, pipeline: &Pipeline<P::I,P::O>) {
        pipeline.transport_active();
        self.register(event_loop);
    }
    pub fn readable(&mut self, event_loop: &mut EventLoop<EventBase<P>>, pipeline: &Pipeline<P::I,P::O>) {
        if self.buf.len() == 0 {
            self.buf = AppendBuf::new(READ_BUF_SIZE);
        }

        match self.sock.try_read(self.buf.as_mut_window_slice()) {
            Ok(Some(0)) => {
                debug!("conn token {:?} EOF", self.token);
                // EOF
                self.is_closed = true   
            }
            Ok(Some(n)) => {
                debug!("conn token read {:?} bytes", n);
                self.buf.advance(n as u32);
                let new_data = self.buf.atomic_slice_from(-(n as i32)-1).expect("can't get atomic buf");

                pipeline.read(new_data);
                self.register(event_loop);
            }
            Ok(None) => {
                debug!("conn token read would block");
                // would block
                self.register(event_loop)
            }
            Err(e) => {
                debug!("conn token read err: {:?}", e);
                self.is_closed = true
            }       
        }
    }
    pub fn writable(&mut self, event_loop: &mut EventLoop<EventBase<P>>, pipeline: &Pipeline<P::I,P::O>) {
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    fn register(&self, event_loop: &mut EventLoop<EventBase<P>>) {
        event_loop.register_opt(&self.sock,
                                self.token,
                                EventSet::all(),
                                PollOpt::edge() | PollOpt::oneshot())
                  .unwrap();
    }
}
