use ed25519_dalek::Keypair;
use rust_coin::*;

fn main() {
    let mut blockchain = Blockchain::new();

    let ryan_key = Wallet::new();
    let dan_key = Wallet::new();

    let miner_key = Wallet::new();

    let mut first_transaction = Transaction {
        sender: Some(ryan_key.public),
        receiver: Some(dan_key.public),
        amount: 2000.0,
        signature: None,
    };

    first_transaction.sign_transaction(Keypair {
        public: ryan_key.public,
        secret: ryan_key.secret,
    });

    blockchain.add_new_transaction(first_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);
}
