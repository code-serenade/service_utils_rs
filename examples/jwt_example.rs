use service_utils_rs::error::Result;
use service_utils_rs::services::jwt::Jwt;
use service_utils_rs::settings::Settings;

fn main() -> Result<()> {
    let settings = Settings::new("examples/config/services.toml").unwrap();
    println!("{:?}", settings);

    let jwt = Jwt::new(settings.jwt);
    let (token, r) = jwt.generate_token_pair("sub".to_string())?;
    println!("access token: {:?}", token);
    println!("reflesh token: {:?}", r);

    let claims = jwt.validate_access_token(&token)?;

    let reflesh = jwt.validate_refresh_token(&r)?;
    println!("access claims: {:?}", claims);
    println!("reflesh claims: {:?}", reflesh);
    Ok(())
}

// cargo run --example jwt_example --features jwt
