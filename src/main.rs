use std::process::exit;
use surge_ping::{Client, Config};
use crate::utilities::{ping, resolve};
use crate::utilities::generate_ips::generate_ips;
use crate::utilities::resolve::get_ip_info;

mod utilities;

#[tokio::main]
async fn main() {
    let my_ip = match resolve::outbound_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };

    let ip_info = get_ip_info(my_ip).unwrap();
    let ip_range_it = generate_ips(ip_info).unwrap();
    
    let client_v4 = Client::new(&Config::default()).unwrap();
    
    match ping::check_active_ips(ip_range_it, client_v4).await { 
        Ok(active_ips) => {
            println!("Total ips found: {}", active_ips.len());
            for ip in active_ips {
                if ip.ip == my_ip { 
                    println!("Address: {}, Name {} -> Me", ip.ip, ip.hostname);
                    continue; 
                }
                println!("Address: {}, Name {}", ip.ip, ip.hostname);
            }
        }
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    }
}
