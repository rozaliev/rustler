use future::Future;

trait Service<Req,Resp,E>
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

trait Filter<ReqIn, RespIn, EIn, ReqOut, RespOut, EOut>
where RespOut: Send+'static,
      RespIn: Send+'static,
	  EIn: Send+'static,
	  EOut: Send+'static  {
    fn apply<S: Service<ReqOut, RespIn, EIn>>(&self, r: ReqIn, s: S) -> Future<RespOut, EOut>;
}

trait SimpleFilter<Req,Resp,E>
where Resp: Send+'static,
      Req: Send+'static,
      E: Send+'static
      {
    fn apply<S: Service<Req, Resp, E>>(&self, r: Req, s: S) -> Future<Resp, E>;
}

impl<T, Req,Resp,E> Filter<Req,Resp,E, Req, Resp, E> for T
where T: SimpleFilter<Req,Resp,E>,
		Resp: Send+'static,
      Req: Send+'static,
      E: Send+'static{
    fn apply<S: Service<Req, Resp, E>>(&self, r: Req, s: S) -> Future<Resp, E> {
        SimpleFilter::apply(self, r, s)
    }
}
