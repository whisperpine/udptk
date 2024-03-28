use crate::UdptkError;

pub fn send(content: String) -> Result<(), UdptkError> {
    println!("send: {}", content);

    Ok(())
}
