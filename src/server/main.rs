#[macro_use]
extern crate bitflags;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

mod channel;
mod client;
mod message;
use crate::channel::build_channel;
use crate::client::build_client;
use crate::client::Client;
use crate::message::parse_message;
use crate::message::Message;

fn handle_message(
    msg: &str,
    mut stream: &TcpStream,
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
            let client = build_client(nick.to_string());

            println!(
                "Creating client {} -> nick {} mode {}",
                stream.peer_addr().unwrap(),
                client.get_nick(),
                client.get_mode()
            );

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

fn handle_client(mut stream: TcpStream, client_clone: Arc<Mutex<HashMap<String, Client>>>) {
    let mut data = [0_u8; 50]; // using 50 byte buffer
    let mut nick = String::new();
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                let mut client_data = client_clone.lock().unwrap();
                if size == 0 {
                    break;
                }

                // echo everything!
                match str::from_utf8(&data[0..size]) {
                    Ok(d) => {
                        handle_message(d, &stream, &mut *client_data, &mut nick);
                    }
                    Err(e) => {
                        println!("Error {}", e);
                        break;
                    }
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
    if !nick.is_empty() {
        // we are exiting so remove ourselves from client map
        let mut client_data = client_clone.lock().unwrap();
        client_data.remove(&nick);
    }
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
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream, client_clone)
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
