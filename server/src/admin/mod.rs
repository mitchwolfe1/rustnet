// src/admin/mod.rs

use std::net::TcpStream;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, Read, Write};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use crate::common::{BOT_COUNT, BOT_REGISTRY, ADMIN_REGISTRY};
use crate::common::ADMIN_COUNT;


static MOTD: &str = "
     _______           _______ _________ _        _______ _________
    (  ____ )|\\     /|(  ____ \\\\__   __/( (    /|(  ____ \\\\__   __/
    | (    )|| )   ( || (    \\/   ) (   |  \\  ( || (    \\/   ) (   
    | (____)|| |   | || (_____    | |   |   \\ | || (__       | |   
    |     __)| |   | |(_____  )   | |   | (\\ \\) ||  __)      | |   
    | (\\ (   | |   | |      ) |   | |   | | \\   || (         | |   
    | ) \\ \\__| (___) |/\\____) |   | |   | )  \\  || (____/\\   | |   
    |/   \\__/(_______)\\_______)   )_(   |/    )_)(_______/   )_( \n\n\n";


trait CommandHandler {
    fn handle(&self, stream: &mut TcpStream, args: &[String]);
}
lazy_static! {
    static ref COMMAND_REGISTRY: Arc<Mutex<HashMap<String, Box<dyn CommandHandler + Send + 'static>>>> = {
        let mut m = HashMap::new();
        m.insert("!botcount".to_string(), Box::new(BotCountCommand) as Box<dyn CommandHandler + Send>);
        m.insert("!admincount".to_string(), Box::new(AdminCountCommand) as Box<dyn CommandHandler + Send>);
        m.insert("!showbots".to_string(), Box::new(ShowBotsCommand) as Box<dyn CommandHandler + Send>);
        m.insert("!showadmins".to_string(), Box::new(ShowAdminsCommand) as Box<dyn CommandHandler + Send>);
        Arc::new(Mutex::new(m))
    };
}

struct BotCountCommand;
impl CommandHandler for BotCountCommand {
    fn handle(&self, stream: &mut TcpStream, _args: &[String]) {
        let count = BOT_COUNT.lock().unwrap();
        let response = format!("Connected bots: {}\n", count);
        stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    }
}

struct AdminCountCommand;
impl CommandHandler for AdminCountCommand {
    fn handle(&self, stream: &mut TcpStream, _args: &[String]) {
        let count = ADMIN_COUNT.lock().unwrap();
        let response = format!("Connected admins: {}\n", count);
        stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    }
}

struct ShowBotsCommand;
impl CommandHandler for ShowBotsCommand {
    fn handle(&self, stream: &mut TcpStream, _args: &[String]) {

        let registry = BOT_REGISTRY.lock().unwrap();
        if registry.is_empty() {
            stream.write_all("No bots connected.\n".as_bytes()).expect("Failed to write to stream");
        } else {
            stream.write_all("Connected bots: \n".as_bytes()).expect("Failed to write to stream");
            for bot_id in registry.keys() {
                let bot_line = format!("-: {}\n", bot_id);
                stream.write_all(bot_line.as_bytes()).expect("Failed to write to stream");
            }
        }
    }
}
struct ShowAdminsCommand;
impl CommandHandler for ShowAdminsCommand {
    fn handle(&self, stream: &mut TcpStream, _args: &[String]) {

        let registry = ADMIN_REGISTRY.lock().unwrap();
        if registry.is_empty() {
            stream.write_all("No Admins connected.\n".as_bytes()).expect("Failed to write to stream");
        } else {
            stream.write_all("Connected Admins: \n".as_bytes()).expect("Failed to write to stream");
            for bot_id in registry.keys() {
                let bot_line = format!("-: {}\n", bot_id);
                stream.write_all(bot_line.as_bytes()).expect("Failed to write to stream");
            }
        }
    }
}

pub fn add_admin(admin_id: String, stream: TcpStream) {
    let stream = Arc::new(Mutex::new(stream));
    {
        let mut registry = ADMIN_REGISTRY.lock().unwrap();
        registry.insert(admin_id, stream);
    }
    {
        let mut count = ADMIN_COUNT.lock().unwrap();
        *count += 1;
    }
}

pub fn remove_admin(admin_id: &str) {
    {
        let mut registry = ADMIN_REGISTRY.lock().unwrap();
        registry.remove(admin_id);
    }
    {
        let mut count = ADMIN_COUNT.lock().unwrap();
        *count -= 1;
    }
}


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

fn handle_authenticated_session(mut stream: TcpStream, username: &str) {
    println!("{} has joined!", username);
    let admin_addr = stream.peer_addr().unwrap().to_string();
    add_admin(admin_addr.clone(), stream.try_clone().expect("Failed to clone TcpStream"));

    stream.write_all(MOTD.as_bytes()).unwrap();
    stream.write_all(b"rustnet> ").unwrap();
    stream.flush().expect("Failed to flush the stream");

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
            remove_admin(&admin_addr);
            break;
        }
        
        // handle commands
        let input = match std::str::from_utf8(&buffer[..size]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid UTF-8 sequence: {}", e);
                continue;
            },
        };
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        if let Some(command) = args.first() {
            let command_registry = COMMAND_REGISTRY.lock().unwrap();
            if let Some(handler) = command_registry.get(command) {
                handler.handle(&mut stream, &args[1..]);
            } else {
                // Handle unknown command
                stream.write_all(b"Unknown command\n").expect("Failed to write to stream");
            }
            stream.write_all(b"rustnet> ").unwrap();
        }
    }
}

