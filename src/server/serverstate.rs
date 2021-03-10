use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::to_clrf_reader_writer::ToClrfReaderWriter;

use crate::channel::build_channel;
use crate::channel::Channel;
use crate::client::build_client;
use crate::client::Client;
use crate::message::parse_message;
use crate::message::Message;

pub struct ServerState {
    client_mutex: Arc<Mutex<HashMap<String, Client>>>,
    channel_mutex: Arc<Mutex<HashMap<String, Channel>>>,
}

impl ServerState {
    pub fn new() -> ServerState {
        let client_map: HashMap<String, Client> = HashMap::new();
        let channel_map: HashMap<String, Channel> = HashMap::new();
        let client_mutex: Arc<Mutex<HashMap<String, Client>>> = Arc::new(Mutex::new(client_map));
        let channel_mutex: Arc<Mutex<HashMap<String, Channel>>> = Arc::new(Mutex::new(channel_map));
        ServerState {
            client_mutex,
            channel_mutex,
        }
    }

    pub fn get_client_map(&self) -> Arc<Mutex<HashMap<String, Client>>> {
        Arc::clone(&self.client_mutex)
    }

    pub fn get_channel_map(&self) -> Arc<Mutex<HashMap<String, Channel>>> {
        Arc::clone(&self.channel_mutex)
    }

    fn handle_client(
        mut stream: &mut TcpStream,
        client_clone: Arc<Mutex<HashMap<String, Client>>>,
        channel_clone: Arc<Mutex<HashMap<String, Channel>>>,
    ) -> Result<(), String> {
        let mut data = [0_u8; 50]; // using 50 byte buffer
        let mut nick = String::new();
        let mut stream = ToClrfReaderWriter::new(&mut stream);
        loop {
            match stream.read(&mut data) {
                Ok(size) => {
                    let mut client_data = client_clone.lock().unwrap();
                    let mut channel_data = channel_clone.lock().unwrap();
                    if size == 0 {
                        break;
                    }

                    // echo everything!
                    match str::from_utf8(&data[0..size]) {
                        Ok(d) => {
                            ServerState::handle_message(
                                d,
                                &mut stream,
                                &mut *client_data,
                                &mut *channel_data,
                                &mut nick,
                            );
                        }
                        Err(e) => return Err(format!("{}", e)),
                    }
                }
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

    fn handle_message<T: Write>(
        msg: &str,
        stream: &mut T,
        client_map: &mut HashMap<String, Client>,
        channel_map: &mut HashMap<String, Channel>,
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
                println!(
                    "Creating client; -> nick {} mode {}",
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
            for chan_name in channels.iter() {
                let channel = build_channel(["nick".to_string()].to_vec());
                println!("Created channel {:#?}", channel);
                channel_map.insert(chan_name.to_string(), channel);
            }
        }
    }

    pub fn handle_incoming_client(&self, mut stream: TcpStream) {
        let peer_addr = stream.peer_addr().unwrap();
        println!("New connection: {}", peer_addr);
        let client_clone = self.get_client_map();
        let channel_clone = self.get_channel_map();
        thread::spawn(move || {
            // connection succeeded
            match ServerState::handle_client(&mut stream, client_clone, channel_clone) {
                Ok(_) => {
                    println!("Terminating connection with {} without error", peer_addr,);
                }
                Err(e) => {
                    println!(
                        "An error occurred, terminating connection with {}: {}",
                        peer_addr, e,
                    );
                }
            }
            stream.shutdown(Shutdown::Both).unwrap();
        });
    }
}
