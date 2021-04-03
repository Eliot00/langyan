use std::cell::RefCell;

pub struct Signal<T: ?Sized> {
    receivers: RefCell<Vec<Receiver<T>>>,
}

impl<T> Signal<T>
where
    T: ?Sized
{
    pub fn new() -> Self {
        Self { receivers: RefCell::new(vec![]) }
    }

    pub fn connect<H>(&self, handler: H, dispatch_id: &str) 
    where H: Handler<T> + 'static
    {
        let receiver = Receiver::new(handler, dispatch_id.to_string());
        self.receivers.borrow_mut().push(receiver);
    }

    pub fn disconnect(&self, dispatch_id: &str) {
        self.receivers.borrow_mut().retain(|r| r.dispatch_id != dispatch_id);
    }

    pub fn send(&self, sender: &T) {
        for receiver in self.receivers.borrow().iter() {
            receiver.handle_signal(sender)
        }
    }
}

pub struct Receiver<T: ?Sized> {
    dispatch_id: String,
    handler: Box<dyn Handler<T>>,
}

impl<T> Receiver<T> where T: ?Sized {
    pub fn new<R>(handler: R, dispatch_id: String) -> Self
    where R: Handler<T> + 'static 
    {
        Self {handler: Box::new(handler), dispatch_id}
    }

    pub fn handle_signal(&self, sender: &T) {
        self.handler.handle_signal(sender)
    }

    pub fn equal(&self, other: &Self) -> bool {
        self.dispatch_id == other.dispatch_id
    }
}

pub trait Handler<T: ?Sized> {
    fn handle_signal(&self, sender: &T);
}

impl<T, F> Handler<T> for F
where
    F: Fn(&T) -> () + 'static,
    T: ?Sized,
{
    fn handle_signal(&self, sender: &T) {
        self(sender);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn should_send_str() {
        #[automock]
        trait Foo {
            fn foo(s: &str);
        }

        let ctx = MockFoo::foo_context();
        ctx.expect().once().returning(|s| println!("Sender: {}", s));

        let signal = Signal::new();
        signal.connect(MockFoo::foo, "mock_foo");
        signal.send("hello world");
    }

    #[test]
    fn test_disconnect() {
        #[automock]
        trait One {
            fn one(s: &str);
        }

        let ctx = MockOne::one_context();
        ctx.expect()
            .once()
            .returning(|s| println!("Only invoke once: {}", s));

        let signal = Signal::new();
        signal.connect(MockOne::one, "mock_one");
        
        signal.send("first");

        signal.disconnect("mock_one");

        signal.send("second");
    }
}