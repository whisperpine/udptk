use std::net::{IpAddr, Ipv4Addr};

use crate::UdptkError;

pub async fn send(domain: Option<String>, port: u16, content: String) -> Result<(), UdptkError> {
    use tokio::net::UdpSocket;

    let bind_port = get_free_port()?;
    tracing::debug!(%bind_port);

    let ip_addr = match domain {
        Some(value) => get_ip(&value).await?,
        None => IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    };
    tracing::debug!(%ip_addr);

    let sock = UdpSocket::bind(("0.0.0.0", bind_port)).await?;
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
            return Ok(port);
        }
    }
    Err(UdptkError::NoFreeSocket)
}

async fn get_ip(domain: &str) -> Result<IpAddr, UdptkError> {
    use trust_dns_resolver::config::*;
    use trust_dns_resolver::name_server::TokioConnectionProvider;
    use trust_dns_resolver::AsyncResolver;

    // Construct a new Resolver with default configuration options
    let mut resolver = AsyncResolver::new(
        ResolverConfig::default(),
        ResolverOpts::default(),
        TokioConnectionProvider::default(),
    );

    // Lookup the IP addresses associated with a name.
    let mut response = resolver.lookup_ip(domain).await?;

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    match response.iter().next() {
        Some(addr) => Ok(addr),
        None => Err(UdptkError::NoIpAddress(domain.to_string())),
    }
}
