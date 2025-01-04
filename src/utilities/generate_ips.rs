use std::net::{Ipv4Addr};
use std::str::FromStr;
use crate::utilities::resolve::{get_ip_info, IpClass, IpInfo};
use ipnet::{IpAddrRange, Ipv4AddrRange};

fn get_ips_from_range(start: String, end: String) -> Vec<IpInfo> {
    let hosts = IpAddrRange::from(Ipv4AddrRange::new(
        Ipv4Addr::from_str(&start).unwrap(),
        Ipv4Addr::from_str(&end).unwrap(),
    ));
    let mut ips = Vec::new();
    for address in hosts {
        let next_ip = match get_ip_info(address) {
            Ok(ip) => ip,
            Err(_) => continue
        };
        ips.push(next_ip);
    }
    ips
}

pub(crate) fn generate_ips(ip_info: IpInfo) -> Option<impl Iterator<Item = IpInfo>> {
    match ip_info.class {
        IpClass::C => {
            let base = ip_info.ip
                .to_string()
                .rsplitn(2, '.')
                .last()
                .unwrap_or("0.0.0")
                .to_string();
            
            Some(get_ips_from_range(
                format!("{}.1", base),
                format!("{}.254", base)
            ).into_iter())
        }
        _ => { None }
    }
}