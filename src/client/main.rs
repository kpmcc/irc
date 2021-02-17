use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    let mut data = [0_u8; 50];
    stream.write_all(&[104, 105])?;
    match stream.read(&mut data) {
        Ok(_) => match str::from_utf8(&data) {
            Ok(hello) => {
                println!("Server says: {}", hello);
            }
            Err(_) => {
                println!("Error parsing data");
            }
        },
        Err(_) => {
            println!("Error reading");
        }
    };
    Ok(())
}
