use std::net::IpAddr;

use pnet::{self, datalink::NetworkInterface};
#[derive(Debug)]
pub struct IpInfo {
    ip: std::net::IpAddr,
    interface_name: String,
}
impl IpInfo {
    pub fn get_ip_info_list() -> Vec<IpInfo> {
        let mut ip_infos: Vec<IpInfo> = Vec::new();
        let interfaces = pnet::datalink::interfaces();
        for interface in interfaces {
            for ip in interface.ips {
                ip_infos.push(IpInfo::new(ip.ip(), interface.name.clone()));
            }
        }
        ip_infos
    }
    pub fn new(ip: IpAddr, interface_name: String) -> Self {
        Self {
            ip: ip,
            interface_name: interface_name,
        }
    }
    pub fn is_global(&self) -> bool {
        return self.ip.is_global();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ip_info_list() {
        let ip_info_list = IpInfo::get_ip_info_list();
        for ip_info in ip_info_list {
            dbg!(&ip_info, ip_info.is_global());
        }
    }
}
