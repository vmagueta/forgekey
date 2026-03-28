# 🔑 forgekey

A fast, minimal password generator CLI built in Rust.

## Installation

```bash
cargo install forgekey
```

## Usage

```bash
# Generate a password (default: 16 characters)
forgekey

# Custom length
forgekey --length 32

# Generate multiple passwords
forgekey -n 5

# Exclude symbols
forgekey --no-symbols

# Exclude numbers and uppercase
forgekey --no-numbers --no-uppercase

# Combine flags
forgekey -l 32 -n 5 --no-symbols
```

## Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--length` | `-l` | Password length | `16` |
| `--number` | `-n` | Number of passwords | `1` |
| `--no-symbols` | | Exclude symbols | `false` |
| `--no-numbers` | | Exclude numbers | `false` |
| `--no-uppercase` | | Exclude uppercase | `false` |

## Built with

- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [rand](https://crates.io/crates/rand) — Cryptographically secure randomness
- [colored](https://crates.io/crates/colored) — Terminal colors

## Contributing

Contributions are welcome! Check out the [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and ideas.

## License

MIT
