use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

pub struct FulfilUpdateSpendLimitsParams {
    pub spend_limits_order: Pubkey,
    pub owner: Pubkey,
    pub order_payer: Pubkey,
    pub admin: Pubkey,
}

pub fn fulfil_update_spend_limits(params: &FulfilUpdateSpendLimitsParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);

    let accounts = crate::pyra_program::client::accounts::FulfilUpdateSpendLimits {
        spend_limits_order: params.spend_limits_order,
        vault,
        order_payer: params.order_payer,
        admin: params.admin,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::FulfilUpdateSpendLimits {}.data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
