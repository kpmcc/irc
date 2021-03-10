#[macro_use]
extern crate bitflags;

use std::net::TcpListener;
mod channel;
mod client;
mod message;
mod serverstate;
mod to_clrf_reader_writer;

use crate::serverstate::ServerState;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    let server = ServerState::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => server.handle_incoming_client(stream),
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
