use future::Future;
use std::marker::PhantomData;

pub trait Service<Req,Resp,E>
where Resp: Send+'static,
	  E: Send+'static {

    fn apply(&self, r: Req) -> Future<Resp, E>;
}

struct ConstService<Req,Resp,E> 
where Resp: Send+'static,
      E: Send+'static
{
    f: Box<Fn(Req) -> Future<Resp,E>+ Send+'static>
}


impl<Req,Resp,E>  ConstService<Req,Resp,E> 
where Resp: Send+'static,
      E: Send+'static
{
    fn new<F>(f: F) -> ConstService<Req,Resp,E> 
    where F: Fn(Req) -> Future<Resp,E>+ Send+'static
    {
        ConstService {
            f: Box::new(f)
        }
    }
}


impl<Req,Resp,E> Service<Req,Resp,E> for ConstService<Req,Resp,E> 
where Resp: Send+'static,
      E: Send+'static
{
    fn apply(&self, r: Req) -> Future<Resp, E> {
        self.f.call((r,))
    }
}

impl<T, Req,Resp,E> Service<Req,Resp,E> for T
where T: Fn(Req) -> Future<Resp,E>,
	  Resp: Send+'static,
	  E: Send+'static {

    fn apply(&self, r: Req) -> Future<Resp, E> {
        self(r)
    }
}


pub trait Filter<ReqIn, RespIn, EIn, ReqOut, RespOut, EOut>
where RespOut: Send+'static,
      RespIn: Send+'static,
	  EIn: Send+'static,
	  EOut: Send+'static,
      Self: Sized+Send+'static  {
    fn filter<S: Service<ReqOut, RespIn, EIn>>(&self, r: ReqIn, s: &S) -> Future<RespOut, EOut>;

    fn then_service<S: Service<ReqOut, RespIn, EIn>+Send+'static>(self, s: S) -> ConstService<ReqIn, RespOut, EOut> {
        ConstService::new(move |r| {
            self.filter(r, &s)
        })
    }
}

pub trait SimpleFilter<Req,Resp,E>
where Resp: Send+'static,
	  E: Send+'static {
    fn filter<S: Service<Req, Resp, E>>(&self, r: Req, s: &S) -> Future<Resp, E>;
}

impl<T, Req, Resp, E> Filter<Req, Resp, E, Req, Resp, E> for T
where Req: Send+'static,
      Resp: Send+'static,
	  E: Send+'static,
	  T: SimpleFilter<Req,Resp,E>,
      T: Send+'static {

    fn filter<S: Service<Req, Resp, E>>(&self, r: Req, s: &S) -> Future<Resp, E> {
        SimpleFilter::filter(self, r, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use future::{Async, Future};
    use testutils::marker;

    struct F;
    impl SimpleFilter<&'static str,&'static str,()> for F {
        fn filter<S: Service<&'static str, &'static str, ()>>(&self,
                                                              r: &'static str,
                                                              s: &S)
                                                              -> Future<&'static str, ()> {
            s.apply("blabla")
        }
    }

    #[test]
    fn service() {
        let (set_marker, assert_marker) = marker();

        let s = |r| Future::<_, ()>::value(r);

        s.apply("hello").receive(move |r| {
            set_marker();
            assert_eq!(r, Ok("hello"));
        });
        assert_marker();
    }

    #[test]
    fn simple_filter() {
        let (set_marker, assert_marker) = marker();

        let s = |r| Future::<_, ()>::value(r);
        
        let f = F;

        Filter::filter(&f, "hello", &s).receive(move |r| {
            set_marker();
            assert_eq!(r, Ok("blabla"));
        });
        assert_marker();
    }

    #[test]
    fn filter_then_service() {
        let (set_marker, assert_marker) = marker();

        let f = F;
        let s = |r| Future::<_, ()>::value(r);
        let ns = f.then_service(s);

        ns.apply("hello").receive(move |r| {
            set_marker();
            assert_eq!(r, Ok("blabla"));
        });
        assert_marker();
    }

}
