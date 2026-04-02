use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
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
    let mut passwords = Vec::new();

    for _ in 0..cli.number {
        match generate_password(&cli) {
            Ok(password) => {
                print_colored(&password);
                passwords.push(password);
            }
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }
    if cli.copy {
        match ClipboardProvider::new() {
            Ok(mut ctx) => {
                match ClipboardContext::set_contents(&mut ctx, passwords.join("\n")) {
                    Ok(_) => println!("Copied to clipboard!"),
                    Err(e) => eprintln!("Failed to copy to clipboard: {e}"),
                }
            }
            Err(e) => eprintln!("Clipboard unavailable: {e}"),
        }
    }
}
