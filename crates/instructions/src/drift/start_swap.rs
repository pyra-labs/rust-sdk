use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::constants::{INSTRUCTIONS_SYSVAR_ID, SYSTEM_PROGRAM_ID};

pub struct StartSwapParams {
    pub owner: Pubkey,
    pub caller: Pubkey,
    pub payer: Pubkey,
    pub mint_from: Pubkey,
    pub mint_to: Pubkey,
    pub token_program_from: Pubkey,
    pub token_program_to: Pubkey,
}

pub fn start_swap(params: &StartSwapParams) -> Instruction {
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

    let accounts = crate::pyra_program::client::accounts::StartSwapDrift {
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
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
        payer: params.payer,
        ledger,
        system_program: SYSTEM_PROGRAM_ID,
        instructions_sysvar_account: INSTRUCTIONS_SYSVAR_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::StartSwapDrift {}.data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
