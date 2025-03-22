// util/config.rs

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("environment variable not found: {0}")]
    EnvVarNotFound(String),
    #[error("config file not found: {0}")]
    ConfigFileNotFound(String),
    #[error("error reading config file: {0}")]
    ReadConfigFileError(#[from] std::io::Error),
    #[error("error parsing config file: {0}")]
    ParseConfigError(#[from] serde_yaml::Error),
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub max_connections: u32,
    pub tls: TlsConfig,
}

#[derive(Debug, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub encryption_key: String,
    pub token_expiration: u64,
}

#[derive(Debug, Deserialize)]
pub struct MessageConfig {
    pub lifespan: i64,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub security: SecurityConfig,
    pub message: MessageConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        // 从环境变量中获取配置文件路径
        let config_path = env::var("APP_CONFIG_PATH")
            .map_err(|_| {
                error!("环境变量 APP_CONFIG_PATH 未找到");
                ConfigError::EnvVarNotFound("APP_CONFIG_PATH".to_string())
            })?;

        info!("配置文件路径: {}", config_path);

        // 检查配置文件是否存在
        if !Path::new(&config_path).exists() {
            error!("配置文件不存在: {}", config_path);
            return Err(ConfigError::ConfigFileNotFound(config_path));
        }

        // 读取配置文件内容
        info!("正在读取配置文件");
        let config_content = fs::read_to_string(&config_path)?;

        // 解析 YAML 配置文件
        info!("正在解析配置文件");
        let config: AppConfig = serde_yaml::from_str(&config_content)?;

        info!("配置文件加载成功");
        Ok(config)
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load() {
        // 设置测试环境变量
        unsafe {
            env::set_var("APP_CONFIG_PATH", "test_config.yaml");
        }

        // 创建测试配置文件
        let test_config = r#"
server:
  bind_address: "127.0.0.1"
  port: 8080
  max_connections: 100
  tls:
    enabled: true
    cert_path: "certs/cert.pem"
    key_path: "certs/key.pem"

database:
  url: "DATABASE_URL"
  max_connections: 30

security:
  encryption_key: "supersecretkey123"
  token_expiration: 3600

message:
  lifespan: 24
"#;

        // 写入测试配置文件
        fs::write("test_config.yaml", test_config).unwrap();

        info!("开始加载配置");

        // 加载配置
        let config = AppConfig::new();

        // 删除测试配置文件
        fs::remove_file("test_config.yaml").unwrap();

        // 检查配置是否正确加载
        assert!(config.is_ok());

        let config = config.unwrap();

        assert_eq!(config.server.bind_address, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.max_connections, 100);
        assert!(config.server.tls.enabled);
        assert_eq!(config.server.tls.cert_path, "certs/cert.pem");
        assert_eq!(config.server.tls.key_path, "certs/key.pem");

        assert_eq!(config.database.url, "DATABASE_URL");
        assert_eq!(config.database.max_connections, 30);

        assert_eq!(config.security.encryption_key, "supersecretkey123");
        assert_eq!(config.security.token_expiration, 3600);

        assert_eq!(config.message.lifespan, 24);

        info!("配置加载测试完成");
    }
}