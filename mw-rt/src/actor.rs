use alloc::boxed::Box;

#[async_trait::async_trait]
pub trait Actor {
    fn new() -> Self;

    async fn init(&mut self);
}

pub trait IntoInner {
    type Inner;

    fn into_inner() -> Self::Inner;
}