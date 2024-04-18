use crate::UdptkError;
use std::net::IpAddr;

/// Send UDP packet to a target.
///
/// This function will first resolve the target to an IP address and a port number,\
/// then bind a UDP socket to a free local port and send the packet to the target.
///
/// The target can be either in the form of "192.168.1.1:80" or "example.com:443".
///
/// The content of the packet is the given string in the `content` parameter.
pub async fn send(target: &str, content: &str) -> Result<(), UdptkError> {
    use tokio::net::UdpSocket;

    let (ip_addr, port) = get_ip_port(target)?;
    tracing::info!(r#"target: "{ip_addr}:{port}", content: "{}""#, content);

    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    tracing::trace!("udp socket bound to: {}", sock.local_addr()?);

    let sent_bytes = sock.send_to(content.as_bytes(), (ip_addr, port)).await?;
    if sent_bytes == content.as_bytes().len() {
        tracing::debug!("packet sent successfully");
    } else {
        tracing::warn!(
            "only {} bytes of {} were sent",
            sent_bytes,
            content.as_bytes().len()
        );
    }

    Ok(())
}

/// Resolve the given target to IP address and port number.
///
/// The target can be either in the form of "192.168.1.1:80" or "example.com:443".\
/// The function will try to resolve the target to an IP address and a port number,
/// and return the result as a tuple `(IpAddr, u16)`.
fn get_ip_port(target: &str) -> Result<(IpAddr, u16), UdptkError> {
    use std::net::ToSocketAddrs;

    let mut addrs_iter = target.to_socket_addrs()?;
    match addrs_iter.find(|addr| addr.is_ipv4()) {
        Some(addr) => Ok((addr.ip(), addr.port())),
        None => Err(UdptkError::NoIpAddress(target.to_string())),
    }
}
