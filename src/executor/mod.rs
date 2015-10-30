mod inline;
mod default;

pub use self::inline::InlineExecutor;
pub use self::default::DefaultExecutor;

use std::boxed::FnBox;

pub trait Executor {
    fn add(&self, f: Box<FnBox()>);
}
