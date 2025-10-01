# rust-file-encryption

# Rust File Encryption Tool

A command-line tool written in Rust for secure file encryption and decryption using **AES-256 for content** and **RSA for key encryption**.

## Tech Stack
- Rust
- AES-256 / RSA hybrid encryption
- rand, base64 crates

## Features
- Encrypt/decrypt files securely
- AES key is encrypted using RSA public key
- CLI interface: `encrypt <input> <output>` and `decrypt <input> <output>`

## Usage
1. Clone the repo  
2. Build with `cargo build --release`  
3. Encrypt a file:  
```bash
./target/release/rust-file-encryption encrypt example.txt encrypted.dat
