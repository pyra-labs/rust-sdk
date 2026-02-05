use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::drift::{FulfilWithdrawParams, InitiateWithdrawParams};

pub struct FeePaymentParams {
    pub owner: Pubkey,
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub drift_market_index: u16,
    pub amount_base_units: u64,
    pub reduce_only: bool,
    /// The keypair for the withdraw order account. Must be a new keypair
    /// that signs the transaction.
    pub withdraw_order: Pubkey,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

pub fn fee_payment(params: &FeePaymentParams) -> Vec<Instruction> {
    vec![
        crate::drift::initiate_withdraw(&InitiateWithdrawParams {
            owner: params.owner,
            withdraw_order: params.withdraw_order,
            payer: params.admin,
            destination: params.admin,
            amount_base_units: params.amount_base_units,
            drift_market_index: params.drift_market_index,
            reduce_only: params.reduce_only,
        }),
        crate::drift::fulfil_withdraw(&FulfilWithdrawParams {
            withdraw_order: params.withdraw_order,
            owner: params.owner,
            order_payer: params.admin,
            admin: params.admin,
            payer: params.admin,
            destination: params.admin,
            mint: params.mint,
            token_program: params.token_program,
            drift_market_index: params.drift_market_index,
            amount_base_units: params.amount_base_units,
            destination_spl: None,
            remaining_accounts: params.remaining_accounts.clone(),
        }),
    ]
}
