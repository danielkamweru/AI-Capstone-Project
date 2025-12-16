use std::env;

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if a name was provided
    if args.len() < 2 {
        println!("Usage: cargo run <your_name>");
        return;
    }

    // The first argument is the program name, the second is the user's name
    let name = &args[1];

    // Print greeting
    println!("Hello, {} 👋", name);
}
