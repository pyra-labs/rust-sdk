use solana_program::pubkey::Pubkey;
use solana_pubkey::pubkey;

pub const PYRA_PROGRAM_ID: Pubkey = pubkey!("6JjHXLheGSNvvexgzMthEcgjkcirDrGduc3HAKB2P1v2");
pub const DRIFT_PROGRAM_ID: Pubkey = pubkey!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

mod pda;
pub use pda::*;
