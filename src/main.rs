use clap::Parser;
use rand::distributions::{Distribution, Uniform};
use rand_chacha::ChaCha20Rng;
use rand_core::{OsRng, SeedableRng};
use once_cell::sync::Lazy;

static CHARS_ALPHA_NUM: Lazy<Vec<char>> = Lazy::new(|| {
    let mut chars = Vec::new();
    // Digits
    chars.extend('0'..='9');
    // Lowercase letters
    chars.extend('a'..='z');
    // Uppercase letters
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
    /// Number of characters, [max: 512]:
    #[arg(short, long, default_value_t = 36)]
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

fn get_secure_rng() -> ChaCha20Rng {
    // Seed a ChaCha20Rng from the system's secure random number generator
    ChaCha20Rng::from_rng(&mut OsRng).expect("Failed to initialize secure RNG")
}


fn generate_password(length: u16, include_symbols: bool, include_extended: bool, allow_space: bool) -> String {
    let mut rng = get_secure_rng();

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

    let char_distribution = Uniform::from(0..chars.len());
    let mut password = String::with_capacity(length as usize);

    for _ in 0..length {
        let index = char_distribution.sample(&mut rng);
        password.push(chars[index]);
    }

    password
}

fn main() {
    let args = Args::parse();
    let limit: u16 = 512;
    let length = args.length.min(limit);

    if args.length > limit {
        eprintln!("Maximum length is {}", limit);
        eprintln!("Setting length to {} characters.", limit);
    }

    // Extended symbols can only be included if regular symbols are included
    let use_extended = args.extended_symbols && !args.exclude_symbols;
    if args.extended_symbols && args.exclude_symbols {
        eprintln!("Warning: Extended symbols cannot be included when symbols are excluded");
    }

    let password = generate_password(length, !args.exclude_symbols, use_extended, args.allow_space);
    println!("{}", password);
}
