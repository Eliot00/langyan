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
