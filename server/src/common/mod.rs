// src/common/mod.rs

use std::net::TcpListener;

static IP_ADDR: &str = "0.0.0.0";

pub fn start_listeners(admin_port: i32, bot_port: i32) -> (TcpListener, TcpListener) {
    let admin_host = format!("{}:{}", IP_ADDR, &admin_port);
    let bot_host = format!("{}:{}", IP_ADDR, &bot_port);

    let admin_listener = TcpListener::bind(&admin_host).expect("couldn't bind to admin address");
    let bot_listener = TcpListener::bind(&bot_host).expect("couldn't bind to bot address");
    println!("Admin server is running on {}", &admin_host);
    println!("Bot server is running on {}", &bot_host);

    return (admin_listener, bot_listener);
}
