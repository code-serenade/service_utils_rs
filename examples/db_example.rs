use service_utils_rs::{services::db, settings::Settings};

#[tokio::main]
async fn main() {
    let settings = Settings::new("examples/config/services.toml").unwrap();
    // println!("{:?}", settings);
    db::init_db(settings.surrealdb).await.unwrap();
    let db = db::get_db();
    let a = db.query("SELECT * FROM user").await.unwrap();
    println!("{:?}", a);
}

// cargo run --example db_example --features db
