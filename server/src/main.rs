// server/src/main.rs
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    admin_port: i32,
}

fn main() {
    let args = Cli::parse();
    let admin_host = format!("0.0.0.0:{}", &args.admin_port);
    let listener = TcpListener::bind(&admin_host).expect("couldn't bind to address");
    println!("Server is running on {}", &admin_host);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // delegate to handler
                thread::spawn(|| {
                    handle_connection(stream)
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // echo everything back
            stream.write(&buffer[0..size]).unwrap();
            true
        }
        Err(_) => {
            // complain and close connection
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}
