pub struct Signal<T: ?Sized> {
    receivers: Vec<Box<dyn Receiver<T>>>,
}

impl<T> Signal<T>
where
    T: ?Sized,
{
    pub fn new() -> Self {
        Self { receivers: vec![] }
    }

    pub fn connect<R>(mut self, receiver: R) -> Self
    where
        R: Receiver<T> + 'static,
    {
        self.receivers.push(Box::new(receiver));
        self
    }

    pub fn disconnect<R>(&self, _receiver: R)
    where
        R: Receiver<T> + 'static,
    {
        // Todo
    }

    pub fn send(&self, sender: &T) {
        for receiver in self.receivers.iter() {
            receiver.handle_signal(sender)
        }
    }
}

pub trait Receiver<T: ?Sized> {
    fn handle_signal(&self, sender: &T);
}

impl<T, F> Receiver<T> for F
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

        let signal = Signal::new().connect(MockFoo::foo);
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

        let signal = Signal::new().connect(MockOne::one);
        signal.send("first");

        signal.disconnect(MockOne::one);

        signal.send("second");
    }
}
