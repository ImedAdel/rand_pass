use base64::{decode, encode};
use dotenv;
use rand::Rng;

fn main() {
    let key = "FOO";
    let value = dotenv::var(key).unwrap(); // Loads env variable. Not used in this code though.
    let private_key = rand::thread_rng().gen::<[u8; 32]>(); // generates 32 bytes array
    let encoded = encode(&private_key); // converts array to base64
    let dec = decode(encoded.clone()).unwrap(); // converts base64 to vector.
    let decoded = {
        let mut array = [0u8; 32];
        for (&x, p) in dec.as_slice().iter().zip(array.iter_mut()) {
            *p = x;
        }
        array
    }; // converts vector to 32 bytes array. pretty sure there is an easier way of doing it. but this already works.
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
