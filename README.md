# Eth crawler

Program that fetches transaction info of ETH accounts.

## Run

`cargo run <params>`

After program finishes fetching data - you will have HTML-page with results opened in your default browser.

## CLI params

To get help info - pass `-h` param when call.

- **jsonrpc_url** - **\[optional\]** Ethereum JSON RPC server url
- **account** - Ethereum account address
- **block_start** - Ethereum block number start (unsigned integer)
- **block_end** - **\[optional\]** Ethereum block number end (unsigned integer)

## Issues

### Deserialization

If we use crate `bincode`, program fails during deserialization with error:

`Error: Custom("Bincode does not support Deserializer::deserialize_identifier")`

As a bypass, we serialize data for DB through JSON-string.
