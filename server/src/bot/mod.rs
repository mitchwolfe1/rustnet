// src/bot/mod.rs

use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    println!("BOT has joined!");

    let mut buffer = [0; 1024];
    loop {
        let size = match stream.read(&mut buffer) {
            Ok(size) => size,
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                break;
            }
        };
        if size == 0 {
            println!("BOT has disconnected");
            break;
        }
        // Echo everything back
        stream.write_all(&buffer[0..size]).unwrap();
    }
}
