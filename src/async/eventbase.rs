use mio::*;
use mio::tcp::*;
use mio::util::Slab;

const MAX_CONNS_PER_LOOP: usize = 100_000;
const MAX_ACCEPTOR_THREADS: usize = 10;

pub const SERVER: Token = Token(0);

pub struct EventBase {
    lst: TcpListener,
    conns: Slab<Conn>,
}

impl EventBase {
    fn new(lst: TcpListener) -> EventBase {
        EventBase {
            lst: lst,
            conns: Slab::new_starting_at(Token(MAX_ACCEPTOR_THREADS), MAX_CONNS_PER_LOOP),
        }
    }
}

impl Handler for EventBase {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<EventBase>, token: Token, events: EventSet) {
        match token {
            SERVER => {
                if !events.is_readable() {
                    return;
                }

                match self.lst.accept() {
                    Ok(Some(sock)) => {
                        let token = self.conns.insert_with(|token| Conn::new(sock, token)).unwrap();

                        event_loop.register_opt(&self.conns[token].sock,
                                                token,
                                                EventSet::readable(),
                                                PollOpt::edge() | PollOpt::oneshot())
                                  .unwrap();
                    }
                    Ok(None) => {}
                    Err(e) => {
                        event_loop.shutdown();
                    }
                }
            }
            _ => {
                if let Some(conn) = self.conns.get_mut(token) {
                    if events.is_readable() {
                        conn.readable(event_loop);
                    }

                    if events.is_writable() {
                        conn.writable(event_loop)
                    }
                }
            }	
        }

    }
}

struct Conn {
    sock: TcpStream,
    token: Token,
}

impl Conn {
    fn new(sock: TcpStream, token: Token) -> Conn {
        Conn {
            sock: sock,
            token: token,
        }
    }

    fn readable(&mut self, event_loop: &mut EventLoop<EventBase>) {
    }
    fn writable(&mut self, event_loop: &mut EventLoop<EventBase>) {
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mio;
    use mio::*;
    use mio::tcp::*;

    #[test]
    fn init() {
        let lst = TcpListener::bind(&"0.0.0.0:9898".parse().unwrap()).unwrap();
        let mut event_loop = mio::EventLoop::new().unwrap();

        event_loop.register(&lst, SERVER);

        let mut eb = EventBase::new(lst);

        event_loop.run_once(&mut eb);
    }
}
