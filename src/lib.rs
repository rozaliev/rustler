#![feature(plugin)]
#![plugin(clippy)]

#![feature(fnbox)]
#![feature(const_fn)]
#![feature(unboxed_closures)]
#![feature(core)]

pub mod future;
pub mod executor;
pub mod service;
pub mod pipeline;



#[cfg(test)]
mod testutils;
