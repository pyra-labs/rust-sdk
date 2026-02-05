use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

pub struct CloseUserParams {
    pub owner: Pubkey,
    pub admin: Pubkey,
}

pub fn close_user(params: &CloseUserParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);

    let accounts = crate::pyra_program::client::accounts::CloseUser {
        vault,
        admin: params.admin,
        drift_program: pyra_accounts::DRIFT_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::CloseUser {}.data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
