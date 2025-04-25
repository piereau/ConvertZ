use clap::{Parser, Subcommand};
use colored::*;
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

// Function to get currency color
fn get_currency_color(currency: &str) -> ColoredString {
    match currency.to_uppercase().as_str() {
        EUR => "EUR".bright_blue(),
        TRY => "TRY".bright_red(),
        UAH => "UAH".bright_yellow(),
        _ => currency.normal(),
    }
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
                    let from_colored = get_currency_color(&from);
                    let to_colored = get_currency_color(&to);
                    
                    // Format the result with 2 decimal places
                    let formatted_amount = format!("{:.2}", amount).green();
                    let formatted_result = format!("{:.2}", result).green();
                    
                    println("test");
                    // Display the conversion with colors
                    println!(
                        "{} {} = {} {}", 
                        formatted_amount, 
                        from_colored,
                        formatted_result, 
                        to_colored
                    );
                    
                    // Show the exchange rate
                    let rate = result / amount;
                    println!(
                        "Exchange rate: {} {} = {} {}", 
                        "1.00".yellow(), 
                        from_colored,
                        format!("{:.4}", rate).yellow(), 
                        to_colored
                    );
                },
                Err(e) => {
                    eprintln!("{}", "Error:".bright_red().bold());
                    eprintln!("  {}", e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
    }
}