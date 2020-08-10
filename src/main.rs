use base64::{decode_config_slice, encode};
use dotenv;
use rand::Rng;

fn main() {
    let key = "FOO";
    let value = dotenv::var(key).unwrap(); // Loads env variable. Not used in this code though.

    let private_key = rand::thread_rng().gen::<[u8; 32]>(); // generates 32 bytes array
    let encoded = encode(&private_key); // converts array to base64
    let mut decoded = [0u8; 32]; // initializes an 32 bytes array
    decode_config_slice(encoded.clone(), base64::STANDARD, &mut decoded).unwrap(); // nb. encode is a shorthand for encode_config with base64::STANDARD (see docs)
    println!(
        "Hello, world!,
        Private Key:\t{:?},
        Length:\t{},
        FOO:\t{},
        Base64:\t{},
        Decoded:\t{:?},",
        private_key,
        private_key.len(),
        value,
        encoded,
        decoded,
    );
}
