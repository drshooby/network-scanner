use std::net::{IpAddr, UdpSocket};

pub enum IpClass {
    A,
    B,
    C,
    Unknown,
}

pub(crate) fn outward_ip() -> Result<IpAddr, String> {
    let google_ipv4: &str = "8.8.8.8:80";
    let wildcard_ipv4 = "0.0.0.0:0";

    let socket = match UdpSocket::bind(wildcard_ipv4) {
        Ok(socket) => socket,
        Err(e) => return Err(format!("Could not bind to socket: {e}"))
    };

    match socket.connect(google_ipv4) {
        Ok(_) => {
            match socket.local_addr() {
                Ok(addr) => Ok(addr.ip()),
                Err(e) => Err(format!("Could not get local address: {e}"))
            }
        }
        Err(e) => Err(format!("Could not connect to google: {e}"))
    }
}

pub(crate) fn classify_ip(ip: IpAddr) -> IpClass {
    let first_octet = ip.to_string().split('.').next().unwrap_or("0").parse::<u8>().unwrap_or(0);

    match first_octet {
        1..=127 => IpClass::A,
        128..=191 => IpClass::B,
        192..=223 => IpClass::C,
        _ => IpClass::Unknown,
    }
}