use arrayvec::ArrayString;

pub type WalletAddress = ArrayString<64>;

#[derive(Debug, Clone, Hash)]
pub struct Wallet {
    pub id: WalletAddress,
}

pub type TxId = ArrayString<64>;

#[derive(Debug, Clone, Hash)]
pub struct Tx {
    pub id: TxId,
    pub fr: Wallet,
    pub to: Wallet,
    pub amount: u128,
}
