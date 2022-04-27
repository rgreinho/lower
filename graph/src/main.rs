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

    // Select only lower.com 30 year purchase rates.
    let purchase_30 = get_term_data(&rates, 30);

    // Extract the min amd max rate.
    let (min_rate, max_rate) = min_max(&purchase_30);
    let min_boundary = min_rate.floor() as i64 * 8;
    let max_boundary = max_rate.ceil() as i64 * 8;

    // Create the line
    let l1 = line("30 years", purchase_30);

    let m = poloto::build::markers([], [min_rate.floor()]);
    let data = poloto::data(plots!(l1, m));

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, true])
        .build();
    let (by, _) = poloto::ticks::bounds(&data, &opt);
    let xtick_fmt = poloto::ticks::from_default(by);
    let ytick_fmt =
        poloto::ticks::from_iter((min_boundary..=max_boundary).map(|x| x as f64 * 0.125));

    let plotter = poloto::plot_with(
        data,
        opt,
        poloto::plot_fmt(
            "lower.com purchase rates",
            "Date",
            "Rate (%)",
            xtick_fmt,
            ytick_fmt,
        ),
    );

    // Write the graph to disk.
    std::fs::write(
        "graph.svg",
        format!("{}", poloto::disp(|w| plotter.simple_theme(w))),
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

fn min_max(rates: &[(UnixTime, f64)]) -> (f64, f64) {
    let it = rates.iter().map(|r| r.1);
    let min_rate = it.clone().fold(f64::INFINITY, |a, b| a.min(b));
    let max_rate = it.fold(0.0, |a: f64, b| a.max(b));
    (min_rate, max_rate)
}
