use crate::error::Result;
use crate::settings::SurrealdbCfg;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

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
