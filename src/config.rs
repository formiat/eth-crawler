use chrono::{DateTime, NaiveDate, ParseError, Utc};
use clap::{Arg, ArgMatches, Command, ValueHint};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
    pub jsonrpc_url: Option<String>,
    pub account: String,
    pub block_start: u64,
    pub block_end: Option<u64>,
    pub timestamp: Option<DateTime<Utc>>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let matches = Self::make_matches();

        Ok(Self {
            jsonrpc_url: matches.get_one("jsonrpc_url").cloned(),
            account: matches.get_one("account").cloned().unwrap(),
            block_start: matches.value_of("block_start").unwrap().parse()?,
            block_end: matches
                .value_of("block_end")
                .map(u64::from_str)
                .transpose()?,
            timestamp: matches
                .value_of("timestamp")
                .map(Self::date_time_from_string)
                .transpose()?,
        })
    }

    /// Call only once
    fn make_matches() -> ArgMatches {
        Command::new("Eth crawler")
            .version("1.0")
            .arg(
                Arg::new("jsonrpc_url")
                    .long("jsonrpc_url")
                    .value_name("URL")
                    .help("Ethereum JSON RPC url")
                    .value_hint(ValueHint::Url),
            )
            .arg(
                Arg::new("account")
                    .long("account")
                    .value_name("ACCOUNT")
                    .help("Ethereum account address")
                    .required(true),
            )
            .arg(
                Arg::new("block_start")
                    .long("block_start")
                    .value_name("BLOCK_START")
                    .help("Ethereum block number start (unsigned integer)")
                    .required(true),
            )
            .arg(
                Arg::new("block_end")
                    .long("block_end")
                    .value_name("BLOCK_END")
                    .help("Ethereum block number end (unsigned integer)"),
            )
            .arg(
                Arg::new("timestamp")
                    .long("timestamp")
                    .value_name("YYYY-MM-DD")
                    .help("Timestamp in YYYY-MM-DD format to fetch account balance"),
            )
            .get_matches()
    }

    fn date_time_from_string(timestamp: &str) -> Result<DateTime<Utc>, ParseError> {
        let timestamp = NaiveDate::parse_from_str(timestamp, "%Y-%m-%d")?;
        let timestamp = timestamp.and_hms(0, 0, 0);
        let timestamp = DateTime::<Utc>::from_utc(timestamp, Utc);

        Ok(timestamp)
    }
}
