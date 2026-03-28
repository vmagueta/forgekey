use clap::Parser;
use colored::Colorize;
use rand::RngExt;

/// 🔑 forgekey — A fast, minimal password generator.
#[derive(Parser)]
#[command(name = "forgekey", version, about)]
struct Cli {
    /// Length of the generated password.
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Number of passwords to generate.
    #[arg(short, long, default_value_t = 1)]
    number: usize,

    /// Exclude symbols from the password.
    #[arg(long, default_value_t = false)]
    no_symbols: bool,

    /// Exclude numbers from the password.
    #[arg(long, default_value_t = false)]
    no_numbers: bool,

    /// Exclude uppercase letters from the password.
    #[arg(long, default_value_t = false)]
    no_uppercase: bool,
}

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

/// Generates a single password based on the provided CLI options.
///
/// # Errors
///
/// Returns an error if the resulting charset is empty or the length is zero.
fn generate_password(cli: &Cli) -> Result<String, String> {
    if cli.length == 0 {
        return Err("Password length must be greater than zero.".to_string());
    }

    let mut charset = String::new();

    charset.push_str(LOWERCASE);

    if !cli.no_uppercase {
        charset.push_str(UPPERCASE);
    }
    if !cli.no_numbers {
        charset.push_str(NUMBERS);
    }
    if !cli.no_symbols {
        charset.push_str(SYMBOLS);
    }

    if charset.is_empty() {
        return Err("At least one character set must be enabled.".to_string());
    }

    let charset_bytes = charset.as_bytes();
    let mut rng = rand::rng();

    let password = (0..cli.length)
        .map(|_| {
            let idx = rng.random_range(0..charset_bytes.len());
            charset_bytes[idx] as char
        })
        .collect();

    Ok(password)
}

/// Prints a password with each character colored by its type.
fn print_colored(password: &str) {
    for ch in password.chars() {
        let colored_char = match ch {
            'A'..='Z' => ch.to_string().green(),
            '0'..='9' => ch.to_string().cyan(),
            'a'..='z' => ch.to_string().white(),
            _ => ch.to_string().yellow(),
        };
        print!("{colored_char}");
    }
    println!();
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.number {
        match generate_password(&cli) {
            Ok(password) => print_colored(&password),
            Err(error) => {
                eprintln!("{}", error.red().bold());
                std::process::exit(1);
            }
        }
    }
}
