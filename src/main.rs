mod block;
mod wallet;

use chrono::{DateTime, Utc};

fn main() {
    let utc_now: DateTime<Utc> = Utc::now();
    println!("now: {:#?}", utc_now);

    let mut wallet_id = wallet::WalletAddress::new();
    wallet_id.push_str("149826a38f759df945dd3f91f47392483e9289d349b67b13f08874894b623499");
    let miner = wallet::Wallet { id: wallet_id };
    println!("miner: {:#?}", miner);

    let b1: block::Block = block::Block::create_block(None, miner.clone());
    println!("block1: {:#?}", b1);
    let b2: block::Block = block::Block::create_block(Some(Box::new(b1)), miner.clone());
    println!("block2: {:#?}", b2);
}
