pub const RESULTS_HTML: &str = "<!DOCTYPE html><html><head><style>th, td {border: solid 1px; padding: 5px 10px; white-space: nowrap;}</style></head><body><h1>Results</h1><article><h2>Account</h2><div><span>{account}</span></div></article>{balance}<article><h2>Transactions</h2><table>{rows}</table></article></body></html>";
pub const BALANCE_HTML: &str = "<article><h2>Balance</h2><div><span style=\"font-weight: bold;\">Timestamp: </span><span>{timestamp}</span></div><div><span style=\"font-weight: bold;\">Balance: </span><span>{balance}</span></div></article>";
pub const ROW_HTML: &str = "<tr>{cells}</tr>";
pub const HEADER_CELL_HTML: &str = "<th>{data}</th>";
pub const DATA_CELL_HTML: &str = "<td>{data}</td>";
