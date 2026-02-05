use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::user::{InitDriftParams, InitUserParams};

pub struct InitPyraUserParams {
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub spend_limit_per_transaction: u64,
    pub spend_limit_per_timeframe: u64,
    pub next_timeframe_reset_timestamp: u64,
    pub timeframe_in_seconds: u64,
}

pub fn init_pyra_user(params: &InitPyraUserParams) -> Vec<Instruction> {
    vec![
        crate::user::init_user(&InitUserParams {
            owner: params.owner,
            payer: params.payer,
            spend_limit_per_transaction: params.spend_limit_per_transaction,
            spend_limit_per_timeframe: params.spend_limit_per_timeframe,
            next_timeframe_reset_timestamp: params.next_timeframe_reset_timestamp,
            timeframe_in_seconds: params.timeframe_in_seconds,
        }),
        crate::user::init_drift(&InitDriftParams {
            owner: params.owner,
            payer: params.payer,
            sub_account_id: 0,
        }),
    ]
}
