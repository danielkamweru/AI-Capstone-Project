# Greeting CLI

A simple command-line application written in Rust that greets users by name.

## Description

The Greeting CLI is a minimalistic tool that takes a user's name as a command-line argument and prints a friendly greeting message. It's built using Rust for performance and safety, making it a great example of a basic CLI application.

## Features

- Accepts a name as a command-line argument
- Prints a personalized greeting with an emoji
- Simple and lightweight with no external dependencies

## Requirements

- Rust (version 1.56 or later recommended, as it uses the 2021 edition)

## Installation

1. Ensure you have Rust installed on your system. If not, download and install it from [rustup.rs](https://rustup.rs/).

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd greeting_cli
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

   This will create an optimized binary in `target/release/greeting_cli`.

## Usage

Run the application using Cargo:

```bash
cargo run <your_name>
```

Or, after building, run the binary directly:

```bash
./target/release/greeting_cli <your_name>
```

If no name is provided, the application will display usage instructions.

## Examples

```bash
$ cargo run Alice
Hello, Alice 👋

$ cargo run Bob
Hello, Bob 👋
```

## Development

To contribute or modify the code:

1. Clone the repository as described above.
2. Make your changes to `src/main.rs`.
3. Test your changes:
   ```bash
   cargo run <test_name>
   ```
4. Build and run tests (if any):
   ```bash
   cargo build
   cargo test
   ```

## Project Structure

- `Cargo.toml`: Project configuration and dependencies
- `src/main.rs`: Main application logic
- `target/`: Build artifacts (generated)

## License

This project is open-source. Please refer to the license file if available.