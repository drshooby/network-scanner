use std::net::{IpAddr, UdpSocket};

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