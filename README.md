# Rust Secret Cipher CLI

A simple command-line tool written in Rust for encrypting and decrypting messages and files using a Caesar cipher.

## Features
- Encrypt or decrypt text messages.
- Encrypt or decrypt files.
- Preserve case and punctuation.
- Brute-force decryption for unknown shift values.
- Handles custom shift values.

## Installation
1. Clone the repository:
   ```bash
   https://github.com/YarRoh/rust_secret_cipher_cli.git
   cd rust_secret_cipher_cli
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the CLI tool:
   ```bash
   cargo run -- [OPTIONS]
   ```

## Usage
### Encrypt a Message
Encrypt a simple message with a custom shift value:
```bash
cargo run -- --encrypt --message "hello world" --shift 5
```

### Decrypt a Message
Decrypt a message with a known shift value:
```bash
cargo run -- --decrypt --message "mjqqt btwqi" --shift 5
```

### Brute-Force Decrypt
Attempt to decrypt a message with an unknown shift value:
```bash
cargo run -- --brute-force --message "mjqqt btwqi"
```

### Encrypt a File
Encrypt the contents of a file and save the result to another file:
```bash
cargo run -- --encrypt --input-file input.txt --output output.txt --shift 5
```

### Decrypt a File
Decrypt the contents of a file:
```bash
cargo run -- --decrypt --input-file encrypted.txt --output decrypted.txt --shift 5
```

## CLI Options
- `--encrypt`: Encrypt the input (message or file).
- `--decrypt`: Decrypt the input (message or file).
- `--brute-force`: Brute-force decrypt an encrypted message.
- `--message`: Specify the message to encrypt/decrypt.
- `--input-file`: Specify the input file for encryption/decryption.
- `--output`: Specify the output file for the result.
- `--shift`: Set the shift value for the Caesar cipher (default is 3).

## Examples
### Encrypt Example
```bash
cargo run -- --encrypt --message "Hello, World!" --shift 3
Output: Khoor, Zruog!
```

### File Encryption Example
```bash
cargo run -- --encrypt --input-file input.txt --output output.txt --shift 4
```

## Testing
Run the included tests to verify functionality:
```bash
cargo test
```

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

---

Happy coding!
