use future::Future;

pub trait Service<Req,Resp,E>
where Resp: Send+'static,
	  E: Send+'static {

    fn apply(&self, r: Req) -> Future<Resp, E>;
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
	  EOut: Send+'static  {
    fn filter<S: Service<ReqOut, RespIn, EIn>>(&self, r: ReqIn, s: S) -> Future<RespOut, EOut>;
}

pub trait SimpleFilter<Req,Resp,E>
where Resp: Send+'static,
	  E: Send+'static {
    fn filter<S: Service<Req, Resp, E>>(&self, r: Req, s: S) -> Future<Resp, E>;
}

impl<T, Req, Resp, E> Filter<Req, Resp, E, Req, Resp, E> for T
where Req: Send+'static,
      Resp: Send+'static,
	  E: Send+'static,
	  T: SimpleFilter<Req,Resp,E>  {

    fn filter<S: Service<Req, Resp, E>>(&self, r: Req, s: S) -> Future<Resp, E> {
        SimpleFilter::filter(self, r, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use future::{Async, Future};
    use testutils::marker;

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

        struct F;
        impl SimpleFilter<&'static str,&'static str,()> for F {
            fn filter<S: Service<&'static str, &'static str, ()>>(&self,
                                                                  r: &'static str,
                                                                  s: S)
                                                                  -> Future<&'static str, ()> {
                s.apply("blabla")
            }
        }
        let f = F;

        Filter::filter(&f, "hello", s).receive(move |r| {
            set_marker();
            assert_eq!(r, Ok("blabla"));
        });
        assert_marker();
    }

}
