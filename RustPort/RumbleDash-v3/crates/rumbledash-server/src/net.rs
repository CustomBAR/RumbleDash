use std::net::{IpAddr, Ipv4Addr};

/// Best effort discovery of the LAN address of this machine.
///
/// The UDP "connect" trick is used: binding a socket and connecting it does
/// not send a single packet, it only asks the OS which local address would be
/// used to reach the given destination. That is exactly the address a device
/// on the same network should use to reach this machine.
pub fn lan_ip() -> Option<IpAddr> {
    let socket = std::net::UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;

    // 192.0.2.0/24 (TEST-NET-1) is reserved for documentation and therefore
    // never routed anywhere – perfect for this trick.
    if socket.connect((Ipv4Addr::new(192, 0, 2, 1), 9)).is_ok() {
        if let Ok(addr) = socket.local_addr() {
            let ip = addr.ip();
            if !ip.is_loopback() && !ip.is_unspecified() {
                return Some(ip);
            }
        }
    }

    None
}

/// The address that should be advertised to OBS.
pub fn advertised_ip() -> IpAddr {
    lan_ip().unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
}
