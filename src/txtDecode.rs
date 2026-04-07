use whitespace_text_steganography::{reveal};

fn main() {
    let output_path = "./hidden.txt";

    // 1. Extract the hidden message from the file
    let revealed_message = reveal(output_path);

    println!("The revealed message is: {}", revealed_message);
}
