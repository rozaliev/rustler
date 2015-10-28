use std::boxed::FnBox;

use super::Executor;


pub struct InlineExecutor;

impl InlineExecutor {
    pub fn new() -> InlineExecutor {
    	InlineExecutor
    }
}

impl Executor for InlineExecutor {
    fn add(&self, f: Box<FnBox()>) {
    	f();
    }
}