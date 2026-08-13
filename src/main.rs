use clap::Parser;

/// Echoes back the argument you pass it.
#[derive(Parser)]
#[command(version, about = "Echoes back the argument you pass it.")]
struct Cli {
    /// The text to echo back
    text: String,
}

fn main() {
    let cli = Cli::parse();
    println!("you wrote: {}", cli.text);
}
