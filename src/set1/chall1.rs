use base64::{engine::general_purpose, Engine as _};
use hex;

pub fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let as_bytes = hex::decode(input).unwrap();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(as_bytes);

    assert_eq!(
        encoded,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned()
    );
}
