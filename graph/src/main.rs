use chrono::{TimeZone, Utc};
use color_eyre::{eyre::Report, Result};
use lower::{load_rates, LendingRates, LoanType};
use poloto::{build::line, num::timestamp::UnixTime, prelude::*};

fn main() -> Result<(), Report> {
    // Load all the rates.
    let rates = load_rates();

    // Select only lower.com 30 year purchase rates.
    let _purchase_30 = get_rate_terms(&rates, 30, LoanType::Purchase);
    let refinance_15 = get_rate_terms(&rates, 15, LoanType::Refinance);
    let refinance_20 = get_rate_terms(&rates, 20, LoanType::Refinance);
    let refinance_30 = get_rate_terms(&rates, 30, LoanType::Refinance);
    let refinance_all_ = vec![
        refinance_15.clone(),
        refinance_20.clone(),
        refinance_30.clone(),
    ];
    let refinance_all = refinance_all_
        .into_iter()
        .flatten()
        .collect::<Vec<(UnixTime, f64)>>();

    // Extract the min amd max rate.
    let (refi_min_rate, refi_max_rate) = min_max(&refinance_all);
    let min_boundary = refi_min_rate.floor() as i64 * 8;
    let max_boundary = refi_max_rate.ceil() as i64 * 8;

    // Create the line
    let l1 = line("Refinance 15", refinance_15);
    let l2 = line("Refinance 20", refinance_20);
    let l3 = line("Refinance 30", refinance_30);

    let m = poloto::build::markers([], []);
    let data = poloto::data(plots!(l1, l2, l3, m));

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
        format!("{}", poloto::disp(|w| plotter.simple_theme_dark(w))),
    )?;

    Ok(())
}

fn get_rate_terms(rates: &[LendingRates], term: u8, loan_type: LoanType) -> Vec<(UnixTime, f64)> {
    rates
        .iter()
        .filter(|e| e.term == term)
        .filter(|e| e.loan_type == Some(loan_type))
        .map(|e| {
            (
                UnixTime::from(Utc.from_utc_date(&e.current_as_of_date.date())),
                e.apr,
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
