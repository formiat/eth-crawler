use crate::cached_transactions::CachedTransactions;
use crate::config::Config;
use crate::html::file::open_results_in_browser;
use crate::logging::start_logger;
use web3::types::U64;

mod cached_transactions;
mod config;
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

    let client = CachedTransactions::new(config.jsonrpc_url).await?;

    info!("Fetch started.");
    let transactions = client
        .get_by_account(
            config
                .account
                .parse()
                .map_err(|e| format!("Account address parse error: {}", e))?,
            U64::from(config.block_start),
            config.block_end.map(U64::from),
        )
        .await?;
    info!("Fetch finished.");

    open_results_in_browser(transactions)?;

    Ok(())
}
