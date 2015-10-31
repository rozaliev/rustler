use super::core::{Core, State};
use super::promise::Promise;

pub struct Future<T: Send + 'static, E: Send + 'static> {
    core: Core<T, E>,
}

impl<T: Send+'static, E: Send+'static> Future<T,E> {
    pub fn from_core(core: Core<T, E>) -> Future<T, E> {
        Future { core: core }
    }

    pub fn value(v: T) -> Future<T, E> {
        Future { core: Core::from_result(Ok(v)) }
    }

    pub fn error(e: E) -> Future<T, E> {
        Future { core: Core::from_result(Err(e)) }
    }

    pub fn is_ready(&self) -> bool {
        self.core.state() == State::Armed
    }
}

impl<T: Send+'static, E: Send+'static> Async for Future<T,E> {
    type Value = T;
    type Error = E;

    fn receive<F>(mut self, f: F)
        where F: FnOnce(Result<Self::Value, Self::Error>) + Send + 'static
    {
        self.core.set_callback(f)
    }
}

pub trait Async: Send+'static+Sized {
    type Value: Send+'static;
    type Error: Send+'static;


    fn receive<F>(self, f: F) where F: FnOnce(Result<Self::Value, Self::Error>) + Send + 'static;

    fn then<F, U: Async<Error = Self::Error>>(self, f: F) -> Future<U::Value, Self::Error>
        where F: FnOnce(Self::Value) -> U + Send + 'static,
              U::Value: Send + 'static
    {

        let (future, mut promise) = Promise::pair();
        self.receive(|res| {
            match res {
                Ok(v) => {
                    f(v).receive(move |res| {
                        promise.resolve(res);
                    })
                }
                Err(e) => {
                    promise.resolve(Err(e));
                }
            }
        });

        future
    }

    fn map<F, U>(self, f: F) -> Future<U, Self::Error>
        where F: FnOnce(Self::Value) -> U + Send + 'static,
              U: Send + 'static
    {
        self.then(move |v| Ok(f(v)))
    }

    fn map_err<F, U>(self, f: F) -> Future<Self::Value, U>
        where F: FnOnce(Self::Error) -> U + Send + 'static,
              U: Send + 'static
    {
        let (future, mut promise) = Promise::pair();

        self.receive(move |res| {
            match res {
                Ok(v) => {
                    promise.resolve(Ok(v))
                }
                Err(e) => {
                    promise.resolve(Err(f(e)));
                }
            }
        });

        future
    }



}

impl<T: Send+'static,E: Send+'static> Async for Result<T,E> {
    type Value = T;
    type Error = E;

    fn receive<F>(self, f: F)
        where F: FnOnce(Result<Self::Value, Self::Error>) + Send + 'static
    {
        f(self)
    }
}


unsafe impl<T: Send+'static, E: Send+'static> Send for Future<T,E> {}

#[cfg(test)]
mod tests {
    use super::*;
    use testutils::marker;


    #[test]
    fn then() {
        let (set_marker, assert_marker) = marker();

        let f = Future::<(), ()>::value(())
                    .then(move |_| Ok(1))
                    .then(move |r| Ok(r + 333))
                    .then(move |r| Ok(r + 2))
                    .then(move |r| {
                        assert_eq!(r, 336);
                        set_marker();
                        Ok(())
                    });

        assert_marker();
    }

    #[test]
    fn map() {
        let (set_marker, assert_marker) = marker();

        Future::<(), ()>::value(())
            .map_err(|_| "")
            .then(move |v| {
                assert_eq!(v, ());
                if false {
                    return Ok(());
                }
                Err("test error")
            })
            .map_err(move |e| (e, "add smth"))
            .receive(move |r| {
                assert_eq!(r, Err(("test error", "add smth")));
                set_marker();
            });

        assert_marker();

    }

    #[test]
    fn inner_chain() {
        let (set_marker, assert_marker) = marker();

        let f = Future::<_, ()>::value(1)
                    .then(move |v| Future::value(1).map(move |v2| v2 + v))
                    .then(move |v| {
                        assert_eq!(v, 2);
                        set_marker();
                        Ok(())
                    });

        assert_marker();
    }
}
