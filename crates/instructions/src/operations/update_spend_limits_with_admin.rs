use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::user::{FulfilUpdateSpendLimitsParams, InitiateUpdateSpendLimitsParams};

pub struct UpdateSpendLimitsWithAdminParams {
    pub owner: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    /// The keypair for the spend limits order account. Must be a new keypair
    /// that signs the transaction.
    pub spend_limits_order: Pubkey,
    pub spend_limit_per_transaction: u64,
    pub spend_limit_per_timeframe: u64,
    pub next_timeframe_reset_timestamp: u64,
    pub timeframe_in_seconds: u64,
}

pub fn update_spend_limits_with_admin(
    params: &UpdateSpendLimitsWithAdminParams,
) -> Vec<Instruction> {
    vec![
        crate::user::initiate_update_spend_limits(&InitiateUpdateSpendLimitsParams {
            owner: params.owner,
            spend_limits_order: params.spend_limits_order,
            payer: params.payer,
            spend_limit_per_transaction: params.spend_limit_per_transaction,
            spend_limit_per_timeframe: params.spend_limit_per_timeframe,
            next_timeframe_reset_timestamp: params.next_timeframe_reset_timestamp,
            timeframe_in_seconds: params.timeframe_in_seconds,
        }),
        crate::user::fulfil_update_spend_limits(&FulfilUpdateSpendLimitsParams {
            spend_limits_order: params.spend_limits_order,
            owner: params.owner,
            order_payer: params.payer,
            admin: params.admin,
        }),
    ]
}
