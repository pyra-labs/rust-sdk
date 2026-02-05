use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::constants::{RENT_SYSVAR_ID, SYSTEM_PROGRAM_ID};

pub struct InitDriftParams {
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub sub_account_id: u16,
}

pub fn init_drift(params: &InitDriftParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let drift_user = pyra_accounts::get_drift_user(&params.owner, params.sub_account_id);
    let drift_user_stats = pyra_accounts::get_drift_user_stats(&params.owner);
    let drift_state = pyra_accounts::get_drift_state();

    let accounts = crate::pyra_program::client::accounts::InitDrift {
        vault,
        payer: params.payer,
        drift_user,
        drift_user_stats,
        drift_state,
        rent: RENT_SYSVAR_ID,
        drift_program: pyra_accounts::DRIFT_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::InitDrift {
        sub_account_id: params.sub_account_id,
    }
    .data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
