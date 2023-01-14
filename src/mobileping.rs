use anyhow::Result;
use std::net::TcpStream;

pub fn ping_mobile(ip: &str, port: u16) -> Result<()> {
    TcpStream::connect(format!("{0}:{1}", ip, port))?;

    Ok(())
}
