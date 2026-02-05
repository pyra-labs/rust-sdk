use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct InitiateWithdrawParams {
    pub owner: Pubkey,
    pub withdraw_order: Pubkey,
    pub payer: Pubkey,
    pub destination: Pubkey,
    pub amount_base_units: u64,
    pub drift_market_index: u16,
    pub reduce_only: bool,
}

pub fn initiate_withdraw(params: &InitiateWithdrawParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);

    let accounts = crate::pyra_program::client::accounts::InitiateWithdrawDrift {
        vault,
        owner: params.owner,
        withdraw_order: params.withdraw_order,
        payer: params.payer,
        system_program: SYSTEM_PROGRAM_ID,
        destination: params.destination,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::InitiateWithdrawDrift {
        amount_base_units: params.amount_base_units,
        drift_market_index: params.drift_market_index,
        reduce_only: params.reduce_only,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
