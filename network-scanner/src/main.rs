use std::process::exit;
use crate::utilities::outward_ip;

mod utilities;

fn main() {
    println!("Hello, world!");
    let my_ip = match outward_ip::outward_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };
    println!("My IP is: {my_ip}");
}
