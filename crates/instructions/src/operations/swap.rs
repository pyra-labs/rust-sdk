use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::drift::{ExecuteSwapParams, StartSwapParams};

pub struct SwapInstructions {
    pub start: Instruction,
    pub end: Instruction,
}

pub struct SwapParams {
    pub owner: Pubkey,
    pub caller: Pubkey,
    pub payer: Pubkey,
    pub mint_from: Pubkey,
    pub mint_to: Pubkey,
    pub token_program_from: Pubkey,
    pub token_program_to: Pubkey,
    pub price_update_from: Pubkey,
    pub price_update_to: Pubkey,
    pub drift_market_index_from: u16,
    pub drift_market_index_to: u16,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

/// Build the start and end instructions for a V1 swap. The caller is
/// responsible for inserting external swap instructions between them.
pub fn build_swap(params: &SwapParams) -> SwapInstructions {
    let start = crate::drift::start_swap(&StartSwapParams {
        owner: params.owner,
        caller: params.caller,
        payer: params.payer,
        mint_from: params.mint_from,
        mint_to: params.mint_to,
        token_program_from: params.token_program_from,
        token_program_to: params.token_program_to,
    });

    let end = crate::drift::execute_swap(&ExecuteSwapParams {
        owner: params.owner,
        caller: params.caller,
        payer: params.payer,
        mint_from: params.mint_from,
        mint_to: params.mint_to,
        token_program_from: params.token_program_from,
        token_program_to: params.token_program_to,
        price_update_from: params.price_update_from,
        price_update_to: params.price_update_to,
        drift_market_index_from: params.drift_market_index_from,
        drift_market_index_to: params.drift_market_index_to,
        remaining_accounts: params.remaining_accounts.clone(),
    });

    SwapInstructions { start, end }
}

/// Assemble a full swap transaction: `[start, ...external_ixs, end]`.
pub fn assemble_swap(
    swap: SwapInstructions,
    external_ixs: Vec<Instruction>,
) -> Vec<Instruction> {
    let mut ixs = Vec::new();
    ixs.push(swap.start);
    ixs.extend(external_ixs);
    ixs.push(swap.end);
    ixs
}
