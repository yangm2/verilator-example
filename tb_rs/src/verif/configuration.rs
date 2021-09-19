//! configuration injection

pub trait Configurable<T> {
    fn config(self, c: Configuration<T>) -> T;
}

pub struct Configuration<T> {
    f: Box<dyn Fn() -> T>,
}

pub fn new<T>(f: Box<dyn Fn() -> T>) -> Configuration<T> {
    Configuration { f: f }
}
