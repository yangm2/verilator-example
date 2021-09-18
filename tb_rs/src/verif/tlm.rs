//! Transaction-Level-Modeling

/// Tlm interface
pub trait Tlm<T> {
    fn get_next_item(&self) -> T;

    fn set_callback(&mut self, c: impl Fn() -> T + 'static);

    fn call(&mut self) -> T;
}

pub struct TlmPort<T> {
    it: T,
    callback: Option<Box<dyn Fn() -> T>>,
}

impl<T> Tlm<T> for TlmPort<T> {
    fn get_next_item(&self) -> T {
        todo!()
    }

    fn set_callback(&mut self, c: impl Fn() -> T + 'static) {
        self.callback = Some(Box::new(c));
    }

    // TODO: not sure if this is safe ... Fn vs FnMut???
    fn call(&mut self) -> T{
        if let Some(c) = &self.callback {
            c()
        } else {
            panic!("callback was not connected")
        }
    }
}

pub fn new<T>(it: T) -> TlmPort<T> {
    TlmPort::<T> {
        it: it,
        callback: None,
    }
}
