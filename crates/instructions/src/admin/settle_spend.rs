use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

pub struct SettleSpendParams {
    pub admin: Pubkey,
    pub spend_settlement_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub amount_base_units: u64,
}

pub fn settle_spend(params: &SettleSpendParams) -> Instruction {
    let spend_hold = pyra_accounts::get_spend_hold();
    let spend_hold_spl =
        pyra_accounts::get_associated_token_address(&spend_hold, &params.mint, &params.token_program);
    let spend_settlement_account_spl = pyra_accounts::get_associated_token_address(
        &params.spend_settlement_account,
        &params.mint,
        &params.token_program,
    );

    let accounts = crate::pyra_program::client::accounts::SettleSpend {
        admin: params.admin,
        spend_settlement_account: params.spend_settlement_account,
        spend_settlement_account_spl,
        spend_hold,
        spend_hold_spl,
        mint: params.mint,
        token_program: params.token_program,
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::SettleSpend {
        amount_base_units: params.amount_base_units,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
