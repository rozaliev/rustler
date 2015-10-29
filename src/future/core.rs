use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::boxed::FnBox;

use executor::{Executor, InlineExecutor};

use self::State::*;

#[derive(PartialEq, Eq, Debug)]
pub enum State {
	New,
	NeedsResult,
	NeedsCallback,
	Armed
}

impl From<usize> for State {
    fn from(v: usize) -> State {
    	match v {
    		0 => New,
    		1 => NeedsResult,
    		2 => NeedsCallback,
    		3 => Armed,
    		_ => panic!("unexpected state value")
    	}
    }
}

impl From<State> for usize {
    fn from(v: State) -> usize {
    	match v {
    		New => 0,
    		NeedsResult => 1,
    		NeedsCallback => 2,
    		Armed => 3
    	}
    }
}

pub struct Core<T: Send+'static, E:Send+'static> {
	inner: *mut Inner<T,E>
}

impl<T: Send+'static, E:Send+'static> Core<T,E> {
	pub fn new() -> Core<T,E> {
		Core {
			inner: Box::into_raw(Box::new(Inner::new()))
		}
	}

    pub fn from_result(v: Result<T,E>) -> Core<T,E> {
    	Core {
    		inner: Box::into_raw(Box::new(Inner::from_result(v)))
    	}
    }

    pub fn state(&self) -> State {
    	self.inner().state()
    }

    pub fn set_callback<F>(&mut self, f: F)
    where F: FnOnce(Result<T,E>)+'static {
    	self.inner_mut().set_callback(f)
    }

    pub fn set_result(&mut self, r: Result<T,E>) {
    	self.inner_mut().set_result(r)
    }

    #[inline]
    fn inner(&self) -> &Inner<T,E> {
    	unsafe { &*self.inner }
    }

    #[inline]
    fn inner_mut(&mut self) -> &mut Inner<T,E> {
    	unsafe { &mut *self.inner }
    }


    
}


pub struct Inner<T: Send+'static, E:Send+'static> {
	result: Option<Result<T,E>>,
	state: AtomicUsize,
	cb: Option<Box<FnBox(Result<T,E>)>>,
	x: Option<Arc<Executor>>
}

impl<T: Send+'static, E:Send+'static> Inner<T,E> {
	pub fn new() -> Inner<T,E> {
		Inner {
			result: None,
			state: AtomicUsize::new(New.into()),
			cb: None,
			x: None
		}
	}


    pub fn from_result(v: Result<T,E>) -> Inner<T,E> {
    	Inner {
    		result: Some(v),
    		state: AtomicUsize::new(Armed.into()),
    		cb: None,
    		x: None
    	}
    }

    pub fn state(&self) -> State {
    	self.state.load(Ordering::Relaxed).into()
    }

    pub fn set_callback<F>(&mut self, f: F)
    where F: FnOnce(Result<T,E>)+'static {
    	match self.state() {
    		New => { 
    			let n = self.state.compare_and_swap(New.into(), NeedsResult.into(), Ordering::Relaxed).into();
    			match n {
    				New => {
    					self.cb = Some(Box::new(f));
    				},
    				NeedsCallback => {
    					self.state.store(Armed.into(), Ordering::Relaxed);
    					self.cb = Some(Box::new(f));
    					self.schedule_callback();
    				},
    				_ => panic!("invalid state")	

    			}
    		},
    		NeedsCallback => {
    			self.state.store(Armed.into(), Ordering::Relaxed);
    			self.cb = Some(Box::new(f));
    			self.schedule_callback();
    		},
    		_ => panic!("invalid state")	

    	}
    }

    pub fn set_result(&mut self, r: Result<T,E>) {
    	self.result = Some(r); 
    	match self.state() {
    		New => {
    			let n = self.state.compare_and_swap(New.into(), NeedsCallback.into(), Ordering::Relaxed).into();
    			match n {
    				New => {},
    				NeedsResult => {
    					self.state.store(Armed.into(), Ordering::Relaxed);
    					self.schedule_callback();
    				},
    				_ => panic!("invalid state")	
    			}
    		},
    		NeedsResult => {
    			self.state.store(Armed.into(), Ordering::Relaxed);
    			self.schedule_callback();
    		},
    		_ => panic!("invalid state")
    	}	
    }

    fn schedule_callback(&mut self) {
    	let cb = self.cb.take().expect("needs cb");
    	let result = self.result.take().expect("needs result");
    	match self.x {
    		Some(ref x) => {
    			x.add(Box::new(move ||{
    				cb(result)
    			}));
    		},
    		None => {
    			InlineExecutor::new().add(Box::new(move ||{
	    			cb(result)
	    		}));
    		}
    	}
    }


}

#[cfg(test)]
mod tests {
	use super::*;
    use testutils::marker;

	#[test]
	fn from_result() {
		let c = Core::<_,()>::from_result(Ok(3));
		assert_eq!(c.inner().result, Some(Ok(3)));
		assert_eq!(c.state(), State::Armed);

		let c = Core::<usize,_>::from_result(Err(()));
		assert_eq!(c.inner().result, Some(Err(())));
		assert_eq!(c.inner().state(), State::Armed);

	}

	#[test]
	fn state() {
		let c = Core::<(),()>::new();
		assert_eq!(c.inner().state(), State::New);
	}

	#[test]
	fn lifecycle() {
		let (set_marker, assert_marker) = marker();

		let mut c = Core::<usize,()>::new();
		assert_eq!(c.state(), State::New);

		c.set_callback(move |r| {
			assert_eq!(r, Ok(333));
			set_marker()
		});
		assert_eq!(c.state(), State::NeedsResult);

		c.set_result(Ok(333));
		assert_eq!(c.state(), State::Armed);

		assert_marker();

	}
}