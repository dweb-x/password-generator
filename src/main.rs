use clap::Parser;
use rand::distributions::{Distribution, Uniform};
use rand_chacha::ChaCha20Rng;
use rand_core::{OsRng, SeedableRng};
use once_cell::sync::Lazy;
use std::process;

static CHARS_ALPHA_NUM: Lazy<Vec<char>> = Lazy::new(|| {
    let mut chars = Vec::new();
    chars.extend('0'..='9');
    chars.extend('a'..='z');
    chars.extend('A'..='Z');
    chars
});

static CHARS_SYMBOLS: Lazy<Vec<char>> = Lazy::new(|| {
    // Special characters (carefully chosen set)
    "!@#$%^&*()-_=+[]{}|;:,.<>?".chars().collect()
});

static CHARS_SYMBOLS_EXTENDED: Lazy<Vec<char>> = Lazy::new(|| {
    // AWS valid but potentially problematic
    "`\"'/\\".chars().collect()
});

#[derive(Parser, Debug)]
#[command(author, version, about = "Cryptographically secure password generator")]
struct Args {
    /// Password length (between 1 and 512 characters)
    #[arg(short, long, default_value_t = 36)]
    #[arg(value_parser = clap::builder::ValueParser::new(validate_length))]
    length: u16,

    /// Exclude symbols from the password (include alphanumeric only)
    #[arg(short = 'n', long = "no-symbols", default_value_t = false)]
    exclude_symbols: bool,

    /// Include extended symbols set (`\"'/\)
    #[arg(short = 'e', long = "extended-symbols", default_value_t = false)]
    extended_symbols: bool,

    /// Allow space character in password
    #[arg(short = 's', long = "allow-space", default_value_t = false)]
    allow_space: bool,
}

#[derive(Debug)]
enum PasswordError {
    InvalidSymbolCombination,
    EmptyCharacterSet,
    RngInitializationError,
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PasswordError::InvalidSymbolCombination => 
                write!(f, "Cannot use extended symbols (-e) when symbols are excluded (-n)"),
            PasswordError::EmptyCharacterSet => 
                write!(f, "No character sets available for password generation"),
            PasswordError::RngInitializationError => 
                write!(f, "Failed to initialize secure random number generator"),
        }
    }
}

fn validate_length(s: &str) -> Result<u16, String> {
    let length: u16 = s.parse().map_err(|_| format!(
        "The length must be a positive number between 1 and 512. Got: {}", s
    ))?;

    if length < 1 || length > 512 {
        return Err(format!(
            "Password length must be between 1 and 512 characters. Got: {}", length
        ));
    }

    Ok(length)
}

fn validate_args(args: &Args) -> Result<(), PasswordError> {
    // Check for invalid combination of extended symbols without regular symbols
    if args.extended_symbols && args.exclude_symbols {
        return Err(PasswordError::InvalidSymbolCombination);
    }

    // Check if we would have an empty character set
    let has_any_chars = !args.exclude_symbols || args.allow_space;
    if !has_any_chars && CHARS_ALPHA_NUM.is_empty() {
        return Err(PasswordError::EmptyCharacterSet);
    }

    Ok(())
}

fn get_secure_rng() -> Result<ChaCha20Rng, PasswordError> {
    ChaCha20Rng::from_rng(&mut OsRng)
        .map_err(|_| PasswordError::RngInitializationError)
}

fn generate_password(
    length: u16,
    include_symbols: bool,
    include_extended: bool,
    allow_space: bool
) -> Result<String, PasswordError> {
    let mut rng = get_secure_rng()?;
    let mut chars = CHARS_ALPHA_NUM.clone();

    if include_symbols {
        chars.extend(CHARS_SYMBOLS.iter());
        if include_extended {
            chars.extend(CHARS_SYMBOLS_EXTENDED.iter());
        }
    }

    if allow_space {
        chars.push(' ');
    }

    if chars.is_empty() {
        return Err(PasswordError::EmptyCharacterSet);
    }

    let char_distribution = Uniform::from(0..chars.len());
    let mut password = String::with_capacity(length as usize);

    for _ in 0..length {
        let index = char_distribution.sample(&mut rng);
        password.push(chars[index]);
    }

    Ok(password)
}

