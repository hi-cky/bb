// 项目设置
#![deny(warnings)] // 禁用警告

pub mod util;
pub mod model;
pub mod schema;
mod network;
mod service;

#[macro_use]
extern crate log;
extern crate simple_logger;

use util::APP_CONFIG;

fn main() {
    // 初始化简单的终端日志器，设定日志级别
    simple_logger::init().unwrap();

    info!("Server binding to: {}:{}", APP_CONFIG.server.bind_address, APP_CONFIG.server.port);
}