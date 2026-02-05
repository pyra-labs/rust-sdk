use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::constants::{INSTRUCTIONS_SYSVAR_ID, SYSTEM_PROGRAM_ID};

pub struct StartSwapV2Params {
    pub owner: Pubkey,
    pub caller: Pubkey,
    pub payer: Pubkey,
    pub mint_substitute: Pubkey,
    pub mint_from: Pubkey,
    pub mint_to: Pubkey,
    pub token_program_substitute: Pubkey,
    pub token_program_from: Pubkey,
    pub token_program_to: Pubkey,
    pub drift_market_index_substitute: u16,
    pub amount_base_units_substitute: u64,
    pub drift_market_index_from: u16,
    pub amount_base_units_from: u64,
    pub drift_market_index_to: u16,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

pub fn start_swap_v2(params: &StartSwapV2Params) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let vault_spl_substitute = pyra_accounts::get_associated_token_address(
        &vault,
        &params.mint_substitute,
        &params.token_program_substitute,
    );
    let vault_spl_from = pyra_accounts::get_associated_token_address(
        &vault,
        &params.mint_from,
        &params.token_program_from,
    );
    let caller_spl_substitute = pyra_accounts::get_associated_token_address(
        &params.caller,
        &params.mint_substitute,
        &params.token_program_substitute,
    );
    let caller_spl_from = pyra_accounts::get_associated_token_address(
        &params.caller,
        &params.mint_from,
        &params.token_program_from,
    );
    let caller_spl_to = pyra_accounts::get_associated_token_address(
        &params.caller,
        &params.mint_to,
        &params.token_program_to,
    );
    let drift_user = pyra_accounts::get_drift_user(&params.owner, 0);
    let drift_user_stats = pyra_accounts::get_drift_user_stats(&params.owner);
    let drift_state = pyra_accounts::get_drift_state();
    let drift_spot_market_vault_substitute =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index_substitute);
    let drift_spot_market_vault_from =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index_from);
    let drift_signer = pyra_accounts::get_drift_signer();
    let ledger = pyra_accounts::get_swap_ledger_v2(&params.mint_from);

    let mut accounts = crate::pyra_program::client::accounts::StartSwapDriftV2 {
        vault,
        vault_spl_substitute,
        vault_spl_from,
        caller: params.caller,
        caller_spl_substitute,
        caller_spl_from,
        caller_spl_to,
        mint_substitute: params.mint_substitute,
        mint_from: params.mint_from,
        drift_user,
        drift_user_stats,
        drift_state,
        drift_spot_market_vault_substitute,
        drift_spot_market_vault_from,
        drift_signer,
        token_program_substitute: params.token_program_substitute,
        token_program_from: params.token_program_from,
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
        payer: params.payer,
        ledger,
        drift_program: pyra_accounts::DRIFT_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
        instructions_sysvar_account: INSTRUCTIONS_SYSVAR_ID,
    }
    .to_account_metas(None);
    accounts.extend_from_slice(&params.remaining_accounts);

    let data = crate::pyra_program::client::args::StartSwapDriftV2 {
        drift_market_index_substitute: params.drift_market_index_substitute,
        amount_base_units_substitute: params.amount_base_units_substitute,
        drift_market_index_from: params.drift_market_index_from,
        amount_base_units_from: params.amount_base_units_from,
        drift_market_index_to: params.drift_market_index_to,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
