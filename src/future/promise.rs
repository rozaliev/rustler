use super::core::Core;
use super::future::Future;


pub struct Promise<T: Send + 'static, E: Send + 'static> {
    core: Core<T, E>,
}

impl<T: Send+'static, E: Send+'static> Promise<T,E> {
    pub fn pair() -> (Future<T, E>, Promise<T, E>) {
        let core = Core::new();
        let f = Future::from_core(core.clone());
        let p = Promise { core: core };
        (f, p)
    }

    pub fn resolve(&mut self, r: Result<T, E>) {
        self.core.set_result(r);
    }
}


unsafe impl<T: Send+'static, E: Send+'static> Send for Promise<T,E> {}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair() {
        let (f, p) = Promise::<(), ()>::pair();
    }
}
