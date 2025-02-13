use std::{future::Future, pin::Pin};

pub struct Response<T> {
    pub body: T,
}

pub trait IntoResponse {
    fn into_response(self) -> Response<Self>
    where
        Self: Sized;
}

impl<T> IntoResponse for T {
    fn into_response(self) -> Response<Self> {
        Response { body: self }
    }
}

pub trait Handler<T, Res>: Clone + Send + Sized + 'static {
    // type Future: Future<Output = Response<Res>> + Send + 'static;
    fn call(self, req: T) -> Pin<Box<dyn Future<Output = Response<Res>> + Send>>;
}

impl<F, Fut, T, Res> Handler<T, Res> for F
where
    F: FnOnce(T) -> Fut + Clone + Send + 'static, // 闭包本身需要满足 `Send`
    Fut: Future<Output = Res> + Send,             // 返回的 `Future` 需要满足 `Send`
    Res: IntoResponse + 'static,                  // 响应需要支持 `IntoResponse`
    T: Send + 'static,
{
    // type Future = Pin<Box<dyn Future<Output = Response<Res>> + Send>>;

    fn call(self, req: T) -> Pin<Box<dyn Future<Output = Response<Res>> + Send>> {
        // 将 `async move` 返回的 Future 包装成 `Pin<Box>` 并确保是 `Send`
        Box::pin(async move { self(req).await.into_response() })
    }
}
