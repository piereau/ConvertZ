use clap::{Parser, Subcommand};
use colored::*;
use serde::Deserialize;
use std::collections::HashMap;

const EUR: &str = "EUR";
const TRY: &str = "TRY";
const UAH: &str = "UAH";

#[derive(Parser)]
#[command(author, version, about = "Simple Currency Converter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(required = true)]
        amount: f64,
        #[arg(required = true)]
        from: String,
        #[arg(required = true)]
        to: String,
    },
}

#[derive(Deserialize)]
struct ExchangeRateResponse {
    rates: HashMap<String, f64>,
    date: String,
}

fn color(currency: &str) -> ColoredString {
    match currency {
        EUR => currency.bright_blue(),
        TRY => currency.bright_red(),
        UAH => currency.bright_yellow(),
        _ => currency.normal(),
    }
}

async fn fetch_rate(from: &str) -> Result<ExchangeRateResponse, reqwest::Error> {
    let url = format!("https://api.exchangerate-api.com/v4/latest/{}", from);
    Ok(reqwest::get(&url).await?.json().await?)
}

#[tokio::main]
async fn main() {
    let Cli { command } = Cli::parse();

    if let Commands::Convert { amount, from, to } = command {
        let from = from.to_uppercase();
        let to = to.to_uppercase();

        println!("{}", "\nConvertZ currency converter".bold().underline());
        println!("{}", "Fetching exchange rates...".dimmed());

        match fetch_rate(&from).await {
            Ok(data) => {
                println!("{} {}", "✓".green(), format!("Got rates as of {}", data.date).dimmed());

                if let Some(rate) = data.rates.get(&to) {
                    println!(
                        "{} with exchange rate: 1.00 {} = {:.4} {}",
                        "Converting".bright_yellow().bold(),
                        color(&from),
                        rate,
                        color(&to)
                    );

                    let result = amount * rate;
                    println!(
                        "\n     {} {} = {} {} \n",
                        format!("{:.2}", amount),
                        from,
                        format!("{:.3}", result).bright_yellow(),
                        color(&to)
                    );
                    
                } else {
                    eprintln!("{} Currency '{}' not supported.", "Error:".red().bold(), to);
                }
            }
            Err(e) => {
                eprintln!("{} Failed to fetch exchange rates: {}", "Error:".red().bold(), e);
            }
        }
    }
}
