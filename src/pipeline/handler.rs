use pipeline::InboundHandlerContext;


pub trait InboundHandler {
	type RIn: Send+'static;
	type ROut: Send+'static;
	type E: Send+'static;

	fn read<WOut: Send+'static>(&self, ctx: &mut InboundHandlerContext<Self::RIn, Self::ROut,Self::E,WOut>, i: Self::RIn);
}



#[cfg(test)]
mod tests {
    use super::*;
}