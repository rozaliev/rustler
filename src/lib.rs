#![feature(plugin)]
#![plugin(clippy)]

#![feature(fnbox)]
#![feature(const_fn)]

mod future;
mod executor;
mod service;



#[cfg(test)]
mod testutils;
