use chrono::TimeZone;
use chrono::Utc;
use color_eyre::{eyre::Report, Result};
use lower::{load_rates, LendingRates, LoanType};
use poloto::build::line;
use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

fn main() -> Result<(), Report> {
    // Load all the rates.
    let rates = load_rates();

    let purchase_30 = get_term_data(&rates, 30);
    let purchase_20 = get_term_data(&rates, 20);
    let purchase_15 = get_term_data(&rates, 15);
    dbg!(&purchase_30);

    let l1 = line("30 years", purchase_30);
    let l2 = line("20 years", purchase_20);
    let l3 = line("15 years", purchase_15);

    let mut plotter = plots!(l1, l2, l3).build_with([], [0.0]).stage().plot(
        "lower.com purchase rates",
        "Date",
        "Rate (%)",
    );

    // Write the graph to disk.
    std::fs::write(
        "graph.svg",
        format!("{}", poloto::disp(|w| plotter.simple_theme_dark(w))),
    )?;

    Ok(())
}

fn get_term_data(rates: &[LendingRates], term: u8) -> Vec<(UnixTime, f64)> {
    rates
        .iter()
        .filter(|e| e.term == term)
        .filter(|e| e.loan_type == LoanType::Purchase)
        .map(|e| {
            (
                UnixTime::from(Utc.from_utc_date(&e.current_as_of_date.date())),
                e.rate,
            )
        })
        .collect::<Vec<(UnixTime, f64)>>()
}
