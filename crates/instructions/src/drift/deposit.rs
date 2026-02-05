use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct DepositParams {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub payer: Pubkey,
    pub drift_market_index: u16,
    /// Optional: the deposit address SPL token account. Pass `None` if no
    /// legacy deposit address token account exists.
    pub deposit_address_spl: Option<Pubkey>,
    /// Drift remaining accounts (oracle + spot market accounts).
    pub remaining_accounts: Vec<AccountMeta>,
}

pub fn deposit(params: &DepositParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let vault_spl =
        pyra_accounts::get_associated_token_address(&vault, &params.mint, &params.token_program);
    let deposit_address = pyra_accounts::get_deposit_address(&params.owner);
    let drift_user = pyra_accounts::get_drift_user(&params.owner, 0);
    let drift_user_stats = pyra_accounts::get_drift_user_stats(&params.owner);
    let drift_state = pyra_accounts::get_drift_state();
    let drift_spot_market_vault =
        pyra_accounts::get_drift_spot_market_vault(params.drift_market_index);

    let mut accounts = crate::pyra_program::client::accounts::DepositDrift {
        vault,
        vault_spl,
        deposit_address,
        deposit_address_spl: params.deposit_address_spl,
        mint: params.mint,
        payer: params.payer,
        drift_user,
        drift_user_stats,
        drift_state,
        drift_spot_market_vault,
        token_program: params.token_program,
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
        drift_program: pyra_accounts::DRIFT_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    accounts.extend_from_slice(&params.remaining_accounts);

    let data = crate::pyra_program::client::args::DepositDrift {
        drift_market_index: params.drift_market_index,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
