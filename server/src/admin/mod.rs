// src/admin/mod.rs


use std::net::TcpStream;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, Read, Write};

pub fn load_credentials() -> HashMap<String, String> {
    let file = File::open("admin.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut credentials = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            credentials.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    credentials
}

pub fn handle_connection(mut stream: TcpStream, credentials: HashMap<String, String>) {
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));

    // Prompt for username
    stream.write_all(b"Username: ").expect("Failed to write to stream");
    let mut username = String::new();
    reader.read_line(&mut username).expect("Failed to read from stream");
    let username = username.trim_end();

    // Prompt for password
    stream.write_all(b"Password: ").expect("Failed to write to stream");
    let mut password = String::new();
    reader.read_line(&mut password).expect("Failed to read from stream");
    let password = password.trim_end();

    // Validate credentials
    if credentials.get(username) == Some(&password.to_string()) {
        // Proceed with authenticated session
        handle_authenticated_session(stream, username);
    } else {
        // Invalid login
        stream.write_all(b"Invalid login\n").expect("Failed to write to stream");
        return;
    }
}



pub fn handle_authenticated_session(mut stream: TcpStream, username: &str) {
    println!("{} has joined!", username);

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
            println!("{} has disconnected", username);
            break;
        }
        // Echo everything back
        stream.write_all(&buffer[0..size]).unwrap();
    }
}
