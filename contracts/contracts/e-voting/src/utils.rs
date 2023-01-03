use ecies::{decrypt, encrypt, PublicKey, SecretKey};

pub fn encrypt_message_to_hex(public_key: PublicKey, message: String) -> String {
    let encrypted_message = encrypt(&public_key.serialize(), message.as_bytes()).unwrap();
    hex::encode(encrypted_message)
}

pub fn decrypt_message_from_hex(secret_key: SecretKey, encrypted_message_hex: String) -> String {
    let encrypted_message_bytes = hex::decode(encrypted_message_hex).unwrap();
    String::from_utf8(decrypt(&secret_key.serialize(), encrypted_message_bytes.as_slice()).unwrap()).unwrap()
}
