use lopdf::{Document, Object, StringFormat};

fn main() {
    // 1. Load an existing PDF
    let mut doc = Document::load("original.pdf").unwrap();

    // 2. Define your secret message
    let secret_key = "SecretData";
    let secret_value = "Hidden message inside PDF metadata";

    // 3. Inject the secret into the Document Information Dictionary
    if let Ok(info_id) = doc.get_or_create_info_id() {
        if let Ok(info) = doc.get_object_mut(info_id) {
            if let Object::Dictionary(ref mut dict) = info {
                dict.set(secret_key, Object::String(secret_value.into(), StringFormat::Literal));
            }
        }
    }

    // 4. Save the modified PDF
    doc.save("hidden_metadata.pdf").unwrap();
    
    println!("Secret hidden in PDF metadata!");
}
