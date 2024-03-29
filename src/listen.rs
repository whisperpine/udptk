use crate::UdptkError;
use tracing::info;

pub async fn listen(port: u16) -> Result<(), UdptkError> {
    use tokio::net::UdpSocket;

    info!("app version: {}", crate::PKG_VERSION);
    info!("listening at port {}", port);

    let sock = UdpSocket::bind(("0.0.0.0", port)).await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        tracing::debug!("{:?} bytes received from {:?}", len, addr);
        let content = String::from_utf8_lossy(&buf[..len]);
        info!(%content);
    }
}
