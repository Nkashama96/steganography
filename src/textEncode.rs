extern crate whitespace_text_steganography;

use std::fs::File;
use std::io::prelude::*;
use whitespace_text_steganography::{hide};

fn main() {
    // Paths to input files
    let payload_path = "./texts/payload.txt"; // The secret message file
    let carrier_path = "./texts/carrier.txt"; // The innocent cover text file
    let output_path = "./hidden.txt";

    // 1. Generate the hidden text
    let hidden_text = hide(payload_path, carrier_path);

    // 2. Save the result to a new file
    let mut file = File::create(output_path).expect("Could not create output file");
    file.write_all(hidden_text.as_bytes()).expect("Could not write to file");

    println!("Secret successfully hidden in {}", output_path);
}
