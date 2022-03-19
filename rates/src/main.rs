use chrono::NaiveDate;
use color_eyre::{eyre::Report, Result};
use lower::{load_rates, LendingRates};

#[tokio::main]
async fn main() -> Result<(), Report> {
    const RATE_FILE: &str = "lending-rates.json";

    // Load the file with the previous rates if it exists.
    let mut lending_rates = load_rates();

    // Retrieve the new rates.
    let rates = reqwest::Client::new()
        .get("https://platform.lower.com/api/v1/Content/LendingRates")
        .send()
        .await?
        .json::<Vec<LendingRates>>()
        .await?;

    // Check whether we already inserted today's rate.
    let rate_date: Option<NaiveDate> = rates.get(0).map(|entry| entry.current_as_of_date.date());
    if let Some(d) = rate_date {
        if lending_rates
            .iter()
            .all(|entry| entry.current_as_of_date.date() != d)
        {
            // Filter the rates from Lower.
            let mut lower_rates = rates
                .iter()
                .filter(|entry| entry.lender == Some("lower".into()))
                .cloned()
                .collect::<Vec<LendingRates>>();

            // Concatenate the new rates with the previous ones.
            lending_rates.append(&mut lower_rates);

            // Update the file.
            std::fs::write(RATE_FILE, serde_json::to_string_pretty(&lending_rates)?)?;
        }
    }

    Ok(())
}
