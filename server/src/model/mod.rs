// model/mod.rs

pub mod user;
pub mod message;

use diesel::prelude::*;
// use diesel::sqlite::SqliteConnection;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;

use crate::util::APP_CONFIG;

// // 建立数据库连接
// pub fn get_conn() -> SqliteConnection {
//     dotenv().ok();
//     let binding = dotenv::var(APP_CONFIG.database.url.as_str()).unwrap();
//     let url = binding.as_str();
//     SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to SQLite database: {}", url))
// }

// 建立数据库连接
pub fn get_conn() -> MysqlConnection {
    dotenv().ok();
    let binding = dotenv::var(APP_CONFIG.database.url.as_str()).unwrap();
    let url = binding.as_str();
    MysqlConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to MySQL database: {}", url))
}