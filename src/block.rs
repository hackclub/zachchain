use super::wallet::{Tx, Wallet};
use arrayvec::ArrayString;
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

type BlockHash = ArrayString<64>;
type MerkleHash = ArrayString<64>;

#[derive(Debug, Clone, Hash)]
pub struct Block {
    pub chain_version: u8,
    pub ts: DateTime<Utc>,
    pub height: u128,
    pub miner: Wallet,
    pub prev_block: Option<Box<Block>>,
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub merkle_root: Option<MerkleHash>,
    pub txs: Vec<Tx>,
}

impl Block {
    pub fn new(
        height: u128,
        miner: Wallet,
        prev_block: Option<Box<Block>>,
        prev_hash: BlockHash,
    ) -> Self {
        let mut b = Block {
            chain_version: 0,
            ts: Utc::now(),
            height,
            miner,
            prev_block,
            hash: BlockHash::new(),
            prev_hash,
            merkle_root: None,
            txs: vec![],
        };
        let mut h = BlockHash::new();
        let s = calculate_hash(&b).to_string();
        h.push_str(&s);
        b.hash = h;
        b
    }
    //pub fn create_block(&mut self, data: String, miner: Wallet, txs: Vec<Tx>) -> Self {
    pub fn create_block(a_prev_block: Option<Box<Block>>, miner: Wallet) -> Block {
        let prev_block: Option<Box<Block>> = match a_prev_block {
            Some(ref e) => Some(e.to_owned()),
            None => None,
        };
        let prev_hash: BlockHash = match prev_block {
            Some(ref e) => e.hash,
            None => {
                let mut h = BlockHash::new();
                h.push_str("0000000000000000000000000000000000000000000000000000000000000000");
                h
            }
        };
        let last_height: u128 = match prev_block {
            Some(ref e) => e.height,
            None => 0,
        };
        Block::new(last_height + 1, miner, prev_block, prev_hash)
    }
}
