use std::fs;
use std::io;


fn main() -> io::Result<()> {

    let plaintext_path = "plaintext.txt";

    fs::write(plaintext_path, "This is some example content.")?;
    
    let keytext_path = "keytext.txt";

    fs::write(keytext_path, "Four ma sour jengrat something")?;

    

    let p_bytes = fs::read(plaintext_path)?;
    let k_bytes = fs::read(keytext_path)?;
    let xor_result = xor_vectors(&p_bytes, &k_bytes);

    println!("keytext_bytes {:?}", k_bytes);
    println!("\nplaintext_bytes {:?}", p_bytes);
    println!("\n XOR Result: {:?}", xor_result);

    println!("\n outputting ciphertext.");
    
    match encrypt(&xor_result) {
        Ok(()) => println!("file written successfully!"),
        Err(e) => eprintln!("Error writing file: {}", e)
    }

    let decrypted_result: Vec<u8> = decrypt(&xor_result, &k_bytes);

    println!("\n Testing Decryption.");
    
    if decrypted_result == p_bytes {
        println!("Decryption Succesful!");
    } else {
        println!("Decryption failed!")
    };
    
    




    if k_bytes.len() >= p_bytes.len() {
        println!(" ");
    } else {
        println!("invalid key. ")
    };

    Ok(())


}

fn xor_vectors(p_bytes: &Vec<u8>, k_bytes: &Vec<u8>) -> Vec<u8> {
    if k_bytes.len() < p_bytes.len() {
        panic!("Key Bytes must be >= Plain Bytes!")
    }

    p_bytes.iter()
        .zip(k_bytes.iter()) // Pair elements from both byte vectors 
        .map(|(&x1, &x2)| x1 ^ x2) // Apply XOR to pairs 
        .collect()
}

fn encrypt(xor_result: &Vec<u8>) -> io::Result<()> {
    let cipher_path = "ciphertext.bin";

    fs::write(cipher_path, xor_result)?;

    Ok(())


}

fn decrypt(xor_result: &Vec<u8>, k_bytes: &Vec<u8>) -> Vec<u8> {

    let plain_test = xor_result.iter()
        .zip(k_bytes.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
    plain_test


}

