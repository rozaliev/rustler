use std::marker::PhantomData;

pub struct InboundHandlerContext<RIn,ROut,E,WIn>{
	phantom: PhantomData<(RIn, ROut, E, WIn)>,
}

impl<RIn,ROut,E,WIn> InboundHandlerContext<RIn,ROut,E,WIn> {
    pub fn new() -> InboundHandlerContext<RIn,ROut,E,WIn> {
    	InboundHandlerContext {
    		phantom: PhantomData
    	}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}