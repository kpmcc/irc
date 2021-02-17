use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    stream.write_all(&[1])?;
    stream.read_exact(&mut [0; 128])?;
    Ok(())
}
