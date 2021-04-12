use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Signal<T: ?Sized> {
    receivers: RefCell<Vec<Weak<Box<dyn Receiver<T>>>>>,
}

impl<T> Signal<T>
where
    T: ?Sized,
{
    pub fn new() -> Self {
        Self {
            receivers: RefCell::new(vec![]),
        }
    }

    pub fn connect<R>(&self, receiver: R) -> Rc<Box<dyn Receiver<T>>>
    where
        R: Receiver<T> + 'static,
    {
        let receiver: Box<dyn Receiver<T>> = Box::new(receiver);
        let subscription = Rc::new(receiver);
        self.receivers
            .borrow_mut()
            .push(Rc::downgrade(&subscription));
        subscription
    }

    pub fn disconnect(&self, _dispatch_id: &str) {}

    pub fn send(&self, sender: &T) {
        let mut remove_indices = Vec::new();
        for (index, receiver) in self.receivers.borrow_mut().iter().enumerate() {
            let receiver = receiver.upgrade();
            match receiver {
                Some(receiver) => receiver.handle_signal(sender),
                None => remove_indices.push(index),
            }
        }

        for &i in remove_indices.iter() {
            self.receivers.borrow_mut().remove(i);
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

        let signal = Signal::new();
        let _subscription = signal.connect(MockFoo::foo);
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
        let subscription = signal.connect(MockOne::one);
        signal.send("first");
        drop(subscription);

        signal.send("second");
    }
}
