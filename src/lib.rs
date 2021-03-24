pub struct Signal<T: ?Sized> {
    receivers: Vec<Box<dyn Fn(&T) -> ()>>
}

impl<T> Signal<T> where T: ?Sized {
    pub fn new() -> Self {
        Self { receivers: vec![] }
    }

    pub fn connect(mut self, receiver: impl Fn(&T) -> () + 'static) -> Self {
        self.receivers.push(Box::new(receiver));
        self
    }

    pub fn send(&self, sender: &T) {
        for receiver in self.receivers.iter() {
            (receiver)(sender)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_send_str() {
        use mockall::*;

        #[automock]
        pub trait Foo {
            fn foo(s: &str);
        }

        let ctx = MockFoo::foo_context();
        ctx.expect().once().returning(|s| println!("Sender: {}", s));

        let signal = Signal::new()
            .connect(MockFoo::foo);
        signal.send("hello world");
    }
}
