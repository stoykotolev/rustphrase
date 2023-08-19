use clap::Parser;
use passwords::PasswordGenerator;

#[derive(Debug, Parser)]
#[command(
    author = "Stoyko Tolev",
    version = "0.1.0",
    about = "A quick and easy way to generate passwords through the terminal"
)]
struct PasswordGeneratorArguments {
    /// Length of the password
    #[arg(long, default_value_t = 16)]
    length: usize,
    /// Should the password contain lowercase letters. Default value is true
    #[arg(short, long, default_value_t = true)]
    lowercase_letters: bool,
    /// Should the password contain any numbers.
    #[arg(short, long)]
    numbers: bool,
    /// Should the password contain uppercase letters.
    #[arg(short, long)]
    uppercase_letters: bool,
    /// Should the password contain symbols.
    #[arg(short, long)]
    symbols: bool,
    /// Should the password not contain any repeated characters.
    #[arg(short, long)]
    exclude_similar_characters: bool,
}

fn main() {
    let args: PasswordGeneratorArguments = PasswordGeneratorArguments::parse();

    let pg = PasswordGenerator {
        strict: true,
        spaces: false,
        length: args.length,
        lowercase_letters: args.lowercase_letters,
        numbers: args.numbers,
        uppercase_letters: args.uppercase_letters,
        symbols: args.symbols,
        exclude_similar_characters: args.exclude_similar_characters,
    };

    println!("Args: {:?}", args);

    println!(
        "Password: {}",
        pg.generate_one().expect("A password to be generated.")
    );
}
