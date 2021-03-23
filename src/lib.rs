pub struct Signal {
    receivers: Vec<Box<dyn Fn(&str) -> ()>>
}

impl Signal {
    pub fn new() -> Self {
        Self { receivers: vec![] }
    }

    pub fn connect(mut self, f: impl Fn(&str) -> () + 'static) -> Self {
        self.receivers.push(Box::new(f));
        self
    }

    pub fn send(&self, message: &str) {
        for receiver in self.receivers.iter() {
            (receiver)(message)
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
        ctx.expect().once().returning(|s| println!("Msg: {}", s));

        let signal = Signal::new()
            .connect(MockFoo::foo);
        signal.send("hello world");
    }
}
