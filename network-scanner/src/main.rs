use std::process::exit;
use crate::utilities::{ip_resolve};
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

    match get_ip_info(my_ip) { 
        Ok(ip) => println!("Ip: {}, Class: {}, Visibility: {}", ip.ip, ip.class.as_ref(), ip.visibility.as_ref()),
        Err(e) => println!("Error: {e}")
    }

}
