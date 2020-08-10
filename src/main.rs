use base64::{decode, encode};
use dotenv;
use rand::Rng;

fn main() {
    let key = "FOO";
    let value = dotenv::var(key).unwrap();
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    let encoded = encode(&private_key);
    let dec = decode(encoded.clone()).unwrap();
    let decoded = {
        let mut array = [0u8; 32];
        for (&x, p) in dec.as_slice().iter().zip(array.iter_mut()) {
            *p = x;
        }
        array
    };
    println!(
        "Hello, world!,
        Private Key:\t{:?},
        Length:\t{},
        FOO:\t{},
        Base64:\t{},
        Decoded:\t{:?}",
        private_key,
        private_key.len(),
        value,
        encoded,
        decoded
    );
}
