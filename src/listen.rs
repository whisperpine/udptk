use tracing::info;

pub fn listen(port: u16) -> Result<(), crate::UdptkError> {
    info!("app version: {}", crate::PKG_VERSION);
    info!("listening at {}", port);

    Ok(())
}
