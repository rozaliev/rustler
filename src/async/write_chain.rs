use std::collections::VecDeque;
use std::io;

use iobuf::{Iobuf, ROIobuf};
use mio::TryWrite;


pub struct WriteChain {
	chain: VecDeque<ROIobuf<'static>>
}

impl WriteChain {
    fn new() -> WriteChain {
    	WriteChain {
    		chain: VecDeque::new()
    	}
    }


    fn push(&mut self, buf: ROIobuf<'static>) {
    	self.chain.push_back(buf);
    }

    fn write_to<T: TryWrite>(&mut self, mut out: T) -> Option<io::Result<Option<usize>>> {
 		if self.chain.len() == 0 {
 			return None
 		}
    	let r = out.try_write(unsafe { self.chain[0].as_window_slice() });
    	match r {
    		Ok(None) => {
    			Some(Ok(None))
    		},
    		Ok(Some(n)) => {
    			self.chain[0].advance(n as u32);
    			if self.chain[0].len() == 0 {
    				self.chain.pop_front();
    			};

    			Some(Ok(Some(n)))
    		},
    		Err(e) => {
    			Some(Err(e))
    		}
    	}
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use iobuf::{Iobuf, ROIobuf};

	#[test]
	fn write_to() {
		let mut wc = WriteChain::new();
		let b = ROIobuf::from_str("hello");
		let b2 = ROIobuf::from_str("qweqweqwe");
		wc.push(b);

		let mut out = vec![0; 2];
		wc.write_to(&mut out[0..2]);
		assert_eq!(out, "he".as_bytes());

		let mut out = vec![0; 2];
		wc.write_to(&mut out[0..2]);
		assert_eq!(out, "ll".as_bytes());

		wc.push(b2);
		assert_eq!(wc.chain.len(), 2);

		let mut out = vec![0; 2];
		wc.write_to(&mut out[0..2]);
		assert_eq!(out, "o\0".as_bytes());
		assert_eq!(wc.chain.len(), 1);

		let mut out = vec![0; 9];
		wc.write_to(&mut out[0..9]);
		assert_eq!(out, "qweqweqwe".as_bytes());

		assert_eq!(wc.chain.len(), 0);

	}
}
