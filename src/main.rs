use crate::balance::get_balance_by_timestamp;
use crate::cached_transactions::CachedTransactions;
use crate::config::Config;
use crate::connection::try_connect;
use crate::html::file::open_results_in_browser;
use crate::logging::start_logger;
use web3::types::U64;

mod balance;
mod cached_transactions;
mod config;
mod connection;
mod constants;
mod html;
mod logging;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_logger(None);

    let config = Config::new()?;
    debug!("Got config: {:?}", config);

    let web3 = try_connect(config.jsonrpc_url)?;

    let account = config
        .account
        .parse()
        .map_err(|e| format!("Account address parse error: {}", e))?;

    let balance = if let Some(timestamp) = config.timestamp {
        info!("Fetch balance started.");
        let balance = get_balance_by_timestamp(&web3, account, timestamp)
            .await?
            .map(|v| (timestamp, v));
        info!("Fetch balance finished.");

        balance
    } else {
        None
    };

    let client = CachedTransactions::new(web3).await?;

    info!("Fetch transactions started.");
    let transactions = client
        .get_by_account(
            account,
            U64::from(config.block_start),
            config.block_end.map(U64::from),
        )
        .await?;
    info!("Fetch transactions finished.");

    open_results_in_browser(config.account, transactions, balance)?;

    Ok(())
}
