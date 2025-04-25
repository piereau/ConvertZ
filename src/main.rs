use clap::{Parser, Subcommand};
use std::error::Error;
use std::fmt;

// Define currency codes
const EUR: &str = "EUR";
const TRY: &str = "TRY";
const UAH: &str = "UAH";

// Custom error type for our application
#[derive(Debug)]
struct ConverterError(String);

impl fmt::Display for ConverterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Conversion error: {}", self.0)
    }
}

impl Error for ConverterError {}

// Define our CLI structure
#[derive(Parser)]
#[command(author, version, about = "Currency Converter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Define the available commands
#[derive(Subcommand)]
enum Commands {
    /// Convert from one currency to another
    Convert {
        /// Amount to convert
        #[arg(required = true)]
        amount: f64,

        /// Source currency (EUR, TRY, UAH)
        #[arg(required = true)]
        from: String,

        /// Target currency (EUR, TRY, UAH)
        #[arg(required = true)]
        to: String,
    },
}

// Function to convert between currencies
fn convert_currency(amount: f64, from: &str, to: &str) -> Result<f64, ConverterError> {
    // Exchange rates (as of April 2025, for example purposes)
    // These would ideally come from an API but we'll use fixed rates for simplicity
    const EUR_TO_TRY: f64 = 38.5;  // 1 EUR = 38.5 TRY
    const EUR_TO_UAH: f64 = 40.2;  // 1 EUR = 40.2 UAH
    
    // First convert to EUR as our base currency
    let amount_in_eur = match from.to_uppercase().as_str() {
        EUR => amount,
        TRY => amount / EUR_TO_TRY,
        UAH => amount / EUR_TO_UAH,
        _ => return Err(ConverterError(format!("Unknown source currency: {}", from))),
    };
    
    // Then convert from EUR to target currency
    let result = match to.to_uppercase().as_str() {
        EUR => amount_in_eur,
        TRY => amount_in_eur * EUR_TO_TRY,
        UAH => amount_in_eur * EUR_TO_UAH,
        _ => return Err(ConverterError(format!("Unknown target currency: {}", to))),
    };
    
    Ok(result)
}

fn main() {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Match on the command
    match cli.command {
        Commands::Convert { amount, from, to } => {
            // Convert the currencies
            match convert_currency(amount, &from, &to) {
                Ok(result) => {
                    println!("{} {} = {:.2} {}", amount, from.to_uppercase(), result, to.to_uppercase());
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}