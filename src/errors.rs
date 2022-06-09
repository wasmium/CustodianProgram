#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractErrors {
    StoreError = 1,
    WalletError = 2,
    InvalidInstructionData4Borsh = 3,
}
