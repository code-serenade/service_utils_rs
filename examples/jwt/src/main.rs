mod settings;
use crate::settings::Settings;
use service_utils_rs::jwt::Jwt;

fn main() {
    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let jwt = Jwt::new(settings.jwt);
    let (token, r) = jwt.generate_token_pair("sub".to_string());
    println!("access token: {:?}", token);
    println!("reflesh token: {:?}", r);

    let claims = jwt.validate_access_token(&token);
    let reflesh = jwt.validate_refresh_token(&r);
    println!("access claims: {:?}", claims);
    println!("reflesh claims: {:?}", reflesh);
}
