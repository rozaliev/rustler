use std::boxed::FnBox;


pub fn marker() -> (Box<Fn() + Send + 'static>, Box<Fn() + Send + 'static>) {
    use std::sync::{Arc, Mutex};

    let m = Arc::new(Mutex::new(false));
    let ma = m.clone();

    let set_marker = move || {
        let mut l = m.lock().unwrap();
        *l = true;
    };

    let assert_marker = move || {
        let l = ma.lock().unwrap();
        assert!(*l, "expected marker");
    };

    (Box::new(set_marker), Box::new(assert_marker))
}
