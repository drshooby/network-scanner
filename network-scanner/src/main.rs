use std::process::exit;
use crate::utilities::ip_resolve;

mod utilities;

fn main() {
    let my_ip = match ip_resolve::outward_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };

    let ip_class = match ip_resolve::classify_ip(my_ip) {
        ip_resolve::IpClass::A => "A",
        ip_resolve::IpClass::B => "B",
        ip_resolve::IpClass::C => "C",
        _ => "Unknown",
    };

    let is_private = match ip_resolve::is_private_ip(my_ip) {
        Ok(is_private) => is_private,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };

    println!("My IP is: {my_ip}, class {ip_class}, is private: {is_private}");
}
