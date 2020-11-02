use bacon_cipher::codecs::char_codec::CharCodec;
use bacon_cipher::BaconCodec;
use std::iter::FromIterator;

use std::io;

fn main() {
    let mut input = String::new();

    println!("Welcome to my bacon cypher presentation");

    let mut input = String::new();

    println!("Input the text you want to encode");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Define a Bacon Codec that encodes using the characters 'A' and 'B'
    let codec = CharCodec::new('A', 'B');
    // This is the secret to encode
    let secret: Vec<char> = input.chars().collect();
    // Get the encoded chars
    let encoded_chars = codec.encode(&secret);
    let encoded_string = String::from_iter(encoded_chars.iter());

    println!("{}", encoded_string);

    let mut input = String::new();

    println!("Input the text you want to decode");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Define a Bacon Codec that encodes using the characters 'A' and 'B'
    let codec = CharCodec::new('A', 'B');

    // These are the encoded characters
    let encoded_chars: Vec<char> = input.chars().collect();

    // Retrieve the decoded chars
    let decoded = codec.decode(&encoded_chars);
    let string = String::from_iter(decoded.iter());

    println!("{}", string);
}
