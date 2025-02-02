use dotenvy::dotenv;
use std::env as std_env;
use std::sync::LazyLock;

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:8080";
    pub const MAX_ROWS: u32 = 1000;
    pub const AWS_MAX_RETRIES: u32 = 10;
    pub const REGION: &str = "eu-central-1";
    pub const LOGGING_TABLE_NAME: &str = "logs";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub const MAX_ROWS: u32 = 1000;
    pub const AWS_MAX_RETRIES: u32 = 10;
    pub const REGION: &str = "eu-central-1";
    pub const LOGGING_TABLE_NAME: &str = "logs";
}

pub mod env {
    pub const LOG_GROUP_NAME_ENV_VAR: &str = "LOG_GROUP_NAME";
}

pub static LOG_GROUP_NAME_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret =
        std_env::var(env::LOG_GROUP_NAME_ENV_VAR).expect("LOG_GROUP_NAME_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("LOG_GROUP_NAME_ENV_VAR must not be empty.");
    }
    secret
});
