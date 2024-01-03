
# rustnet
Rustnet is a botnet system developed in Rust and modeled after qBot. The project includes a server-side application and a client-side bot. The server manages connections from both admin users and bots, allowing admins to execute various commands. The client-side bot connects to the server and awaits instructions. This project is a proof of concept and is intended for educational purposes only.

## Setup
Install rust and cargo:
```curl https://sh.rustup.rs -sSf | sh"```

Configure current shell: ```source "$HOME/.cargo/env"```

Install/Configure dependencies and run cross-compilation: 
```python3 cc7-rust.py client/ <IP_ADDR>```

