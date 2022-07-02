use crate::html::templates::{DATA_CELL_HTML, HEADER_CELL_HTML, RESULTS_HTML, ROW_HTML};
use chrono::{DateTime, NaiveDateTime, Utc};
use web3::types::{Transaction, TransactionReceipt};

pub fn render_html(
    transactions: Vec<(u64, Transaction, Option<TransactionReceipt>)>,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(RESULTS_HTML.replace("{rows}", &render_rows(transactions)?))
}

fn render_rows(
    transactions: Vec<(u64, Transaction, Option<TransactionReceipt>)>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut rows = String::new();

    // Header row
    let row = ROW_HTML.replace("{cells}", &render_header_row()?);
    rows.push_str(&row);

    for transaction in transactions {
        let row = ROW_HTML.replace("{cells}", &render_row(transaction)?);

        rows.push_str(&row);
    }

    Ok(rows)
}

fn render_header_row() -> Result<String, Box<dyn std::error::Error>> {
    let header_cells = [
        // Transaction
        "transaction hash",
        "block number",
        "timestamp",
        "from",
        "to",
        "value",
        "gas price",
        "gas used",
        "transaction type",
        // TransactionReceipt
        "status",
    ];

    let mut row = String::new();
    for header_cell in header_cells {
        let cell = HEADER_CELL_HTML.replace("{data}", header_cell);

        row.push_str(&cell);
    }

    Ok(row)
}

fn render_row(
    transaction: (u64, Transaction, Option<TransactionReceipt>),
) -> Result<String, Box<dyn std::error::Error>> {
    let mut row = String::new();

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace("{data}", &format!("{:?}", transaction.1.hash));
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        &format!("{:?}", transaction.1.block_number.unwrap_or_default()),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        &format!("{:?}", date_time_from_timestamp_sec(transaction.0))
            .replace('T', " ")
            .replace('Z', ""),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        &format!("{:?}", transaction.1.from.unwrap_or_default()),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        &format!("{:?}", transaction.1.to.unwrap_or_default()),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        &format!("{:?} ETH", transaction.1.value.as_u64() as f64 / 1e18),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        &format!("{:?}", transaction.1.gas_price.unwrap_or_default()),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace("{data}", &format!("{:?}", transaction.1.gas));
    row.push_str(&cell);

    // *******************************************************************************************************************

    let cell = DATA_CELL_HTML.replace(
        "{data}",
        transaction
            .1
            .transaction_type
            .map(|v| v.as_u64())
            .map(|v| match v {
                0 => "legacy",
                1 => "accesslists",
                _ => "eip1559",
            })
            .unwrap_or(""),
    );
    row.push_str(&cell);

    // *******************************************************************************************************************

    if let Some(tr_receipt) = transaction.2 {
        // *******************************************************************************************************************

        let cell = DATA_CELL_HTML.replace(
            "{data}",
            tr_receipt
                .status
                .map(|v| v.as_u64())
                .map(|v| if v == 1 { "success" } else { "failure" })
                .unwrap_or(""),
        );
        row.push_str(&cell);

        // *******************************************************************************************************************
    } else {
        for _ in 0..1 {
            row.push_str("");
        }
    }

    Ok(row)
}

fn date_time_from_timestamp_sec(timestamp_sec: u64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp_sec as i64, 0);

    DateTime::from_utc(naive, Utc)
}
