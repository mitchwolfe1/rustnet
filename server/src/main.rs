// src/main.rs

mod admin;
mod bot;
mod common;

use clap::Parser;
use common::start_listeners;
use std::thread;

#[derive(Parser)]
struct Cli {
    admin_port: i32,
    bot_port: i32,
}

fn main() {
    let args = Cli::parse();
    let (admin_listener, bot_listener) = start_listeners(args.admin_port, args.bot_port);

    thread::spawn(move || {
        for stream in admin_listener.incoming() {
            if let Ok(stream) = stream {
                thread::spawn(|| admin::handle_connection(stream));
            }
        }
    });

    thread::spawn(move || {
        for stream in bot_listener.incoming() {
            if let Ok(stream) = stream {
                thread::spawn(|| bot::handle_connection(stream));
            }
        }
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
