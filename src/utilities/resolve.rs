use std::net::{IpAddr, UdpSocket};
use strum_macros::AsRefStr;

#[derive(AsRefStr)]
pub enum IpClass {
    C,
    Unknown,
}

#[derive(AsRefStr)]
pub enum IpVisStatus {
    Public,
    Private,
    Unknown,
}

pub struct IpInfo {
    // just for testing, otherwise keep private
    pub class: IpClass,
    pub ip: IpAddr,
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
        192..=223 => IpClass::C,
        _ => IpClass::Unknown,
    }
}

fn get_c_class_visibility(octets: Vec<u8>) -> IpVisStatus {
    if octets[0] == 192 && octets[1] == 168 { IpVisStatus::Private } else { IpVisStatus::Public }
}

pub(crate) fn get_ip_info(ip: IpAddr) -> Result<IpInfo, String> {
    let ip_class: IpClass = classify_ip(ip);
    let octets: Vec<u8> = ip.to_string()
        .split('.')
        .map(|v| v.parse::<u8>().map_err(|_| format!("Could not parse octet: {v}")))
        .collect::<Result<Vec<u8>, _>>()?;

    match ip_class {
        IpClass::C => get_c_class_visibility(octets),
        _ => IpVisStatus::Unknown,
    };

    Ok(IpInfo {
        class: ip_class,
        ip,
    })
}
