use tower_http::cors::{Any, CorsLayer};

pub fn create_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(Any) // 允许任意 HTTP 方法
        .allow_origin(Any) // 允许任意来源
        .allow_headers(Any) // 允许任意请求头，包括 Content-Type
}
