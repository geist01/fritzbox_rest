use std::net::TcpStream;

use crate::errors::*;

pub fn ping_mobile(ip : &str, port : &str) -> Result<()> {
    TcpStream::connect(format!("{0}:{1}", ip, port))?;
    
    Ok(())
}
