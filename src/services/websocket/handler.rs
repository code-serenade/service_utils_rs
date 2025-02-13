use std::{future::Future, pin::Pin};

pub struct Args<T = ()> {
    pub body: T,
}

pub trait Handler: Clone + Send + Sized + 'static {
    /// The type of future calling this handler returns.
    type Future: Future<Output = String> + Send + 'static;
    /// Call the handler with the given request.
    fn call(self, args: Args) -> Self::Future;
}

impl<F, Fut> Handler for F
where
    F: FnOnce(Args) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = String> + Send,
{
    type Future = Pin<Box<dyn Future<Output = String> + Send>>;

    fn call(self, args: Args) -> Self::Future {
        // 将 `async move` 返回的 Future 包装成 `Pin<Box>` 并确保是 `Send`
        Box::pin(async move { self(args).await })
    }
}
