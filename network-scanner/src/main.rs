use std::process::exit;
use surge_ping::{Client, Config};
use crate::utilities::{ip_ping, ip_resolve};
use crate::utilities::ip_generate::generate_ips;
use crate::utilities::ip_resolve::get_ip_info;

mod utilities;

#[tokio::main]
async fn main() {
    let my_ip = match ip_resolve::outbound_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };

    let ip_info = get_ip_info(my_ip).unwrap();
    let ips = generate_ips(ip_info).unwrap();
    
    let client_v4 = Client::new(&Config::default()).unwrap();
    
    match ip_ping::check_active_ips(ips, client_v4).await { 
        Ok(active_ips) => {
            println!("Total ips found: {}", active_ips.len());
            for ip in active_ips {
                if ip.ip == my_ip { 
                    println!("{} -> me", ip.ip);
                    continue; 
                }
                println!("{} is active", ip.ip);
            }
        }
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    }
}
