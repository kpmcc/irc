#[macro_use]
extern crate bitflags;
extern crate structopt;

use std::net::TcpListener;

mod channel;
mod client;
mod message;
mod serverstate;
mod to_clrf_reader_writer;

use crate::serverstate::ServerState;
use structopt::StructOpt;

#[derive(StructOpt)]
struct MyArgs {
    #[structopt(short = "p", long = "port", default_value = "3333")]
    port: u32,
}

fn main() {

    let port = MyArgs::from_args().port;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port {}", port);

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
