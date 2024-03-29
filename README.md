# Eth crawler

Program that fetches transaction info of ETH accounts.

## Run

`cargo run -- <params>`

Call example: `cargo run -- --account 0x73BCEb1Cd57C711feaC4224D062b0F6ff338501e --block_start 15069120 --block_end 15069121 --timestamp 2022-07-03`

- After program finishes fetching data - you will have HTML-page with results opened in your default browser.
- Program creates local folder `db` for DB, so, please, make sure that program has all necessary rights for it.

## CLI params

To get help info - pass `-h` (`cargo run -- -h`) param when call.

- **jsonrpc_url** - **\[optional\]** Ethereum JSON RPC server url
- **account** - Ethereum account address
- **block_start** - Ethereum block number start (unsigned integer)
- **block_end** - **\[optional\]** Ethereum block number end (unsigned integer)
- **timestamp** - **\[optional\]** Timestamp to fetch Ethereum account balance. Format: `YYYY-MM-DD`

## Issues

### Deserialization

If we use crate `bincode`, program fails during deserialization with error:

`Error: Custom("Bincode does not support Deserializer::deserialize_identifier")`

As a bypass, we serialize data for DB through JSON-string.
