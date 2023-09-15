use clap::Parser;
use rand::Rng;

/// Generates a random string of fixed length made up of the full range of alphanumeric characters and symbols acceptable for use in password strings.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of characters, [max: 512]:
    #[arg(short, long, default_value_t = 36)]
    length: u16,
}

fn get_random_char() -> char {
    let characters = String::from("1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM!@#$%^&*");
    let mut rng = rand::thread_rng();
    let i: u8 = rng.gen();
    let i = i % characters.len() as u8;

    return characters.chars().nth(i as usize).unwrap()
}

fn main() {
    let args = Args::parse();
    let limit: u16 = 512;
    let mut length: u16 = args.length;


    if length > limit {
        println!("Maximum length is {}", limit);
        println!("Setting length to 512 characters.");
        length = limit;
    }

    let mut password = String::with_capacity(length as usize);

    for _i in 0..length {
        password.push(get_random_char());
    }

    println!("{}", password);
}
