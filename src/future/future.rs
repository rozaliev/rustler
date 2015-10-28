use super::core::{Core, State};

pub struct Future<T: Send+'static, E: Send+'static> {
	core: Core<T,E>,
}

impl<T: Send+'static, E: Send+'static> Future<T,E> {
    pub fn value(v: T) -> Future<T,E> {
    	Future {
    		core: Core::from_result(Ok(v))
    	}
    }

    pub fn error(e: E) -> Future<T,E> {
    	Future {
    		core: Core::from_result(Err(e))
    	}
    }

    pub fn is_ready(&self) -> bool {
    	self.core.state() == State::Armed
    }


}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn value() {
		let f = Future::<_,()>::value(3usize);
		assert!(f.is_ready());
	}

	#[test]
	fn error() {
		let f = Future::<usize,_>::error(());
		assert!(f.is_ready());
	}
}