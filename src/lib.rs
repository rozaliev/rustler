#![feature(plugin)]
#![plugin(clippy)]

#![feature(fnbox)]
#![feature(const_fn)]
#![feature(unboxed_closures)]
#![feature(core)]

extern crate mio;
extern crate iobuf;
#[macro_use]
extern crate log;

pub mod future;
pub mod executor;
pub mod service;
pub mod pipeline;
pub mod async;


#[cfg(test)]
mod testutils;
