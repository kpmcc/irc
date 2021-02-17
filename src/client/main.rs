use std::io::prelude::*;
use std::net::{TcpStream};
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    let mut data = [0 as u8; 50];
    let mut hello = "";
    stream.write(&[1])?;
    match stream.read(&mut data) {
        Ok(size) => {
            match str::from_utf8(&data) {
                Ok(hello) => {
                    println!("working {}", hello); 
                }
                Err(_) => {
                    println!("Error parsing data");
                }
            }
        }
        Err(_) => {
            println!("Error reading");
        }

    };
    Ok(())
}
