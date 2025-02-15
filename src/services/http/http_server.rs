use axum::Router;

use crate::error::Result;

pub async fn start(port: u16, router: Router) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("HTTP Server is running on http://{}", addr);
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
