use std::net::{IpAddr, Ipv4Addr};
use crate::utilities::ip_resolve::{get_ip_info, IpClass, IpInfo};


fn get_a_class_range() -> (u32, u32) {
    (0, 2147483647)
}

fn get_b_class_range() -> (u32, u32) {
    (2147483648, 3221225471)
}

fn get_c_class_range() -> (u32, u32) {
    (3221225472, 3758096383)
}

pub(crate) fn generate_ips(ip_info: IpInfo) -> Result<Vec<IpInfo>, String> {

    let class = ip_info.class;
    let (start, end) = match class {
        IpClass::A => get_a_class_range(),
        IpClass::B => get_b_class_range(),
        IpClass::C => get_c_class_range(),
        _ => return Err(String::from("Unknown class")),
    };

    let mut ips = Vec::new();

    for i in start..=end {
        let curr_ip = IpAddr::V4(Ipv4Addr::from(i));
        println!("Generating ip: {}", curr_ip);
        let next_ip = match get_ip_info(curr_ip) {
            Ok(ip) => ip,
            Err(_) => continue
        };
        ips.push(next_ip);
    }

    Ok(ips)
}