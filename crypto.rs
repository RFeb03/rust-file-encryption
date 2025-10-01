use aes::Aes256;
use aes::cipher::{KeyInit, BlockEncrypt, BlockDecrypt, generic_array::GenericArray};
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use base64::{encode, decode};
use std::fs;

pub fn generate_rsa_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut OsRng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt_aes(key: &[u8;32], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut block = GenericArray::clone_from_slice(&plaintext[0..16.min(plaintext.len())]);
    cipher.encrypt_block(&mut block);
    block.to_vec()
}

pub fn decrypt_aes(key: &[u8;32], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut block = GenericArray::clone_from_slice(&ciphertext[0..16.min(ciphertext.len())]);
    cipher.decrypt_block(&mut block);
    block.to_vec()
}

pub fn encrypt_rsa(public_key: &RsaPublicKey, data: &[u8]) -> Vec<u8> {
    public_key.encrypt(&mut OsRng, PaddingScheme::new_pkcs1v15_encrypt(), data).expect("RSA encryption failed")
}

pub fn decrypt_rsa(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> Vec<u8> {
    private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted_data).expect("RSA decryption failed")
}
