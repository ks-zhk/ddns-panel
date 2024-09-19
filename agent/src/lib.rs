#![feature(ip)]

use serde::Serialize;
use anyhow::anyhow;
pub mod client;
pub mod server;

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct ChooseConfig {
    interface_name: Option<String>,
    /// ip类型
    ip_type: Option<String>,
    /// 匹配ip的正则表达式
    regex: Option<String>,
    /// 是否启动自动获取功能
    auto_recommand: bool,
    /// 获取最大ip数量
    ip_max_num: u64,
    /// 是否仅获取公网IP
    global_only: bool
}

impl ChooseConfig {
    pub fn is_valid(&self) -> anyhow::Result<()> {
        if let Some(ip_type) = &self.ip_type {
            match ip_type.as_str() {
                "v4" | "v6" => Ok(()),
                _ => Err(anyhow!("ip_type must be v4 or v6"))
            }
        } else {
            Ok(())
        }
    }
}