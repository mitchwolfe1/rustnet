// src/bot/mod.rs

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::common::{BOT_COUNT, BOT_REGISTRY};



pub fn add_bot(bot_id: String, stream: TcpStream) {
    let stream = Arc::new(Mutex::new(stream));
    {
        let mut registry = BOT_REGISTRY.lock().unwrap();
        registry.insert(bot_id, stream);
    }
    {
        let mut count = BOT_COUNT.lock().unwrap();
        *count += 1;
    }
}

pub fn remove_bot(bot_id: &str) {
    {
        let mut registry = BOT_REGISTRY.lock().unwrap();
        registry.remove(bot_id);
    }
    {
        let mut count = BOT_COUNT.lock().unwrap();
        *count -= 1;
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    println!("BOT has joined!");
    let bot_addr = stream.peer_addr().unwrap().to_string();
    add_bot(bot_addr.clone(), stream.try_clone().expect("Failed to clone TcpStream"));
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
            remove_bot(&bot_addr);
            break;
        }
        // Echo everything back
        stream.write_all(&buffer[0..size]).unwrap();
    }
}
