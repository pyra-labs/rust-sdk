use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

pub struct CancelUpdateSpendLimitsParams {
    pub spend_limits_order: Pubkey,
    pub owner: Pubkey,
    pub order_payer: Pubkey,
}

pub fn cancel_update_spend_limits(params: &CancelUpdateSpendLimitsParams) -> Instruction {
    let accounts = crate::pyra_program::client::accounts::CancelUpdateSpendLimits {
        spend_limits_order: params.spend_limits_order,
        owner: params.owner,
        order_payer: params.order_payer,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::CancelUpdateSpendLimits {}.data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
