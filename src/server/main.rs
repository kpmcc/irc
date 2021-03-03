#[macro_use]
extern crate bitflags;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

mod client;
mod message;
mod channel;
use crate::client::build_client;
use crate::message::parse_message;
use crate::channel::build_channel;

fn handle_message(msg: &str, mut stream: &TcpStream) {
    let m = parse_message(msg);
    println!("Message {:#?}", m);
    stream.write_all(msg.as_bytes()).unwrap();

    // TODO how will we manage a shared client map? idk how locks work in rust
    if let Message::Nickname(nick) = &m {
        let mut client = build_client(nick.to_string());

        println!(
            "Creating client {} -> nick {}",
            stream.peer_addr().unwrap(),
            client.get_nick()
        );
        // Unnecessary, just tryin stuff out
        client.update_nick(String::from("nick_reset"));
    }

    if let Message::Join(channels, _) = m {
        for _ in channels.iter() {
            let channel = build_channel(["nick".to_string()].to_vec());
            println!("Created channel {:#?}", channel);
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0_u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            match str::from_utf8(&data[0..size]) {
                Ok(d) => {
                    handle_message(d, &stream);
                }
                Err(e) => {
                    println!("Error {}", e);
                }
            }
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
