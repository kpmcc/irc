#[macro_use]
extern crate bitflags;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

mod channel;
mod to_clrf_reader_writer;
mod client;
mod message;
use crate::channel::build_channel;
use crate::to_clrf_reader_writer::ToClrfReaderWriter;
use crate::client::build_client;
use crate::client::Client;
use crate::message::parse_message;
use crate::message::Message;

fn handle_message<T: Write>(
    msg: &str,
    stream: &mut T,
    client_map: &mut HashMap<String, Client>,
    nick_ref: &mut String,
) {
    let m = parse_message(msg);
    println!("Message {:#?}", m);
    stream.write_all(msg.as_bytes()).unwrap();

    if let Message::Nickname(nick) = &m {
        if !nick_ref.is_empty() {
            println!("Changing nick from {} -> {}", nick_ref, nick.to_string());
            if let Some(mut client) = client_map.remove(nick_ref) {
                client.update_nick(nick.to_string());
                client_map.insert(client.get_nick().to_string(), client);
            } else {
                println!("Existing nick not found?");
            }
        } else {
            if client_map.contains_key(nick) {
                println!("Nickname {} is already taken!", nick);
                return;
            }

            let client = build_client(nick.to_string());

            client_map.insert(client.get_nick().to_string(), client);
        }

        *nick_ref = nick.to_string();
        println!("full client map");
        for (key, value) in client_map {
            println!("{} -> {}", key, value.get_nick());
        }
    }

    if let Message::Join(channels, _) = m {
        for _ in channels.iter() {
            let channel = build_channel(["nick".to_string()].to_vec());
            println!("Created channel {:#?}", channel);
        }
    }
}

fn handle_client(mut stream: &mut TcpStream, client_clone: Arc<Mutex<HashMap<String, Client>>>) -> Result<(), String> {
    let mut data = [0_u8; 50]; // using 50 byte buffer
    let mut nick = String::new();
    let mut stream = ToClrfReaderWriter::new(&mut stream);
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                let mut client_data = client_clone.lock().unwrap();
                if size == 0 {
                    break;
                }

                // echo everything, returning error if decoding failed
                match str::from_utf8(&data[0..size]) {
                    Ok(d) => {
                        handle_message(d, &mut stream, &mut *client_data, &mut nick);
                    },
                    Err(e) => {
                        return Err(format!("{}", e));
                    }
                }
            },
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }
    if !nick.is_empty() {
        // we are exiting so remove ourselves from client map
        let mut client_data = client_clone.lock().unwrap();
        client_data.remove(&nick);
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    let client_map: HashMap<String, Client> = HashMap::new();
    let mutex = Arc::new(Mutex::new(client_map));

    for stream in listener.incoming() {
        let client_clone = Arc::clone(&mutex);
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("New connection: {}", peer_addr);
                thread::spawn(move || {
                    // connection succeeded
                    match handle_client(&mut stream, client_clone) {
                        Ok(_) => {
                            println!(
                                "Terminating connection with {} without error",
                                peer_addr,
                            );
                        },
                        Err(e) => {
                            println!(
                                "An error occurred, terminating connection with {}: {}",
                                peer_addr,
                                e,
                            );
                        }
                    }
                    stream.shutdown(Shutdown::Both).unwrap();
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
