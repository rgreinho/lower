use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const RATE_FILE: &'static str = "lending-rates.json";

#[derive(Deserialize, Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LoanType {
    Purchase,
    Refinance,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
// {
//   "lender":"opendoor",
//   "term":30,
//   "apr":4.179,
//   "discountPoints":1.75,
//   "loanAmount":200000.00,
//   "currentAsOfDate":"2022-03-16T15:40:09",
//   "loanType":"purchase",
//   "rate":3.990,
//   "payment":953.68
// }
pub struct LendingRates {
    pub lender: Option<String>,
    pub term: u8,
    pub apr: f64,
    pub discount_points: f64,
    #[serde(with = "date_serializer")]
    pub current_as_of_date: NaiveDateTime,
    pub loan_type: LoanType,
    pub rate: f64,
}

pub fn load_rates() -> Vec<LendingRates> {
    // Load the file with the previous rates if it exists.
    let rate_file = PathBuf::from(RATE_FILE);
    let mut lending_rates: Vec<LendingRates> = Vec::new();
    if rate_file.exists() {
        lending_rates = std::fs::read_to_string(&rate_file)
            .map(|p| serde_json::from_str::<Vec<LendingRates>>(&p).unwrap())
            .unwrap();
    }

    lending_rates
}

mod date_serializer {
    use chrono::NaiveDateTime;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        dt.format(DATE_FORMAT).to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, DATE_FORMAT).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};

    #[test]
    fn test_date_parsing() {
        let dt = NaiveDateTime::parse_from_str("2022-03-16T15:40:09", date_serializer::DATE_FORMAT);
        assert_eq!(
            dt,
            Ok(NaiveDate::from_ymd(2022, 03, 16).and_hms(15, 40, 09))
        );
    }
}
