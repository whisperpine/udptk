use crate::UdptkError;
use std::net::IpAddr;

pub async fn send(target: String, content: String) -> Result<(), UdptkError> {
    use tokio::net::UdpSocket;

    let (ip_addr, port) = get_ip_addr(&target)?;
    tracing::debug!("target ip address: {}", ip_addr);

    let sock = UdpSocket::bind(("0.0.0.0", get_free_port()?)).await?;
    sock.send_to(content.as_bytes(), (ip_addr, port)).await?;
    tracing::info!(r#"target: "{ip_addr}:{port}", content: "{}""#, content);

    Ok(())
}

fn get_free_port() -> Result<u16, UdptkError> {
    use rand::Rng;
    use std::net::UdpSocket;

    const MAX_TRY_TIMES: u32 = 50;
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_TRY_TIMES {
        let port: u16 = rng.gen_range(5000..9000);
        if UdpSocket::bind(("0.0.0.0", port)).is_ok() {
            tracing::trace!("port to bind with: {}", port);
            return Ok(port);
        }
    }
    Err(UdptkError::NoFreeSocket)
}

fn get_ip_addr(domain: &str) -> Result<(IpAddr, u16), UdptkError> {
    use std::net::ToSocketAddrs;

    let mut addrs_iter = domain.to_socket_addrs()?;
    match addrs_iter.find(|addr| addr.is_ipv4()) {
        Some(addr) => Ok((addr.ip(), addr.port())),
        None => Err(UdptkError::NoIpAddress(domain.to_string())),
    }
}
