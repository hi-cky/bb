pub mod config;

use config::AppConfig;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_CONFIG: AppConfig = AppConfig::new().unwrap();
}