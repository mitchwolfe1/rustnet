// server/src/main.rs
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    admin_port: i32,
    bot_port: i32,
}

fn main() {
    let args = Cli::parse();
    let listeners = start_listeners(args.admin_port, args.bot_port);

    // start a new thread for the admin listener
    let admin_listener = listeners.0;
    thread::spawn(move || {
        for stream in admin_listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        handle_admin_connection(stream) // use a specific handler for admin
                    });
                }
                Err(e) => {
                    println!("Failed to accept admin connection: {}", e);
                }
            }
        }
    });

    // start a new thread for the bot listener
    let bot_listener = listeners.1;
    thread::spawn(move || {
        for stream in bot_listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        handle_bot_connection(stream) // use a specific handler for bots
                    });
                }
                Err(e) => {
                    println!("Failed to accept bot connection: {}", e);
                }
            }
        }
    });

    // prevent the main thread from exiting immediately
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn start_listeners(admin_port: i32, bot_port: i32) -> (TcpListener, TcpListener) {
    let admin_host = format!("0.0.0.0:{}", &admin_port);
    let bot_host = format!("0.0.0.0:{}", &bot_port);

    let admin_listener = TcpListener::bind(&admin_host).expect("couldn't bind to admin address");
    let bot_listener = TcpListener::bind(&bot_host).expect("couldn't bind to bot address");
    println!("Admin server is running on {}", &admin_host);
    println!("Bot server is running on {}", &bot_host);

    return (admin_listener, bot_listener);
}

fn handle_admin_connection(mut stream: TcpStream) {
    println!("An admin has joined!");

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
            println!("Admin has disconnected");
            break;
        }
        // Echo everything back
        stream.write_all(&buffer[0..size]).unwrap();
    }
}


fn handle_bot_connection(mut stream: TcpStream) {
    println!("A BOT has joined!");

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

