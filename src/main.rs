use rust_secret_cipher_cli::{decrypt, encrypt, brute_force_decrypt};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Args {
    #[arg(short, long)]
    encrypt: bool,
    
    #[arg(short, long)]
    decrypt: bool,
    
    #[arg(long)]
    brute_force: bool,
    
    #[arg(short, long,conflicts_with = "input_file")]
    message: Option<String>,
    
    #[arg(short, long, aliases = ["input-file", "input" ], conflicts_with = "message")]
    input_file: Option<String>,
    
    #[arg(short,long)]
    output : Option<String>,
    
    #[arg(short, long, default_value = "3")]
    shift: u8,
}


fn main() {
    let args = Args::parse();

    let input = if let Some(message) = &args.message {
        message.clone()
    } else if let Some(input_file) = &args.input_file {
        fs::read_to_string(input_file).expect("Failed to read input file")
    } else {
        eprintln!("Error: You must specify either a message or an input file");
        std::process::exit(1);
    };

    let result = if args.brute_force {
        let possibilities = brute_force_decrypt(&input);
        possibilities.join("\n")
    } else if args.encrypt {
        encrypt(&input, args.shift)
    } else if args.decrypt {
        decrypt(&input, args.shift)
    } else {
        eprintln!("Error: You must specify either --encrypt, --decrypt, or --brute-force");
        std::process::exit(1);
    };

    if let Some(output_file) = &args.output {
        fs::write(output_file, &result).expect("Failed to write output file");
    } else {
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("hello", 3), "khoor");
        assert_eq!(encrypt("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("khoor", 3), "hello");
        assert_eq!(decrypt("Khoor, Zruog!", 3), "Hello, World!");
    }

    #[test]
    fn test_brute_force_decrypt() {
        let results = brute_force_decrypt("khoor");
        assert!(results.contains(&"hello".to_string()));
    }
}