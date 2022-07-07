use sled::IVec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::str;
use std::str::FromStr;
use web3::transports::Http;
use web3::types::{
    Address, BlockId, BlockNumber, Transaction, TransactionId, TransactionReceipt, H256, U64,
};
use web3::Web3;

pub struct CachedTransactions {
    web3: Web3<Http>,
    db: sled::Db,

    /// HashMap key - account
    /// HashMap value - (first_block_number, last_block_number)
    cache_keys: RefCell<HashMap<Address, (U64, U64)>>,
}

impl CachedTransactions {
    pub async fn new(web3: Web3<Http>) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sled::open("db")?;

        let cache_keys = Self::read_cache_keys(&db).await?;
        let cache_keys = RefCell::new(cache_keys);

        Ok(Self {
            web3,
            db,
            cache_keys,
        })
    }

    async fn calculate_borders(
        cache_keys: Vec<(Address, U64)>,
    ) -> Result<HashMap<Address, (U64, U64)>, Box<dyn std::error::Error>> {
        let mut cache_keys_2: HashMap<Address, (U64, U64)> = HashMap::new();

        for (account, block_number) in cache_keys {
            let entry = cache_keys_2
                .entry(account)
                .or_insert((block_number, block_number));

            // min
            if block_number < entry.0 {
                entry.0 = block_number;
            }

            // max
            if block_number > entry.1 {
                entry.1 = block_number;
            }
        }

        Ok(cache_keys_2)
    }

    async fn read_cache_keys(
        db: &sled::Db,
    ) -> Result<HashMap<Address, (U64, U64)>, Box<dyn std::error::Error>> {
        let mut errors: Vec<Box<dyn std::error::Error>> = Vec::new();
        let cache_keys: Vec<IVec> = db
            .scan_prefix("")
            .keys()
            .filter_map(|r| r.map_err(|e| errors.push(Box::new(e))).ok())
            .collect();
        let cache_keys: Vec<_> = cache_keys
            .iter()
            .map(|v: &IVec| str::from_utf8(v))
            .filter_map(|r| r.map_err(|e| errors.push(Box::new(e))).ok())
            .collect();
        let cache_keys: Vec<_> = cache_keys
            .iter()
            .map(|v| Self::parse_key_string_short(v))
            .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
            .collect();

        if !errors.is_empty() {
            Err(errors.swap_remove(0))?;
        }

        let cache_keys = Self::calculate_borders(cache_keys).await?;

        Ok(cache_keys)
    }

    /// Param `block_end` - `None` means last
    pub async fn get_by_account(
        &self,
        account: Address,
        block_start: U64,
        block_end: Option<U64>,
    ) -> Result<Vec<(u64, Transaction, Option<TransactionReceipt>)>, Box<dyn std::error::Error>>
    {
        let block_end = block_end.unwrap_or(self.web3.eth().block_number().await?);

        let cache_block_numbers = self.cache_keys.borrow().get(&account).cloned();

        let need_fetch_from_server =
            if let Some((old_block_start, old_block_end)) = cache_block_numbers {
                // If requested borders are inside cached borders - then return `false`
                // else - return `true`

                block_start < old_block_start || block_end > old_block_end
            } else {
                true
            };

        let transactions = if need_fetch_from_server {
            self.get_from_server_and_save_to_cache(account, block_start, block_end)
                .await?
        } else {
            self.get_from_cache(account, block_start, block_end).await?
        };

        Ok(transactions)
    }

    /// TODO: Remove json layer (serialize directly into bytes)
    async fn get_from_cache(
        &self,
        account: Address,
        block_start: U64,
        block_end: U64,
    ) -> Result<Vec<(u64, Transaction, Option<TransactionReceipt>)>, Box<dyn std::error::Error>>
    {
        info!(
            "From cache. Block start: {:?}. Block end: {:?}",
            block_start, block_end,
        );

        let key_range = Self::stringify_key_range(account, block_start, block_end);
        debug!("key range: {:?}", key_range);

        let mut errors: Vec<Box<dyn std::error::Error>> = Vec::new();
        let transactions: Vec<_> = self
            .db
            .range(key_range)
            .filter_map(|r| r.map_err(|e| errors.push(Box::new(e))).ok())
            .map(|v| {
                // Discard key
                v.1
            })
            .collect();
        let transactions: Vec<_> = transactions
            .into_iter()
            .map(|ivec| {
                // TODO: Remove json layer (serialize directly into bytes)
                serde_json::from_slice(&ivec)
            })
            .filter_map(|r| r.map_err(|e| errors.push(Box::new(e))).ok())
            .collect();

        if !errors.is_empty() {
            Err(errors.swap_remove(0))?;
        }

        info!("Got transactions len: {}", transactions.len());

        Ok(transactions)
    }

    async fn get_from_server_and_save_to_cache(
        &self,
        account: Address,
        block_start: U64,
        block_end: U64,
    ) -> Result<Vec<(u64, Transaction, Option<TransactionReceipt>)>, Box<dyn std::error::Error>>
    {
        info!(
            "From server. Block start: {:?}. Block end: {:?}",
            block_start, block_end,
        );

        let block_numbers = block_start.as_u64()..=block_end.as_u64();

        let mut transactions = Vec::new();

        for block_number in block_numbers {
            let block_number = U64::from(block_number);

            let block_id = BlockId::Number(BlockNumber::Number(block_number));
            let block = self.web3.eth().block(block_id).await?;

            if let Some(block) = block {
                let block_timestamp = block.timestamp.as_u64();
                let tr_hashes = block.transactions;

                for tr_hash in tr_hashes {
                    let tr_id = TransactionId::Hash(tr_hash);

                    if let Some(transaction) = self.web3.eth().transaction(tr_id).await? {
                        if transaction.from == Some(account) || transaction.to == Some(account) {
                            // Desired transaction

                            let tr_receipt = self.web3.eth().transaction_receipt(tr_hash).await?;
                            let key = (account, block_number, tr_hash);
                            let transaction = (block_timestamp, transaction, tr_receipt);

                            self.cache_transaction(&key, &transaction).await?;

                            transactions.push(transaction);
                        }
                    }
                }
            }
        }

        // Save new cached borders
        self.cache_keys
            .borrow_mut()
            .insert(account, (block_start, block_end));

        info!("Got transactions len: {}", transactions.len());

        Ok(transactions)
    }

    fn stringify_key(account: Address, block_number: U64, tr_hash: H256) -> String {
        // Add leading zeros to `block_number` string view
        format!("{:?}_{:0>32?}_{:?}", account, block_number, tr_hash)
    }

    fn stringify_key_range(account: Address, block_start: U64, block_end: U64) -> Range<String> {
        // Add leading zeros to `block_number` string view
        let begin = format!("{:?}_{:0>32?}", account, block_start);

        // Add leading zeros to `block_number` string view
        // Make max value for `block_number` - `tilda` char has biggest code in ASCII table
        let end = format!("{:?}_{:0>32?}_~~~", account, block_end);

        begin..end
    }

    fn parse_key_string(key: &str) -> Result<(Address, U64, H256), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = key.split('_').collect();
        assert_eq!(parts.len(), 3);

        let account = parts[0].parse()?;
        let block_number = U64::from(parts[1].parse::<u64>()?);
        let tr_hash = H256::from_str(parts[2])?;

        let res = (account, block_number, tr_hash);
        Ok(res)
    }

    fn parse_key_string_short(key: &str) -> Result<(Address, U64), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = key.split('_').collect();
        assert!(parts.len() >= 2);

        let account = parts[0].parse()?;
        let block_number = U64::from(parts[1].parse::<u64>()?);

        let res = (account, block_number);
        Ok(res)
    }

    /// TODO: Remove json layer (serialize directly into bytes)
    /// Save transaction to DB
    async fn cache_transaction(
        &self,
        key: &(Address, U64, H256),
        transaction: &(u64, Transaction, Option<TransactionReceipt>),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = Self::stringify_key(key.0, key.1, key.2);
        debug!("key: {}", key);

        // TODO: Remove json layer (serialize directly into bytes)
        let bytes = serde_json::to_vec(&transaction)?;

        self.db.insert(key, bytes)?;

        Ok(())
    }
}
