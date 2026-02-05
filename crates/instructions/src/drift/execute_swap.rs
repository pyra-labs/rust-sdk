use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::constants::{INSTRUCTIONS_SYSVAR_ID, SYSTEM_PROGRAM_ID};

pub struct ExecuteSwapParams {
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

pub fn execute_swap(params: &ExecuteSwapParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let vault_spl_from = pyra_accounts::get_associated_token_address(
        &vault,
        &params.mint_from,
        &params.token_program_from,
    );
    let vault_spl_to = pyra_accounts::get_associated_token_address(
        &vault,
        &params.mint_to,
        &params.token_program_to,
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
    let ledger = pyra_accounts::get_swap_ledger(&params.mint_from);
    let drift_user = pyra_accounts::get_drift_user(&params.owner, 0);
    let drift_user_stats = pyra_accounts::get_drift_user_stats(&params.owner);
    let drift_state = pyra_accounts::get_drift_state();
    let drift_spot_market_vault_from =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index_from);
    let drift_spot_market_vault_to =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index_to);
    let drift_signer = pyra_accounts::get_drift_signer();

    let mut accounts = crate::pyra_program::client::accounts::ExecuteSwapDrift {
        owner: params.owner,
        vault,
        vault_spl_from,
        vault_spl_to,
        caller: params.caller,
        caller_spl_from,
        caller_spl_to,
        mint_from: params.mint_from,
        mint_to: params.mint_to,
        token_program_from: params.token_program_from,
        token_program_to: params.token_program_to,
        payer: params.payer,
        ledger,
        drift_user,
        drift_user_stats,
        drift_state,
        drift_spot_market_vault_from,
        drift_spot_market_vault_to,
        drift_signer,
        price_update_from: params.price_update_from,
        price_update_to: params.price_update_to,
        drift_program: pyra_accounts::DRIFT_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
        instructions_sysvar_account: INSTRUCTIONS_SYSVAR_ID,
    }
    .to_account_metas(None);
    accounts.extend_from_slice(&params.remaining_accounts);

    let data = crate::pyra_program::client::args::ExecuteSwapDrift {
        drift_market_index_from: params.drift_market_index_from,
        drift_market_index_to: params.drift_market_index_to,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
