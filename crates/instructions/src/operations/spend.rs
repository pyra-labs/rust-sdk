use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::admin::{SettleSpendParams, SpendDriftParams};

pub struct SpendParams {
    pub owner: Pubkey,
    pub admin: Pubkey,
    pub spend_fee_destination: Pubkey,
    pub spend_settlement_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub drift_market_index: u16,
    pub spend_amount_base_units: u64,
    pub fee_amount_base_units: u64,
    /// Amount to settle (send from spend hold to settlement account).
    /// Pass 0 to skip the settle instruction.
    pub settle_amount_base_units: u64,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

pub fn spend(params: &SpendParams) -> Vec<Instruction> {
    let mut ixs = vec![crate::admin::spend_drift(&SpendDriftParams {
        owner: params.owner,
        admin: params.admin,
        spend_fee_destination: params.spend_fee_destination,
        mint: params.mint,
        token_program: params.token_program,
        drift_market_index: params.drift_market_index,
        spend_amount_base_units: params.spend_amount_base_units,
        fee_amount_base_units: params.fee_amount_base_units,
        remaining_accounts: params.remaining_accounts.clone(),
    })];

    if params.settle_amount_base_units > 0 {
        ixs.push(crate::admin::settle_spend(&SettleSpendParams {
            admin: params.admin,
            spend_settlement_account: params.spend_settlement_account,
            mint: params.mint,
            token_program: params.token_program,
            amount_base_units: params.settle_amount_base_units,
        }));
    }

    ixs
}
