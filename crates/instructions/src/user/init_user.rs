use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct InitUserParams {
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub spend_limit_per_transaction: u64,
    pub spend_limit_per_timeframe: u64,
    pub next_timeframe_reset_timestamp: u64,
    pub timeframe_in_seconds: u64,
}

pub fn init_user(params: &InitUserParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);

    let accounts = crate::pyra_program::client::accounts::InitUser {
        vault,
        owner: params.owner,
        payer: params.payer,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::InitUser {
        spend_limit_per_transaction: params.spend_limit_per_transaction,
        spend_limit_per_timeframe: params.spend_limit_per_timeframe,
        next_timeframe_reset_timestamp: params.next_timeframe_reset_timestamp,
        timeframe_in_seconds: params.timeframe_in_seconds,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
