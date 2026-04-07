extern crate steganography;

use steganography::encoder::*;
use steganography::util::*;

fn main() {
    // 1. Define your secret message
    let message = "This is a secret message hidden in Rust!".to_string();
    
    // 2. Convert the message to a byte payload
    let payload = str_to_bytes(&message);
    
    // 3. Load the carrier image (must support an alpha channel, like PNG)
    let destination_image = file_as_dynamic_image("carrier.png".to_string());
    
    // 4. Create the encoder and hide the payload in the alpha channel
    let enc = Encoder::new(payload, destination_image);
    let result = enc.encode_alpha();
    
    // 5. Save the new image containing the hidden data
    save_image_buffer(result, "stego_image.png".to_string());
    
    println!("Message successfully hidden in stego_image.png");
}
