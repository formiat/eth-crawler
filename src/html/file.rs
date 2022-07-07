use crate::html::render::render_html;
use chrono::{DateTime, Utc};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{thread, time::Duration};
use web3::types::{Transaction, TransactionReceipt, U256};

pub fn open_results_in_browser(
    account: String,
    transactions: Vec<(u64, Transaction, Option<TransactionReceipt>)>,
    balance: Option<(DateTime<Utc>, U256)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (html_file_dir, html_file_path) = save_results_to_file(account, transactions, balance)?;
    let html_file_url = format!("file://{}", html_file_path);

    debug!("Html file path: {}", html_file_path);
    debug!("Html file url: {}", html_file_url);
    open::that(&html_file_url).map_err(|e| format!("Error opening html file in browser: {}", e))?;

    info!("Waiting for 5 seconds for the browser to read html file...");
    thread::sleep(Duration::from_millis(5000));

    fs::remove_file(html_file_path)?;
    info!("Html file removed.");
    fs::remove_dir(html_file_dir)?;
    info!("Tmp dir removed.");

    Ok(())
}

fn save_results_to_file(
    account: String,
    transactions: Vec<(u64, Transaction, Option<TransactionReceipt>)>,
    balance: Option<(DateTime<Utc>, U256)>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let current_path = fs::canonicalize(&PathBuf::from("./"))?
        .as_path()
        .to_str()
        .ok_or("Path error.")?
        .to_string();

    let html_file_dir_name = "tmp";
    let html_file_name = "results.html";
    let html_file_dir_path = format!("{}/{}", current_path, html_file_dir_name);
    let html_file_path = format!("{}/{}", html_file_dir_path, html_file_name);

    fs::create_dir_all(&html_file_dir_path)?;
    let mut html_file = File::create(&html_file_path)?;

    let html_string = render_html(account, transactions, balance)?;
    html_file.write_all(html_string.as_bytes())?;

    let res = (html_file_dir_path, html_file_path);
    Ok(res)
}
