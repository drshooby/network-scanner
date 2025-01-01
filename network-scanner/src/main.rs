use std::process::exit;
use crate::utilities::ip_resolve;

mod utilities;

fn main() {
    println!("Hello, world!");
    let my_ip = match ip_resolve::outward_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };
    
    match ip_resolve::classify_ip(my_ip) {
        ip_resolve::IpClass::A => println!("My IP is an A class IP"),
        ip_resolve::IpClass::B => println!("My IP is an B class IP"),
        ip_resolve::IpClass::C => println!("My IP is an C class IP"),
        _ => println!("Unknown IP class"),
    }
    
    println!("My IP is: {my_ip}");
}
