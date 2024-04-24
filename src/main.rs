use ethnum::U256;
use k256::ecdsa::{SigningKey, VerifyingKey};
use sha2::Sha256;
use sha2::Digest;
use ripemd::Ripemd160;
use anyhow::Result;
use hex;
use base58::ToBase58;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[macro_use]
extern crate arrayref;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 || args[2] != "-f" {
        println!("Usage: {} <-ac | -au | -wc | -wu> -f <filename>", args[0]);
        return Ok(());
    }

    let filename = &args[3];

    // Read private keys from file
    let private_keys_hex = read_private_keys_from_file(filename)?;

    for key_hex in private_keys_hex {
        let key_bytes = hex::decode(&key_hex)?;
        let private_key = U256::from_be_bytes(*array_ref!(key_bytes, 0, 32));

        let (public_key_compressed, public_key_uncompressed, wif_compressed, wif_uncompressed) = generate_public_key(private_key);

        // Generate Bitcoin addresses
        let bitcoin_address_compressed = generate_bitcoin_address(&public_key_compressed);
        let bitcoin_address_uncompressed = generate_bitcoin_address(&public_key_uncompressed);

        match args[1].as_str() {
            "-ac" => println!("{}", bitcoin_address_compressed),
	    "-au" => println!("{}", bitcoin_address_uncompressed),
	    "-wc" => println!("{}", wif_compressed),
            "-wu" => println!("{}", wif_uncompressed),
            _ => { println!("Usage: {} <-ac | -au | -wc | -wu> -f <filename>", args[0]);
            return Ok(()); }
        }
    }

    Ok(())
}

fn read_private_keys_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

fn generate_bitcoin_address(public_key: &[u8]) -> String {
    let sha256_hash = sha256(public_key);
    let ripemd160_hash = ripemd160(&sha256_hash);
    base58check_encode(&ripemd160_hash)
}

fn generate_public_key(private_key: U256) -> (Vec<u8>, Vec<u8>, String, String) {
    // Convert U256 private key to bytes
    let private_key_bytes = u256_to_bytes_be(private_key);
    
    // Flip-flop the first and second 32 bits (16 bytes)
    // This is a bug on how the vec gets converted to U256, the first and second 32 bits get flip-flopped for whatever reason
    let mut modified_private_key_bytes = [0u8; 32];
    modified_private_key_bytes[..16].copy_from_slice(&private_key_bytes[16..32]);
    modified_private_key_bytes[16..].copy_from_slice(&private_key_bytes[..16]);

    // Create a SigningKey (private key) from the modified bytes
    let signing_key = match SigningKey::from_bytes((&modified_private_key_bytes).into()) {
        Ok(key) => key,
        Err(_) => return (vec![], vec![], String::new(), String::new()), // Return empty vectors if the key is invalid
    };

    // Obtain the VerifyingKey (public key) associated with the SigningKey
    let verifying_key = VerifyingKey::from(&signing_key);

    // Generate both compressed and uncompressed public keys
    let public_key_compressed = verifying_key.to_encoded_point(true).as_bytes().to_vec();
    let public_key_uncompressed = verifying_key.to_encoded_point(false).as_bytes().to_vec();

    // Generate WIF for both compressed and uncompressed keys
    let wif_compressed = to_wif(&private_key_bytes, true);
    let wif_uncompressed = to_wif(&private_key_bytes, false);

    (public_key_compressed, public_key_uncompressed, wif_compressed, wif_uncompressed)
}

fn to_wif(private_key_bytes: &[u8], compressed: bool) -> String {
    let mut extended_key = vec![0x80]; // Prefix for mainnet
    extended_key.extend_from_slice(private_key_bytes);
    if compressed {
        extended_key.push(0x01); // Compression flag
    }

    // Double SHA-256 for checksum
    let checksum = Sha256::digest(&Sha256::digest(&extended_key));
    extended_key.extend_from_slice(&checksum[0..4]);

    // Base58Check encoding
    extended_key.to_base58()
}

// Convert U256 to a 32-byte array in big-endian format
fn u256_to_bytes_be(value: U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let u128_parts = value.0; // Extract the [u128; 2] array

    // Process each u128 part
    for (i, &part) in u128_parts.iter().enumerate() {
        // Convert each u128 to big-endian bytes
        let part_bytes = part.to_be_bytes();

        // Determine the starting index for copying bytes into the final array
        let start_index = i * 16; // Each u128 has 16 bytes

        // Copy bytes into the appropriate position of the final array
        bytes[start_index..start_index + 16].copy_from_slice(&part_bytes);
    }

    bytes
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn ripemd160(data: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn base58check_encode(data: &[u8]) -> String {
    // Step 1: Prepare the data by adding a prefix (0x00 for Bitcoin mainnet)
    let mut payload = vec![0x00];
    payload.extend_from_slice(data);

    // Step 2: Calculate the double SHA-256 hash of the data
    let checksum = sha256(&sha256(&payload));

    // Step 3: Append the first 4 bytes of the checksum to the payload
    payload.extend_from_slice(&checksum[..4]);

    // Step 4: Encode the payload with Base58
    payload.to_base58()
}
