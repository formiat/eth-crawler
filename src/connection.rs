use crate::constants::JSONRPC_URLS;
use web3::transports::Http;
use web3::Web3;

/// Param `jsonrpc_url` - `None` means default
pub fn try_connect(jsonrpc_url: Option<String>) -> Result<Web3<Http>, Box<dyn std::error::Error>> {
    let urls = jsonrpc_url
        .as_ref()
        .map(|v| vec![v.as_str()])
        .unwrap_or(JSONRPC_URLS.to_vec());

    let mut errors = Vec::new();
    let transport = urls
        .into_iter()
        .map(Http::new)
        .find_map(|r| r.map_err(|e| errors.push(e)).ok());

    let web3 = transport
        .map(Web3::new)
        .ok_or_else(|| errors.swap_remove(0))?;

    Ok(web3)
}
