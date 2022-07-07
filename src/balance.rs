use chrono::{DateTime, Utc};
use web3::transports::Http;
use web3::types::{Address, BlockId, BlockNumber, U256, U64};
use web3::Web3;

/// Binary search
pub async fn get_balance_by_timestamp(
    web3: &Web3<Http>,
    account: Address,
    timestamp: DateTime<Utc>,
) -> Result<Option<U256>, Box<dyn std::error::Error>> {
    let timestamp = timestamp.timestamp();

    let mut block_start = U64::from(0_u64);
    let mut block_end = web3.eth().block_number().await?;
    let mut current_block = (block_end + block_start) / 2_u64;

    let mut min_diff = u64::MAX;
    let mut min_diff_block = None;
    loop {
        let current_block_number_old = current_block;

        if let Some(diff) = check_block(web3, current_block, timestamp).await? {
            if diff < 0 {
                // If "block timestamp" is to the LEFT of "desired timestamp" - then we need to move RIGHT

                move_right(&mut block_start, &mut current_block, block_end);
            } else {
                // If "block timestamp" is to the RIGHT of "desired timestamp" - then we need to move LEFT

                if (diff.abs() as u64) < min_diff {
                    min_diff = diff.abs() as u64;
                    min_diff_block = Some(current_block);
                }

                move_left(block_start, &mut current_block, &mut block_end);
            }
        } else {
            // if we haven't received block - we just move right (towards later blocks)

            move_right(&mut block_start, &mut current_block, block_end);
        }

        // If we haven't moved OR we found minimal diff - then we exit
        if current_block_number_old == current_block || min_diff == 0 {
            break;
        }
    }

    let balance = if let Some(min_diff_block) = min_diff_block {
        let balance = web3
            .eth()
            .balance(account, Some(BlockNumber::Number(min_diff_block)))
            .await?;

        Some(balance)
    } else {
        None
    };

    Ok(balance)
}

/// Move towards earlier blocks
fn move_left(block_start: U64, current_block: &mut U64, block_end: &mut U64) {
    *block_end = *current_block;
    *current_block = (*current_block + block_start) / 2_u64;
}

/// Move towards later blocks
fn move_right(block_start: &mut U64, current_block: &mut U64, block_end: U64) {
    *block_start = *current_block;
    *current_block = (block_end + *current_block) / 2_u64;
}

/// Result: `block_timestamp` - `timestamp`
async fn check_block(
    web3: &Web3<Http>,
    block_number: U64,
    timestamp: i64,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    let block_id = BlockId::Number(BlockNumber::Number(block_number));

    let diff = if let Some(block) = web3.eth().block(block_id).await? {
        let block_timestamp = block.timestamp.as_u64() as i64;
        let diff = block_timestamp - timestamp;

        Some(diff)
    } else {
        None
    };

    Ok(diff)
}
