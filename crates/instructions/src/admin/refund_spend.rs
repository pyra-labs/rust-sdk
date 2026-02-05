use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct RefundSpendParams {
    pub admin: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub amount_base_units: u64,
}

pub fn refund_spend(params: &RefundSpendParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let deposit_address = pyra_accounts::get_deposit_address(&params.owner);
    let deposit_address_spl = pyra_accounts::get_associated_token_address(
        &deposit_address,
        &params.mint,
        &params.token_program,
    );
    let spend_hold = pyra_accounts::get_spend_hold();
    let spend_hold_spl =
        pyra_accounts::get_associated_token_address(&spend_hold, &params.mint, &params.token_program);

    let accounts = crate::pyra_program::client::accounts::RefundSpend {
        admin: params.admin,
        vault,
        owner: params.owner,
        deposit_address,
        deposit_address_spl,
        spend_hold,
        spend_hold_spl,
        mint: params.mint,
        token_program: params.token_program,
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::RefundSpend {
        amount_base_units: params.amount_base_units,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
