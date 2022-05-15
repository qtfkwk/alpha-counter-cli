use clap::Parser;
use alpha_counter::AlphaCounter;

/// Alphabetic Counter (A, B, C, ..., X, Y, Z, AA, AB, AC, ...)
#[derive(Parser)]
#[clap(name = "alpha-counter")]
struct Args {
    /// Kind (upper, lower)
    #[clap(short, long, default_value = "upper")]
    kind: String,

    /// Start
    #[clap(short, long, default_value_t = 0)]
    start: usize,

    /// Take
    #[clap(short, long, default_value_t = 100)]
    take: usize,
}

/// Command line interface
fn main() -> Result<(), String> {
    let args = Args::parse();

    if !["upper", "lower"].contains(&args.kind.as_str()) {
        return Err(String::from("Kind option must be either \"upper\" or \"lower\"."));
    }

    let ac = if args.kind == "lower" {
        AlphaCounter::lower(args.start)
    } else {
        AlphaCounter::upper(args.start)
    };

    for i in ac.take(args.take) {
        println!("{i}");
    }

    Ok(())
}
