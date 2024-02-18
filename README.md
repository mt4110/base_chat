# NixOS + Rust + Chat
A base for a general-purpose chat system.
Currently, it handles only basic and simple processes.

## Execution Environment: NixOS + Required Software
- MAC OS M1 (2020) OS Sonoma | TODO: Win, Linux
- NixOS
- rustc 1.75.0 (82e1608df 2023-12-21) (built from a source tarball)
- cargo 1.75.0

### Installing NixOS
A `shell.nix` file is already prepared. Just run:
```shell
sh <(curl -L https://nixos.org/nix/install)
nix --help
```
That's all you need.<br>
[Official NixOS Website](https://nixos.org/)

## Starting NixOS
```
cd XXX/nix+Rust
nix-shell
nix-env -iA nixpkgs.nixpkgs-fmt
```

## Test Run
1.Start the WebSocket server<br>
※After starting NixOS:
```
cd websocket-chat
cargo run -p websocket-chat-server

Listening TCP on: 127.0.0.1:53939

# Output when the client server sends a message
New WebSocket connection: 127.0.0.1:61994
Received a text message: Hello WebSocket Server#️⃣

```

2.Client (Server) sends a message <font color="red">in a separate, new terminal</font><br>
※After starting NixOS:<br>
```
cd websocket-chat
cargo run -p websocket-chat-client

# Output
Running `target/debug/websocket-chat-client`
WebSocket handshake has been successfully completed
Received a text message: Hello WebSocket Server#️⃣

```

## Build & Binary Process File
TODO
