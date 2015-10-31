use std::boxed::FnBox;
use std::sync::Arc;
use std::ptr;
use std::mem;
use std::sync::{ONCE_INIT, Once};

use super::InlineExecutor;

use super::Executor;

static mut X: *mut Arc<Executor> = ptr::null_mut();


static INIT: Once = ONCE_INIT;

pub struct DefaultExecutor;

impl DefaultExecutor {
    pub fn new() -> DefaultExecutor {
        INIT.call_once(|| {
            if unsafe { X.is_null() } {
                DefaultExecutor::set_executor(Arc::new(InlineExecutor::new()))
            }
        });
        DefaultExecutor
    }

    pub fn set_executor(x: Arc<Executor>) {
        let mut n = Box::into_raw(Box::new(x));
        unsafe { mem::swap(&mut n, &mut X) }

        if !n.is_null() {
            let b = unsafe { Box::from_raw(n) };
            drop(b)
        }
    }
}

impl Executor for DefaultExecutor {
    fn add(&self, f: Box<FnBox()>) {
        f();
    }
}
