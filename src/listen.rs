use crate::UdptkError;
use tracing::info;

pub async fn listen(port: u16) -> Result<(), UdptkError> {
    use tokio::net::UdpSocket;

    info!("app version: {}", crate::PKG_VERSION);
    info!("listening at {}", port);

    let sock = UdpSocket::bind(("0.0.0.0", port)).await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        tracing::info!("{:?} bytes received from {:?}", len, addr);
    }
    Ok(())
}
