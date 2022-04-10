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
    let min_rate = rates
        .iter()
        .map(|r| r.rate)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max_rate = rates.iter().map(|r| r.rate).fold(0.0, |a: f64, b| a.max(b));
    // dbg!(&max_rate);
    let min_boundary = min_rate.floor() as i64 * 4;
    let max_boundary = max_rate.ceil() as i64 * 4;
    // dbg!(&min_boundary);
    // dbg!(&max_boundary);

    let purchase_30 = get_term_data(&rates, 30);
    let purchase_20 = get_term_data(&rates, 20);
    let purchase_15 = get_term_data(&rates, 15);
    // dbg!(&purchase_30);

    let l1 = line("30 years", purchase_30);
    let l2 = line("20 years", purchase_20);
    let l3 = line("15 years", purchase_15);

    let m = poloto::build::markers([], [min_rate.floor()]);
    // let data = plots!(l1, l2, l3, m);
    let data = poloto::data(plots!(l1, l2, l3, m));

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, true])
        .build();
    let (by, _) = poloto::ticks::bounds(&data, &opt);
    let xtick_fmt = poloto::ticks::from_default(by);
    let ytick_fmt =
        poloto::ticks::from_iter((min_boundary..=max_boundary).map(|x| x as f64 * 0.25));

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
