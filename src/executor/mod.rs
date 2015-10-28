mod inline;

pub use self::inline::InlineExecutor;

use std::boxed::FnBox;

pub trait Executor {
	fn add(&self, f: Box<FnBox()>);
}