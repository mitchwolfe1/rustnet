// src/bot/mod.rs

use std::io::{Read, Write};
use std::net::TcpStream;
use crate::common::BOT_COUNT;

pub fn handle_connection(mut stream: TcpStream) {
    println!("BOT has joined!");
    {
        let mut count = BOT_COUNT.lock().unwrap();
        *count += 1;
    }

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
            {
                let mut count = BOT_COUNT.lock().unwrap();
                *count -= 1;
            }
            break;
        }
        // Echo everything back
        stream.write_all(&buffer[0..size]).unwrap();
    }
}
