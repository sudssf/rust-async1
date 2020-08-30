use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

use log::*;
use tungstenite::handshake::HandshakeRole;
use tungstenite::{accept, Error, HandshakeError, Message, Result};

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;
    let mut counter = 1;
    info!("Running test");
    loop {
        counter += 1;
        socket.write_message(Message::Text(counter.to_string()))?
        
        /*match socket.read_message()? {
            msg @ Message::Text(_) | msg @ Message::Binary(_) => {
                socket.write_message(msg)?;
            }
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) => {}
        }*/
    }
}

fn main() {
    env_logger::init();

    let server = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in server.incoming() {
        spawn(move || match stream {
            Ok(stream) => if let Err(err) = handle_client(stream) {
                match err {
                    Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                    e => error!("test: {}", e),
                }
            },
            Err(e) => error!("Error accepting stream: {}", e),
        });
    }
}