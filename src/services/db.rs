use std::sync::LazyLock;

use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::error::Result;

/// Struct representing the Surrealdb configuration parameters.
#[derive(Debug, Deserialize)]
pub struct SurrealdbCfg {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init_db(cfg: SurrealdbCfg) -> Result<()> {
    let addr = format!("{}:{}", cfg.host, cfg.port);
    DB.connect::<Ws>(addr).await?;
    DB.signin(Root {
        username: &cfg.username,
        password: &cfg.password,
    })
    .await?;
    DB.use_ns(cfg.namespace).use_db(cfg.database).await?;
    Ok(())
}

pub fn get_db() -> &'static Surreal<Client> {
    &DB
}
