use super::core::{Core, State};
use super::promise::Promise;

pub struct Future<T: Send+'static, E: Send+'static> {
	core: Core<T,E>,
}

impl<T: Send+'static, E: Send+'static> Future<T,E> {
    pub fn from_core(core: Core<T,E>) -> Future<T,E> {
        Future {
            core: core
        }
    }

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

    pub fn then<F,G>(&mut self, f:F) -> Future<G,E>
    where F: FnOnce(Result<T,E>) -> Result<G,E>,
          F: Send+'static,
          G: Send+'static
    {
        let (future, mut promise) = Promise::pair();
        if self.is_ready() {
            let r = self.core.take_result();
            self.core.executor().add(Box::new(move || {
                promise.resolve(f(r))
            }));
        } else {    
            self.core.set_callback(move |r| {
                promise.resolve(f(r))
            });    
        }
        
        future
    }


}


#[cfg(test)]
mod tests {
	use super::*;
    use testutils::marker;

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

    #[test]
    fn then() {
        let (set_marker, assert_marker) = marker();

        let f = Future::<(),()>::value(())
        .then(move |_| Ok(1))
        .then(move |r| Ok(r.unwrap()+333))
        .then(move |r| Ok(r.unwrap()+2))
        .then(move |r| {
            assert_eq!(r, Ok(336));
            set_marker();
            Ok(())
        });

        assert_marker();
    }
}