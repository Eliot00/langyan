use std::cell::RefCell;

pub struct Signal<'a, T: ?Sized + 'a> {
    receivers: RefCell<Vec<&'a Receiver<T>>>,
}

impl<'a, T> Signal<'a, T>
where
    T: ?Sized + 'a,
{
    pub fn new() -> Self {
        Self { receivers: RefCell::new(vec![]) }
    }

    pub fn connect(&self, receiver: &'a Receiver<T>) {
        self.receivers.borrow_mut().push(receiver);
    }

    pub fn disconnect(&self, receiver: &Receiver<T>) {
        let idx = self.receivers.borrow().iter().position(
            |x| x.equal(receiver)
        );
        if idx.is_none() {
            return ();
        }
        self.receivers.borrow_mut().remove(idx.unwrap());
    }

    pub fn send(&self, sender: &T) {
        for receiver in self.receivers.borrow().iter() {
            receiver.handle_signal(sender)
        }
    }
}

pub struct Receiver<T: ?Sized> {
    dispatch_id: String,
    handler: Box<dyn Receivable<T>>,
}

impl<T> Receiver<T> where T: ?Sized {
    pub fn new<R>(handler: R, dispatch_id: String) -> Self
    where R: Receivable<T> + 'static 
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

pub trait Receivable<T: ?Sized> {
    fn handle_signal(&self, sender: &T);
}

impl<T, F> Receivable<T> for F
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
        let receiver = Receiver::new(MockFoo::foo, "mock_foo".to_string());
        signal.connect(&receiver);
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
        let receiver = Receiver::new(MockOne::one, "mock_one".to_string());
        signal.connect(&receiver);
        
        signal.send("first");

        signal.disconnect(&receiver);

        signal.send("second");
    }
}
