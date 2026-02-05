use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::drift::{EndSwapV2Params, StartSwapV2Params};

pub struct SwapV2Instructions {
    pub start: Instruction,
    pub end: Instruction,
}

pub struct SwapV2Params {
    pub owner: Pubkey,
    pub caller: Pubkey,
    pub payer: Pubkey,
    pub mint_substitute: Pubkey,
    pub mint_from: Pubkey,
    pub mint_to: Pubkey,
    pub token_program_substitute: Pubkey,
    pub token_program_from: Pubkey,
    pub token_program_to: Pubkey,
    pub price_update_from: Pubkey,
    pub price_update_to: Pubkey,
    pub drift_market_index_substitute: u16,
    pub amount_base_units_substitute: u64,
    pub drift_market_index_from: u16,
    pub amount_base_units_from: u64,
    pub drift_market_index_to: u16,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

/// Build the start and end instructions for a V2 swap. The caller is
/// responsible for inserting external swap instructions between them.
pub fn build_swap_v2(params: &SwapV2Params) -> SwapV2Instructions {
    let start = crate::drift::start_swap_v2(&StartSwapV2Params {
        owner: params.owner,
        caller: params.caller,
        payer: params.payer,
        mint_substitute: params.mint_substitute,
        mint_from: params.mint_from,
        mint_to: params.mint_to,
        token_program_substitute: params.token_program_substitute,
        token_program_from: params.token_program_from,
        token_program_to: params.token_program_to,
        drift_market_index_substitute: params.drift_market_index_substitute,
        amount_base_units_substitute: params.amount_base_units_substitute,
        drift_market_index_from: params.drift_market_index_from,
        amount_base_units_from: params.amount_base_units_from,
        drift_market_index_to: params.drift_market_index_to,
        remaining_accounts: params.remaining_accounts.clone(),
    });

    let end = crate::drift::end_swap_v2(&EndSwapV2Params {
        owner: params.owner,
        caller: params.caller,
        payer: params.payer,
        mint_from: params.mint_from,
        mint_to: params.mint_to,
        mint_substitute: params.mint_substitute,
        token_program_to: params.token_program_to,
        token_program_substitute: params.token_program_substitute,
        price_update_from: params.price_update_from,
        price_update_to: params.price_update_to,
        drift_market_index_substitute: params.drift_market_index_substitute,
        drift_market_index_from: params.drift_market_index_from,
        drift_market_index_to: params.drift_market_index_to,
        remaining_accounts: params.remaining_accounts.clone(),
    });

    SwapV2Instructions { start, end }
}

/// Assemble a full V2 swap transaction: `[start, ...external_ixs, end]`.
pub fn assemble_swap_v2(
    swap: SwapV2Instructions,
    external_ixs: Vec<Instruction>,
) -> Vec<Instruction> {
    let mut ixs = Vec::new();
    ixs.push(swap.start);
    ixs.extend(external_ixs);
    ixs.push(swap.end);
    ixs
}
