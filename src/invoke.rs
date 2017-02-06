
#[doc(hidden)]
pub trait Invoke<T> {
    fn invoke(self: Box<Self>) -> T;
}

impl<T, F> Invoke<T> for F
    where F: FnOnce() -> T
{
    fn invoke(self: Box<F>) -> T {
        let f = *self;
        f()
    }
}
