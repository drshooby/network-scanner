use std::process::exit;
use crate::utilities::{ip_resolve};
use crate::utilities::ip_generate::generate_ips;
use crate::utilities::ip_resolve::get_ip_info;

mod utilities;

fn main() {
    let my_ip = match ip_resolve::outbound_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };

    let ip_info = get_ip_info(my_ip).unwrap();
    let ips = generate_ips(ip_info).unwrap();
    for ip in ips {
        println!("Ip: {}, Class: {}, Visibility: {}", ip.ip, ip.class.as_ref(), ip.visibility.as_ref());
    }
}
