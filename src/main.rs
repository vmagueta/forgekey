use clap::Parser;
use colored::Colorize;
use forgekey::{Cli, generate_password};

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
            Ok(password) => {
                if cli.copy {
                    use arboard::Clipboard;

                    if let Ok(mut clipboard) = Clipboard::new() {
                        let _ = clipboard.set_text(password.clone());
                        println!("Copied to clipboard!");
                    } else {
                        println!("{}", password);
                    }
                } else {
                    print_colored(&password);
                }
            }
            Err(error) => {
                eprintln!("{}", error.red().bold());
                std::process::exit(1);
            }
        }
    }
}
