use std::net::{IpAddr, UdpSocket};

pub enum IpClass {
    A,
    B,
    C,
    Unknown,
}

pub(crate) fn outbound_ip() -> Result<IpAddr, String> {
    let google_ipv4: &str = "8.8.8.8:443";
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

fn is_a_type_private(octets: Vec<u8>) -> bool {
    octets[0] == 10
}

fn is_b_type_private(octets: Vec<u8>) -> bool {
    octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31)
}

fn is_c_type_private(octets: Vec<u8>) -> bool {
    octets[0] == 192 && octets[1] == 168
}

pub(crate) fn is_private_ip(ip: IpAddr) -> Result<bool, String> {
    let ip_class: IpClass = classify_ip(ip);
    let octets: Vec<u8> = ip.to_string()
        .split('.')
        .map(|v| v.parse::<u8>().map_err(|_| format!("Could not parse octet: {v}")))
        .collect::<Result<Vec<u8>, _>>()?;

    match ip_class {
        IpClass::A => Ok(is_a_type_private(octets)),
        IpClass::B => Ok(is_b_type_private(octets)),
        IpClass::C => Ok(is_c_type_private(octets)),
        _ => Err(String::from("Unknown IP class")),
    }
}