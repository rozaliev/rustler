use pipeline::{InboundHandlerContext, OutboundHandlerContext};
use future::Future;


pub trait InboundHandler {
	type RIn: Send+'static;
	type ROut: Send+'static;
	type E: Send+'static;

	fn read<WOut: Send+'static>(&self, ctx: &mut InboundHandlerContext<Self::RIn, Self::ROut,Self::E,WOut>, i: Self::RIn);
}

pub trait OutboundHandler {
	type WIn: Send+'static;
	type WOut: Send+'static;
	type E: Send+'static;

	fn write(&self, ctx: &mut OutboundHandlerContext<Self::WIn, Self::WOut,Self::E>, i: Self::WIn) -> Future<(),Self::E>;
}



#[cfg(test)]
mod tests {
    use super::*;
}