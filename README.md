# Bitcoin Address Generator
This Rust project is designed to convert Bitcoin private keys into their corresponding Wallet Import Format (WIF) and Bitcoin addresses. It reads private keys from a file and generates both compressed and uncompressed Bitcoin addresses and WIFs for each key.

# Features
1. Reads hexadecimal private keys from an external file.
2. Converts private keys to both compressed and uncompressed WIF.
3. Generates both compressed and uncompressed Bitcoin addresses.
4. Utilizes robust cryptographic functions with a focus on accuracy and security.

# Prerequisites
Before running this project, ensure you have Rust installed on your machine. Follow the instructions at The Rust Programming Language to set it up.

# Installation
1. Clone the repository:
```
git clone https://github.com/TheSageMaster/Rust-Bitcoin-Address-from-Private-Key.git
```
2. Change into the project directory:
```
cd Rust-Bitcoin-Address-from-Private-Key
```

# Preparing the Private Keys
Create a file named `private_keys.txt` in the root of the project directory. Add your hexadecimal private keys to this file, placing each key on a new line.

Example format of `private_keys.txt`:
```
[private_key_1_in_hex]
[private_key_2_in_hex]
```

# Usage
To use the Bitcoin Address Generator, follow these steps:

1. Ensure `private_keys.txt` is populated with your private keys.
2. Compile and run the program:
```
cargo run
```

# Dependencies
This project relies on the following external crates:

`ethnum`: For handling large numbers.
`k256`: For elliptic curve cryptography specific to secp256k1.
`sha2` and `ripemd`: For cryptographic hash functions.
`base58`: For Base58Check encoding.

These dependencies are specified in the Cargo.toml file.

# Contributing
Contributions to the Bitcoin Address Generator are welcome. Please fork the repository, make your changes, and create a pull request.

# License
This project is licensed under the MIT License - see the LICENSE.md file for details.
