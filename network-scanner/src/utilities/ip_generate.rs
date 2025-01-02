use std::net::{Ipv4Addr};
use std::str::FromStr;
use crate::utilities::ip_resolve::{get_ip_info, IpClass, IpInfo};
use ipnet::{IpAddrRange, Ipv4AddrRange};

fn get_a_class_range() -> (String, String) {
    (String::from("0.0.0.0"), String::from("127.255.255.255"))
}

fn get_b_class_range() -> (String, String) {
    (String::from("128.0.0.0"), String::from("191.255.255.255"))
}

fn get_c_class_range() -> (String, String) {
    (String::from("192.0.0.0"), String::from("223.255.255.255"))
}

pub(crate) fn generate_ips(ip_info: IpInfo) -> Result<Vec<IpInfo>, String> {
    
    let (start, end) = match ip_info.class {
        IpClass::A => get_a_class_range(),
        IpClass::B => get_b_class_range(),
        IpClass::C => get_c_class_range(),
        _ => return Err(String::from("Unknown class")),
    };

    let hosts = IpAddrRange::from(Ipv4AddrRange::new(
        Ipv4Addr::from_str(&start).unwrap(),
        Ipv4Addr::from_str(&end).unwrap(),
    ));

    let mut ips = Vec::new();

    let mut count = 0;
    for address in hosts {
        if count> 100 {
            break;
        }
        let next_ip = match get_ip_info(address) {
            Ok(ip) => ip,
            Err(_) => continue
        };
        ips.push(next_ip);
        count += 1;
    }

    Ok(ips)
}