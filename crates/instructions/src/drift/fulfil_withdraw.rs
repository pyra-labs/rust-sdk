use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct FulfilWithdrawParams {
    pub withdraw_order: Pubkey,
    pub owner: Pubkey,
    pub order_payer: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub destination: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub drift_market_index: u16,
    pub amount_base_units: u64,
    /// Optional: the destination SPL token account. Pass `None` to let the
    /// program create an ATA automatically.
    pub destination_spl: Option<Pubkey>,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

pub fn fulfil_withdraw(params: &FulfilWithdrawParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let vault_spl =
        pyra_accounts::get_associated_token_address(&vault, &params.mint, &params.token_program);
    let drift_user = pyra_accounts::get_drift_user(&params.owner, 0);
    let drift_user_stats = pyra_accounts::get_drift_user_stats(&params.owner);
    let drift_state = pyra_accounts::get_drift_state();
    let drift_spot_market_vault =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index);
    let drift_signer = pyra_accounts::get_drift_signer();

    let mut accounts = crate::pyra_program::client::accounts::FulfilWithdrawDrift {
        withdraw_order: params.withdraw_order,
        vault,
        order_payer: params.order_payer,
        admin: params.admin,
        payer: params.payer,
        destination: params.destination,
        destination_spl: params.destination_spl,
        vault_spl,
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

    let data = crate::pyra_program::client::args::FulfilWithdrawDrift {
        amount_base_units: params.amount_base_units,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
