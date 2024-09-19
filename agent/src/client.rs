use std::net::{IpAddr, SocketAddr};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use pnet::{datalink::NetworkInterface, packet::ipv4::Ipv4Flags};

use crate::ChooseConfig;

pub struct AgentClient {
    /// 主控制台监听的socket地址
    panel_socket_addr: SocketAddr,
    /// 所有网络接口信息，其中包括ip信息
    interfaces: Vec<NetworkInterface>,
    /// 上报间隔时间
    intervals: u64,
}

impl AgentClient {
    pub fn new(panel_socket_addr: SocketAddr, intervals: u64) -> Self {
        let interfaces = pnet::datalink::interfaces();
        Self {
            panel_socket_addr: panel_socket_addr,
            interfaces: interfaces,
            intervals: intervals,
        }
    }
    pub fn refresh_iterfaces(&mut self) -> &mut Self {
        let interfaces = pnet::datalink::interfaces();
        self.interfaces = interfaces;
        self
    }
    pub fn get_interfaces_by_choice(&self, choice: &ChooseConfig) -> anyhow::Result<Vec<NetworkInterface>> {
        choice.is_valid()?;
        let mut interfaces = Vec::new();
        let mut interfaces_temp = Vec::new();
        if let Some(interface_name) = &choice.interface_name {
            for interface in &self.interfaces {
                if interface_name.eq(&interface.name) {
                    interfaces.push(interface.clone());
                }
            }
        } else {
            interfaces = self.interfaces.clone();
        }
        if let Some(ip_type) = &choice.ip_type {
            for interface in &interfaces {
                for ip in &interface.ips {
                    if ip.is_ipv4() && ip_type.eq("v4") {
                        interfaces_temp.push(interface.clone());
                    }
                    if ip.is_ipv6() && ip_type.eq("v6") {
                        interfaces_temp.push(interface.clone());
                    }
                }
            }
            interfaces = interfaces_temp;
        }
        if let Some(regex) = &choice.regex {
            
        }
        // for interface in &self.interfaces {
        //     if let Some(interface_name) = &choice.interface_name {
        //         if interface.name.eq(interface_name) {
        //             interfaces.push(interface.clone());
        //         }
        //     }
        // }
        Ok(interfaces)
    }
}
// #[derive(Deserialize, Serialize, Debug)]
// pub struct ChooseConfig {
//     interface_name: Option<String>,
//     /// ip类型
//     ip_type: Option<String>,
//     /// 匹配ip的正则表达式
//     regex: Option<String>,
//     /// 是否启动自动获取功能
//     auto_recommand: bool,
//     /// 获取最大ip数量
//     ip_max_num: u64,
//     /// 是否仅获取公网IP
//     global_only: bool
// }

// impl ChooseConfig {
//     pub fn is_valid(&self) -> anyhow::Result<()> {
//         if let Some(ip_type) = &self.ip_type {
//             match ip_type.as_str() {
//                 "v4" | "v6" => Ok(()),
//                 _ => Err(anyhow!("ip_type must be v4 or v6"))
                
//             }
//         } else { 
//             Ok(())
//         }
//     }
// }


