use clap::Parser;
use alpha_counter::AlphaCounter;

/// Alphabetic Counter (A, B, C, ..., X, Y, Z, AA, AB, AC, ...)
#[derive(Parser)]
#[clap(name = "alpha-counter")]
struct Cli {
    /// Kind (upper, lower)
    #[clap(short, long, default_value = "upper")]
    kind: String,

    /// Start
    #[clap(short, long, default_value_t = 0)]
    start: usize,

    /// Take
    #[clap(short, long, default_value_t = 100)]
    take: usize,

    /// Custom alphabet
    #[clap(short, long)]
    alphabet: Option<String>,
}

/// Command line interface
fn main() -> Result<(), String> {
    let cli = Cli::parse();

    if !["upper", "lower"].contains(&cli.kind.as_str()) {
        return Err(String::from("Kind option must be either \"upper\" or \"lower\"."));
    }

    let ac = if let Some(alphabet) = cli.alphabet {
        AlphaCounter::custom(cli.start, &alphabet)
    } else if cli.kind == "lower" {
        AlphaCounter::lower(cli.start)
    } else {
        AlphaCounter::upper(cli.start)
    };

    for i in ac.take(cli.take) {
        println!("{i}");
    }

    Ok(())
}
