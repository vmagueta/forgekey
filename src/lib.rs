//! # forgekey
//!
//! A fast, minimal password generator library.
//!
//! This crate provides the core logic for generating secure random passwords
//! with configurable character sets and length.

use clap::Parser;
use rand::RngExt;

/// Command-line arguments for forgekey.
///
/// # Examples
/// ```
/// use forgekey::Cli;
///
/// let cli = Cli {
///     length: 24,
///     number: 3,
///     no_symbols: true,
///     no_numbers: false,
///     no_uppercase: false,
///     copy: false,
///     strength: false
/// };
/// ```
#[derive(Parser)]
#[command(name = "forgekey", version, about)]
pub struct Cli {
    /// Length of the generated password.
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    /// Number of passwords to generate.
    #[arg(short, long, default_value_t = 1)]
    pub number: usize,

    /// Exclude symbols from the password.
    #[arg(long, default_value_t = false)]
    pub no_symbols: bool,

    /// Exclude numbers from the password.
    #[arg(long, default_value_t = false)]
    pub no_numbers: bool,

    /// Exclude uppercase letters from the password.
    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    /// Copy generated password to clipboard
    #[arg(short, long)]
    pub copy: bool,

    /// Show password strength indicator
    #[arg(short, long)]
    pub strength: bool,
}

/// Characters: `a-z`
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

/// Characters: `A-Z`
pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Characters: `0-9`
pub const NUMBERS: &str = "0123456789";

/// Characters `!@#$%^&*()_+-=[]{}|;:,.<>?`
pub const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

/// Generates a single password based on the provided CLI options.
///
/// Builds a charset from the enabled character sets
///  (lowercase is always included),
/// then randomly selects characters from it using a cryptographically
///  secure RNG.
///
/// # Arguments
///
/// * `cli` - A reference to the parsed CLI options.
///
/// # Returns
///
/// * `Ok(String)` - The generated password.
/// * `Err(String)` - If `length` is zero.
///
/// # Examples
///
/// ```
/// use forgekey::{Cli, generate_password};
///
/// let cli = Cli {
///     length: 20,
///     number: 1,
///     no_symbols: false,
///     no_numbers: false,
///     no_uppercase: false,
///     copy: false,
///     strength: false
/// };
///
/// let (password, _) = generate_password(&cli).unwrap();
/// assert_eq!(password.len(), 20);
/// ```
pub fn generate_password(cli: &Cli) -> Result<(String, usize), String> {
    if cli.length == 0 {
        return Err("Password length must be greater than zero.".to_string());
    }

    let mut charset = String::new();

    // Lowercase is always included - there is no flag to disable it.
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

    let charset_bytes = charset.as_bytes();
    let mut rng = rand::rng();

    let password = (0..cli.length)
        .map(|_| {
            let idx = rng.random_range(0..charset_bytes.len());
            charset_bytes[idx] as char
        })
        .collect();

    Ok((password, charset_bytes.len()))
}

pub fn calculate_entropy(length: usize, charset_size: usize) -> (f64, &'static str) {
    let entropy = length as f64 * (charset_size as f64).log2();

    let result = match entropy as u32 {
        0..28 => "weak",
        28..36 => "fair",
        36..60 => "strong",
        60.. => "very strong",
    };

    (entropy, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a [`Cli`] instance with default values for use in tests.
    fn cli_default() -> Cli {
        Cli {
            length: 16,
            number: 1,
            no_symbols: false,
            no_numbers: false,
            no_uppercase: false,
            copy: false,
            strength: false,
        }
    }

    #[test]
    fn test_default_password_length() {
        let cli = cli_default();
        let (password, _) = generate_password(&cli).unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_custom_length() {
        let mut cli = cli_default();
        cli.length = 32;
        let (password, _) = generate_password(&cli).unwrap();
        assert_eq!(password.len(), 32);
    }

    #[test]
    fn test_zero_length_returns_error() {
        let mut cli = cli_default();
        cli.length = 0;
        let result = generate_password(&cli);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_symbols_excludes_symbols() {
        let mut cli = cli_default();
        cli.no_symbols = true;
        cli.length = 200;
        let (password, _) = generate_password(&cli).unwrap();
        assert!(!password.chars().any(|c| SYMBOLS.contains(c)));
    }

    #[test]
    fn test_no_numbers_excludes_numbers() {
        let mut cli = cli_default();
        cli.no_numbers = true;
        cli.length = 200;
        let (password, _) = generate_password(&cli).unwrap();
        assert!(!password.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_no_uppercase_excludes_uppercase() {
        let mut cli = cli_default();
        cli.no_uppercase = true;
        cli.length = 200;
        let (password, _) = generate_password(&cli).unwrap();
        assert!(!password.chars().any(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_all_exclusions_produces_only_lowercase() {
        let mut cli = cli_default();
        cli.no_symbols = true;
        cli.no_numbers = true;
        cli.no_uppercase = true;
        let (password, _) = generate_password(&cli).unwrap();
        assert!(password.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_multiple_passwords_are_unique() {
        let cli = cli_default();
        let p1 = generate_password(&cli).unwrap();
        let p2 = generate_password(&cli).unwrap();
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_copy_flag() {
        use clap::Parser;

        let cli = Cli::parse_from(["forgekey", "-c"]);
        assert!(cli.copy);
    }

    #[test]
    fn test_weak_password() {
        let (_, level) = calculate_entropy(4, 26);
        assert_eq!(level, "weak");
    }

    #[test]
    fn test_fair_password() {
        let (_, level) = calculate_entropy(6, 36);
        assert_eq!(level, "fair");
    }

    #[test]
    fn test_strong_password() {
        let (_, level) = calculate_entropy(8, 24);
        assert_eq!(level, "strong");
    }

    #[test]
    fn test_very_strong_password() {
        let (_, level) = calculate_entropy(16, 88);
        assert_eq!(level, "very strong");
    }
}