fn main() {
    let args = Args::parse();

    // Validate arguments
    if let Err(err) = validate_args(&args) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    let use_extended = args.extended_symbols && !args.exclude_symbols;
    
    match generate_password(args.length, !args.exclude_symbols, use_extended, args.allow_space) {
        Ok(password) => println!("{}", password),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_args() {
        let args = Args {
            length: 36,
            exclude_symbols: false,
            extended_symbols: false,
            allow_space: false,
        };
        assert!(validate_args(&args).is_ok());
    }

    #[test]
    fn test_invalid_extended_symbols() {
        let args = Args {
            length: 36,
            exclude_symbols: true,
            extended_symbols: true,
            allow_space: false,
        };
        assert!(matches!(
            validate_args(&args),
            Err(PasswordError::InvalidSymbolCombination)
        ));
    }

    #[test]
    fn test_password_length() {
        let password = generate_password(123, true, false, false).unwrap();
        assert_eq!(password.len(), 123);
    }

    #[test]
    fn test_no_symbols() {
        let password = generate_password(100, false, false, false).unwrap();
        assert!(password.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_with_spaces() {
        let password = generate_password(100, true, false, true).unwrap();
        assert!(password.chars().any(|c| c == ' '));
    }

    #[test]
    fn test_with_extended_symbols() {
        let password = generate_password(100, true, true, false).unwrap();
        // Check if at least one extended symbol is present
        assert!(password.chars().any(|c| CHARS_SYMBOLS_EXTENDED.contains(&c)));
    }

    #[test]
    fn test_validate_length_input() {
        assert!(validate_length("1").is_ok());
        assert!(validate_length("512").is_ok());
        assert!(validate_length("0").is_err());
        assert!(validate_length("513").is_err());
        assert!(validate_length("abc").is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Test minimum length
        let min_password = generate_password(1, true, false, false).unwrap();
        assert_eq!(min_password.len(), 1);

        // Test maximum length
        let max_password = generate_password(512, true, false, false).unwrap();
        assert_eq!(max_password.len(), 512);
    }

    #[test]
    fn test_password_uniqueness() {
        let pass1 = generate_password(36, true, false, false).unwrap();
        let pass2 = generate_password(36, true, false, false).unwrap();
        assert_ne!(pass1, pass2, "Passwords should be unique");
    }

    #[test]
    fn test_all_character_sets() {
        let password = generate_password(1000, true, true, true).unwrap();

        // Test the presence of each character set
        assert!(password.chars().any(|c| c.is_ascii_lowercase()), "Missing lowercase letters");
        assert!(password.chars().any(|c| c.is_ascii_uppercase()), "Missing uppercase letters");
        assert!(password.chars().any(|c| c.is_ascii_digit()), "Missing numbers");
        assert!(password.chars().any(|c| CHARS_SYMBOLS.contains(&c)), "Missing symbols");
        assert!(password.chars().any(|c| CHARS_SYMBOLS_EXTENDED.contains(&c)), "Missing extended symbols");
        assert!(password.chars().any(|c| c == ' '), "Missing space");
    }

    #[test]
    fn test_character_distribution() {
        let password = generate_password(10000, true, true, true).unwrap();
        let char_counts: std::collections::HashMap<char, usize> =
            password.chars().fold(std::collections::HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        // Check that each character type appears at least once
        assert!(char_counts.keys().any(|c| c.is_ascii_lowercase()));
        assert!(char_counts.keys().any(|c| c.is_ascii_uppercase()));
        assert!(char_counts.keys().any(|c| c.is_ascii_digit()));
    }

}