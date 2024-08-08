Sure, here's the `README.md` file in raw Markdown format that you can copy directly:

```markdown
# service_utils_rs

`service_utils_rs` is a Rust library that provides utility functions to simplify and speed up service development.

## Features

- Efficient JWT processing
- Integrated logging
- Configuration management tools
- Additional utility functions

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
service_utils_rs = "0.1.1"
config = "0.13.0"
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### Configuration Management

#### 1. Create a Configuration File

Create a `config` directory at the root of your project and add a `config.toml` file with the following content:

```toml
# config/config.toml
[jwt]
access_secret = "3a5df12e1fc87ad045e1767e3f6a285da64139de0199f3d7b1d869f03d8eae30e130bacc2018d8c2e1dced55eac6fbb45f0cf283a5f64dc75a886ac8fd3937e5"
refresh_secret = "b26f570b5d72795815f898cea04a4234a932cded824081767698e58e13ff849f3b6e23fe34efb4f6d78e342f1be4eace18135994e51a070c605c6dc9698a5fab"
audience = "test"
access_token_duration = 86400
refresh_token_duration = 172800    
```

#### 2. Settings Module

Ensure your `settings.rs` file reads the configuration file and deserializes the settings:

```rust
use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub jwt: JwtCfg,
}

#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_secret: String,
    pub refresh_secret: String,
    pub audience: String,
    pub access_token_duration: usize,
    pub refresh_token_duration: usize,
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

#### 3. Using the Library in Your Project

In your project, use the `service_utils_rs` library and pass the configuration file path:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config/config.toml";
    let settings = Settings::new(config_path)?;
    println!("{:?}", settings);
    Ok(())
}
```

By following these steps, you can configure and use the `service_utils_rs` library in your project as intended.
