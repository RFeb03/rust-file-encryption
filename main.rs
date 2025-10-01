mod crypto;
use crypto::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <encrypt|decrypt> <input_file> <output_file>", args[0]);
        return;
    }

    let mode = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let data = fs::read(input_file).expect("Failed to read input file");

    // Generate RSA keys for demo (in practice, store/load keys)
    let (private_key, public_key) = generate_rsa_keys();

    // AES key (32 bytes)
    let aes_key: [u8;32] = rand::random();

    match mode.as_str() {
        "encrypt" => {
            let encrypted = encrypt_aes(&aes_key, &data);
            let encrypted_key = encrypt_rsa(&public_key, &aes_key);
            fs::write(output_file, encrypted).expect("Failed to write output file");
            fs::write("aes_key.enc", encrypted_key).expect("Failed to write encrypted AES key");
            println!("File encrypted successfully!");
        },
        "decrypt" => {
            let encrypted_key = fs::read("aes_key.enc").expect("Failed to read AES key");
            let aes_key = decrypt_rsa(&private_key, &encrypted_key);
            let decrypted = decrypt_aes(&aes_key.try_into().unwrap(), &data);
            fs::write(output_file, decrypted).expect("Failed to write output file");
            println!("File decrypted successfully!");
        },
        _ => println!("Invalid mode. Use encrypt or decrypt."),
    }
}
