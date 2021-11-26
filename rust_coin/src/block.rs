use super::*;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    ///
    /// Block constructor
    /// Creates block from previous block hash and transaction data
    ///

    pub fn new(transactions: Vec<Transaction>) -> Self {
        let time = now();
        let empty_string = String::new();
        let nonce = 0u64;
        Block {
            timestamp: time,
            hash: empty_string.clone(), //calculate_hash(&pre_hash, &transactions, &time),
            pre_hash: empty_string,
            transactions,
            nonce,
        }
    }

    pub fn set_hash(&mut self) {
        self.hash = calculate_hash(
            &self.pre_hash,
            &self.transactions,
            &self.timestamp,
            &self.nonce,
        )
    }

    ///
    /// Set hash of previous block as pre hash
    ///
    pub fn set_pre_hash(&mut self, pre_hash: String) {
        self.pre_hash = pre_hash;
    }

    pub fn mine(&mut self) {
        let target = get_difficult_string();
        while self.hash[..DIFFICULTY_LEVEL as usize] != target {
            self.nonce += 1;
            self.hash = calculate_hash(
                &self.pre_hash,
                &self.transactions,
                &self.timestamp,
                &self.nonce,
            )
        }

        println!("Block Mined");
    }

    pub fn has_valid_transactions(&self) -> bool {
        for tran in &self.transactions {
            if !tran.is_valid_transaction() {
                return false;
            }
        }
        true
    }
}
