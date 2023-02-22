#[cfg(test)]
mod test_module {
    // use ecies::{decrypt, encrypt};
    // use ecies::utils::generate_keypair;
    // use std::string::String;
    // use crate::state::VoteKind;
    //
    // #[test]
    // fn test() {
    //     let (secret_key, public_key) = generate_keypair();
    //
    //     const MSG: &str = "123123123145";
    //     println!("{:?}", secret_key.serialize());
    //     println!("{:?}", public_key.serialize());
    //
    //     let encrypted_message = encrypt(&public_key.serialize(), MSG.as_bytes()).unwrap();
    //     println!("{:?}", encrypted_message);
    //     let encrypted_message_hex = hex::encode(encrypted_message);
    //     println!("{}", encrypted_message_hex);
    //
    //     let encrypted_message_bytes = hex::decode(encrypted_message_hex).unwrap();
    //
    //     println!("{}", String::from_utf8(decrypt(&secret_key.serialize(), encrypted_message_bytes.as_slice()).unwrap()).unwrap());
    //
    //     println!("{}", VoteKind::UpVote);
    // }
}