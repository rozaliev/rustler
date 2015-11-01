extern crate rustler;

use rustler::service::{Service,SimpleFilter};
use rustler::future::{Future, Async};

fn doubler(x: u64) -> Future<u64, ()> {
	Future::value(x * 2)
}

struct RequestLoggerFilter;

impl SimpleFilter<u64, u64, ()> for RequestLoggerFilter {
    fn filter<S: Service<u64, u64, ()>>(&self, r: u64, s: S) -> Future<u64, ()> {
    	println!("got a request {:?}", r);
    	s.apply(r)
    }
}

fn main() {
}