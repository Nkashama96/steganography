use steganography::decoder::*;
use steganography::util::*;

fn main() {
    // 1. Load the image that contains the hidden message
    let encoded_image = file_as_image_buffer("stego_image.png".to_string());
    
    // 2. Initialize the decoder
    let dec = Decoder::new(encoded_image);
    
    // 3. Extract the bytes from the alpha channel
    let out_buffer = dec.decode_alpha();
    
    // 4. Filter out default values (255) and convert back to a string
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|&b| b != 0xff).collect();
    let message = bytes_to_str(&clean_buffer);
    
    println!("The hidden message is: {}", message);
}
