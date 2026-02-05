use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct SpendDriftParams {
    pub owner: Pubkey,
    pub admin: Pubkey,
    pub spend_fee_destination: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub drift_market_index: u16,
    pub spend_amount_base_units: u64,
    pub fee_amount_base_units: u64,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

pub fn spend_drift(params: &SpendDriftParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let vault_spl =
        pyra_accounts::get_associated_token_address(&vault, &params.mint, &params.token_program);
    let spend_hold = pyra_accounts::get_spend_hold();
    let spend_hold_spl =
        pyra_accounts::get_associated_token_address(&spend_hold, &params.mint, &params.token_program);
    let spend_fee_destination_spl = pyra_accounts::get_associated_token_address(
        &params.spend_fee_destination,
        &params.mint,
        &params.token_program,
    );
    let drift_user = pyra_accounts::get_drift_user(&params.owner, 0);
    let drift_user_stats = pyra_accounts::get_drift_user_stats(&params.owner);
    let drift_state = pyra_accounts::get_drift_state();
    let drift_spot_market_vault =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index);
    let drift_signer = pyra_accounts::get_drift_signer();

    let mut accounts = crate::pyra_program::client::accounts::SpendDrift {
        vault,
        vault_spl,
        admin: params.admin,
        spend_hold,
        spend_hold_spl,
        spend_fee_destination: params.spend_fee_destination,
        spend_fee_destination_spl,
        mint: params.mint,
        drift_user,
        drift_user_stats,
        drift_state,
        drift_spot_market_vault,
        drift_signer,
        token_program: params.token_program,
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
        drift_program: pyra_accounts::DRIFT_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    accounts.extend_from_slice(&params.remaining_accounts);

    let data = crate::pyra_program::client::args::SpendDrift {
        drift_market_index: params.drift_market_index,
        spend_amount_base_units: params.spend_amount_base_units,
        fee_amount_base_units: params.fee_amount_base_units,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
