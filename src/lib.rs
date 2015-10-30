#![feature(plugin)]
#![plugin(clippy)]

#![feature(fnbox)]
#![feature(const_fn)]

mod future;
mod executor;



#[cfg(test)]
mod testutils;
