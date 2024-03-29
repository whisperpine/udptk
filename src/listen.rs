use crate::UdptkError;
use std::future::Future;

/// Listen to the given port.
///
/// Can be cancelled by `Ctrl+C`.
pub async fn listen(port: u16) -> Result<(), UdptkError> {
    let (ctrl_c, terminate) = graceful_shutdown();
    tokio::select! {
        _ = ctrl_c => Ok(()),
        _ = terminate => Ok(()),
        output = listen_core(port) => output,
    }
}

/// Listen to the given port.
async fn listen_core(port: u16) -> Result<(), UdptkError> {
    use tokio::net::UdpSocket;
    use tracing::info;
    info!("app version: {}", crate::PKG_VERSION);
    let sock = UdpSocket::bind(("0.0.0.0", port)).await?;
    info!("listening at port {}", port);
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        tracing::debug!("{:?} bytes received from {:?}", len, addr);
        let content = String::from_utf8_lossy(&buf[..len]);
        info!(%content);
    }
}

fn graceful_shutdown() -> (impl Future<Output = ()>, impl Future<Output = ()>) {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    (ctrl_c, terminate)
}
