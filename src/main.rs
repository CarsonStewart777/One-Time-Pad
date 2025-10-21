use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        println!("\n=== XOR Encryption Tool ===");
        println!("1. Encrypt a Message");
        println!("2. Decrypt a Message");
        println!("3. Generate Random Key");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush()?; // flush prompt before input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => encrypt_flow()?,
            "2" => decrypt_flow()?,
            "3" => generate_key()?,
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}

fn encrypt_flow() -> io::Result<()> {
    let message = prompt("Enter your plaintext message: ")?;
    let key_path = "key.txt";
    let plaintext_path = "plaintext.txt";
    let cipher_path = "ciphertext.bin";

    // save plaintext to file
    fs::write(plaintext_path, &message)?;

    // check if key file exists
    if !std::path::Path::new(key_path).exists() {
        println!("No key file found. Generating a new one automatically.");
        generate_key()?;
    }

    let p_bytes = fs::read(plaintext_path)?;
    let k_bytes = fs::read(key_path)?;

    if k_bytes.len() < p_bytes.len() {
        eprintln!("Error: key must be at least as long as the message.");
        return Ok(());
    }

    let xor_result = xor_vectors(&p_bytes, &k_bytes);
    fs::write(cipher_path, xor_result)?;

    println!("Message encrypted successfully.");
    println!("Saved as:");
    println!("  Plaintext -> {}", plaintext_path);
    println!("  Ciphertext -> {}", cipher_path);
    println!("  Key -> {}", key_path);

    Ok(())
}

fn decrypt_flow() -> io::Result<()> {
    let key_path = "key.txt";
    let cipher_path = "ciphertext.bin";
    let output_path = "decrypted.txt";

    if !std::path::Path::new(cipher_path).exists() {
        eprintln!("Ciphertext file not found. Please encrypt something first.");
        return Ok(());
    }

    if !std::path::Path::new(key_path).exists() {
        eprintln!("Key file not found. Please generate or encrypt to create one.");
        return Ok(());
    }

    let cipher_bytes = fs::read(cipher_path)?;
    let k_bytes = fs::read(key_path)?;

    if k_bytes.len() < cipher_bytes.len() {
        eprintln!("Error: key must be at least as long as ciphertext.");
        return Ok(());
    }

    let decrypted_bytes = xor_vectors(&cipher_bytes, &k_bytes);
    fs::write(output_path, &decrypted_bytes)?;

    // Try to interpret the decrypted bytes as UTF-8 text
    match String::from_utf8(decrypted_bytes) {
        Ok(text) => {
            println!("\nDecrypted message: {}", text);
        }
        Err(_) => {
            println!("Decryption complete. Output written to {}", output_path);
        }
    }

    Ok(())
}

fn generate_key() -> io::Result<()> {
    use rand::Rng;

    let key_len_str = prompt("Enter desired key length in bytes (or press Enter for 256): ")?;
    let key_len: usize = if key_len_str.trim().is_empty() {
        256
    } else {
        key_len_str.trim().parse().unwrap_or(256)
    };

    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..key_len).map(|_| rng.r#gen::<u8>()).collect();

    let key_path = "key.txt";
    fs::write(key_path, key)?;

    println!("Key generated and saved to {}", key_path);
    Ok(())
}

/// XOR two byte slices (assumes k_bytes >= p_bytes)
fn xor_vectors(p_bytes: &[u8], k_bytes: &[u8]) -> Vec<u8> {
    p_bytes.iter().zip(k_bytes.iter()).map(|(&a, &b)| a ^ b).collect()
}

/// Prompt helper
fn prompt(message: &str) -> io::Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

