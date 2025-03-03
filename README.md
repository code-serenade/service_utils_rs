Here's an updated version of your README file with examples for multiple services, such as HTTP, JWT, WebSocket, and Database. It provides instructions on how to use each of them in your project.

````markdown
# service_utils_rs

`service_utils_rs` is a Rust library that provides utility functions to simplify and speed up service development. This library includes features for HTTP server handling, JWT processing, WebSocket communication, database management, and more.

## Features

- **JWT Processing**: Efficient JWT handling for authentication.
- **Logging**: Integrated logging for easier debugging and monitoring.
- **Configuration Management**: Easy configuration handling through TOML files.
- **WebSocket Support**: Seamless WebSocket server integration.
- **Database Integration**: Simplified database management (e.g., SurrealDB).
- **Utility Functions**: Various helper functions for common tasks.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
service_utils_rs = "0.3.5"
```
````

## Usage

Below is an overview of how to use different features of `service_utils_rs` in your project.

### 1. Configuration Management

#### a. Create a Configuration File

Create a `config` directory at the root of your project and add a `config.toml` file with the following content:

```toml
# config/config.toml
[jwt]
access_secret = "3a5df12e1fc87ad045e1767e3f6a285da64139de0199f3d7b1d869f03d8eae30e130bacc2018d8c2e1dced55eac6fbb45f0cf283a5f64dc75a886ac8fd3937e5"
refresh_secret = "b26f570b5d72795815f898cea04a4234a932cded824081767698e58e13ff849f3b6e23fe34efb4f6d78e342f1be4eace18135994e51a070c605c6dc9698a5fab"
audience = "test"
access_token_duration = 86400
refresh_token_duration = 172800

[db]
host = "localhost"
port = 8000
username = "root"
password = "root"
namespace = "ns"
database = "db"
```

#### b. Settings Module

use the `settings.rs` file to load and parse the configuration file:

```rust
use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub jwt: JwtCfg,
    pub db: DbCfg,
}

#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_secret: String,
    pub refresh_secret: String,
    pub audience: String,
    pub access_token_duration: usize,
    pub refresh_token_duration: usize,
}

#[derive(Debug, Deserialize)]
pub struct DbCfg {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        c.try_deserialize()
    }
}
```

#### c. Using the Configuration in Your Project

In your `main.rs` or application entry point, use the settings to initialize the services:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config/config.toml";
    let settings = Settings::new(config_path)?;

    // Now you can access JWT and DB configurations
    println!("{:?}", settings.jwt);
    println!("{:?}", settings.db);

    Ok(())
}
```

### 2. JWT Authentication

Use the `service_utils_rs` JWT module to handle JWT-based authentication.

#### a. Example: JWT Authentication

```rust
use service_utils_rs::jwt::{create_jwt, verify_jwt};

fn generate_token() -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        sub: "user123".to_string(),
        exp: 1627658476, // Expiration time
    };

    let secret = "your_jwt_secret_key";
    let token = create_jwt(&claims, secret)?;
    Ok(token)
}

fn validate_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let secret = "your_jwt_secret_key";
    let claims = verify_jwt(token, secret)?;
    Ok(claims)
}
```

### 3. WebSocket Server

Use the WebSocket functionality to create a simple WebSocket server.

#### a. Example: WebSocket Server

```rust
use service_utils_rs::websocket::{WebSocketServer, WebSocketHandler};
use tokio::sync::mpsc;

struct MyHandler;

#[async_trait::async_trait]
impl WebSocketHandler for MyHandler {
    async fn on_message(&self, msg: String) -> String {
        // Handle incoming message and send a response
        format!("Received: {}", msg)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(32);
    let server = WebSocketServer::new(MyHandler).bind("127.0.0.1:8080").await?;
    server.run(tx).await?;
    Ok(())
}
```

### 4. Database Management (SurrealDB)

SurrealDB can be used for database management. Below is how to initialize and query a database using `service_utils_rs` integration.

#### a. Example: Database Initialization

```rust
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Surreal, Response};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init_db(cfg: DbCfg) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", cfg.host, cfg.port);
    DB.connect::<Ws>(addr).await?;
    DB.signin(Root {
        username: &cfg.username,
        password: &cfg.password,
    }).await?;
    DB.use_ns(cfg.namespace).use_db(cfg.database).await?;
    Ok(())
}

pub async fn query_data() -> Result<Response, Box<dyn std::error::Error>> {
    let result = DB.query("SELECT * FROM person").await?;
    Ok(result)
}
```

### 5. Example: Putting It All Together

Here’s an example where you initialize the database and JWT system, then use the WebSocket server to interact with clients.

```rust
use service_utils_rs::settings::Settings;
use db::init_db;
use service_utils_rs::jwt::{create_jwt, verify_jwt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let settings = Settings::new("config/config.toml")?;

    // Initialize DB
    let db_cfg = settings.db.clone();
    init_db(db_cfg)?;

    // Generate a JWT token
    let token = create_jwt(&Claims { sub: "user123".to_string(), exp: 1627658476 }, "your_jwt_secret_key")?;
    println!("Generated JWT: {}", token);

    // Start WebSocket server
    let server = WebSocketServer::new(MyHandler).bind("127.0.0.1:8080")?;
    server.run().await?;

    Ok(())
}
```

## Conclusion

`service_utils_rs` simplifies various aspects of service development. Whether you're building an HTTP server, handling JWT authentication, managing databases, or setting up WebSocket communication, this library has you covered.

### License

This project is licensed under the MIT License.

```

This README now includes sections explaining how to use different parts of the library such as configuration management, JWT authentication, WebSocket server, and database integration. Each section provides clear examples of how to integrate these features into your project.
```
