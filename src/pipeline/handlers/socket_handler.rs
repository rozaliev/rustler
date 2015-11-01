use pipeline::InboundHandler;
use pipeline::InboundHandlerContext;


struct SocketHandler;

impl InboundHandler for SocketHandler {
	type RIn = ();
	type ROut = ();
	type E = ();

	fn read<WOut: Send+'static>(&self, ctx: &mut InboundHandlerContext<Self::RIn, Self::ROut,Self::E,WOut>, i: Self::RIn) {
		
	}
}