mod blockchain;
pub use blockchain::Blockchain;

mod block;
pub use block::Block;

pub mod transaction;
pub use transaction::Transaction;

pub mod wallet;
pub use wallet::Wallet;

use std::time::{SystemTime, UNIX_EPOCH};

const DIFFICULTY_LEVEL: i32 = 2;

pub fn now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub fn calculate_hash(
    pre_hash: &str,
    transactions: &[Transaction],
    timestamp: &u64,
    nonce: &u64,
) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(
        transactions
            .iter()
            .flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>(),
    );
    bytes.extend(pre_hash.as_bytes());
    bytes.extend(&nonce.to_ne_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}

pub fn get_difficult_string() -> String {
    let mut s = String::new();
    for _i in 0..DIFFICULTY_LEVEL {
        s.push('0');
    }
    s
}
