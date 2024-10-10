#![feature(ip)]

use anyhow::anyhow;
use pnet::datalink::NetworkInterface;
use serde::Serialize;
pub mod client;
pub mod server;

#[derive(serde::Deserialize, Serialize, Debug, Clone)]
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
    global_only: bool,
}

impl ChooseConfig {
    pub fn is_valid(&self) -> anyhow::Result<()> {
        if let Some(ip_type) = &self.ip_type {
            match ip_type.as_str() {
                "v4" | "v6" => Ok(()),
                _ => Err(anyhow!("ip_type must be v4 or v6")),
            }
        } else {
            Ok(())
        }
    }
    pub fn filter_interface_name<'a>(&self, interfaces: &'a mut Vec<NetworkInterface>) {
        if let Some(interface_name) = &self.interface_name {
            interfaces.retain(|interface| interface.name.eq(interface_name))
        }
    }
    pub fn filter_ip_type<'a>(&self, interfaces: &'a mut Vec<NetworkInterface>) {
        if let Some(ip_type) = &self.ip_type {
            interfaces.retain_mut(|interface| {
                interface.ips.retain(|ip| {
                    (ip_type.eq("v4") && ip.is_ipv4()) || (ip_type.eq("v6") && ip.is_ipv6())
                });
                interface.ips.len() > 0
            });
        }
    }

    pub fn filter_regex<'a>(
        &self,
        interfaces: &'a mut Vec<NetworkInterface>,
    ) -> anyhow::Result<()> {
        if let Some(regex_str) = &self.regex {
            let re = regex::Regex::new(&regex_str)?;
            interfaces.retain_mut(|interfaces| {
                interfaces.ips.retain(|ip| {
                    let ip_str = ip.to_string();
                    let caps = re.captures(&ip_str);
                    if let Some(_) = caps {
                        return true;
                    }
                    false
                });
                interfaces.ips.len() > 0
            });
        }
        Ok(())
    }
    pub fn filter_global<'a>(&self, interfaces: &'a mut Vec<NetworkInterface>) {
        if self.global_only == true {
            interfaces.retain_mut(|interface| {
                interface.ips.retain(|ip| ip.ip().is_global());
                interface.ips.len() > 0
            });
        }
    }
    pub fn filter_recommend<'a>(&self, interfaces: &'a mut Vec<NetworkInterface>) {
        if self.auto_recommand == false {
            return;
        }
        let mut recommend_config = self.clone();
        recommend_config.auto_recommand = false;
        recommend_config.global_only = true;
        recommend_config.regex = None;
        recommend_config.ip_max_num = 1;
        let mut v4_global = false;
        let mut v6_global = false;
        for interface in interfaces {
            for ip in &interface.ips {
                if ip.is_ipv4() && ip.ip().is_global() {
                    v4_global = true
                }
                if ip.is_ipv6() && ip.ip().is_global() {
                    v6_global = true
                }
            }
        }

    }
}
