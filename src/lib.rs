#![feature(plugin)]
#![plugin(clippy)]

#![feature(fnbox)]
#![feature(const_fn)]

pub mod future;
pub mod executor;
pub mod service;



#[cfg(test)]
mod testutils;
