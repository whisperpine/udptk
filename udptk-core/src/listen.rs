use std::future::Future;

// Listens on a UDP port until Ctrl+C or terminate signal (on unix platforms) is received.
// The returned future resolves when either event is triggered or an error occurs.
// The error is logged but not returned, as the program will shut down anyway.
pub async fn listen(port: u16) -> crate::Result<()> {
    let (ctrl_c, terminate) = graceful_shutdown();
    tokio::select! {
        // Wait for Ctrl+C signal
        _ = ctrl_c => Ok(()),
        // Wait for terminate signal (on unix platforms)
        _ = terminate => Ok(()),
        // Wait for errors in the listener
        output = listen_core(port) => output,
    }
}

// Core UDP listener that runs until an error occurs.
// Logs the app version, listens on the given port and logs any received messages.
async fn listen_core(port: u16) -> crate::Result<()> {
    use tokio::net::UdpSocket;
    use tracing::info;

    info!("app version: {}", env!("CARGO_PKG_VERSION"));
    let sock = UdpSocket::bind(("0.0.0.0", port)).await?;
    info!("listening at port {}", port);
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        tracing::debug!("{:?} bytes received from {:?}", len, addr);
        let content = std::str::from_utf8(&buf[..len])?;
        info!(%content);
    }
}

/// Waits for a Ctrl+C signal or a terminate signal on unix platforms.
///
/// Returns two futures that can be used to wait for either event.
///
/// The Ctrl+C future is resolved when a Ctrl+C signal is received.
/// The terminate future is resolved when a terminate signal is received on unix platforms.
/// On non-unix platforms the terminate future is a no-op future that never resolves.
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
