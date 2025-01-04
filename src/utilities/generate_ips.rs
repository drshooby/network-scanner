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

pub(crate) fn generate_ips(ip_info: IpInfo) -> Result<impl Iterator<Item = IpInfo>, String> {
    
    let is_default_scan = match ip_info.class {
        IpClass::A | IpClass::B => false,
        IpClass::C => true,
        _ => return Err(String::from("Unknown class")),
    };

    let base = ip_info.ip
        .to_string()
        .rsplitn(2, '.')
        .last()
        .unwrap_or("0.0.0")
        .to_string();

    if is_default_scan {
        let ips = get_ips_from_range(
            format!("{}.1", base),
            format!("{}.254", base),
        );
        return Ok(ips.into_iter());
    }

    Err(String::from("Unable to generate ips"))
}