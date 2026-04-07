use stegano_rs::media::audio::Wav;
use stegano_rs::utils::Payload;

fn main() {
    // 1. Load your carrier audio file
    let mut wav = Wav::open("carrier.wav").expect("Failed to open WAV file");
    
    // 2. Prepare the secret message
    let secret = "Top secret audio message!".as_bytes();
    let payload = Payload::from_bytes(secret);

    // 3. Hide the message in the audio samples
    wav.hide(&payload).expect("Failed to hide message");

    // 4. Save the new "stego" audio file
    wav.save("secret_audio.wav").expect("Failed to save WAV file");
    
    println!("Message hidden in secret_audio.wav");
}
