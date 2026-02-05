use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::constants::SYSTEM_PROGRAM_ID;

pub struct ClearLegacyDepositAddressParams {
    pub owner: Pubkey,
    /// The mint of the legacy deposit address token account, if one exists.
    pub mint: Option<Pubkey>,
    /// The token program for the mint (e.g. Token or Token-2022).
    pub token_program: Pubkey,
}

pub fn clear_legacy_deposit_address(params: &ClearLegacyDepositAddressParams) -> Instruction {
    let vault = pyra_accounts::get_vault(&params.owner);
    let deposit_address = pyra_accounts::get_deposit_address(&params.owner);

    let owner_spl = params.mint.map(|m| {
        pyra_accounts::get_associated_token_address(&params.owner, &m, &params.token_program)
    });
    let deposit_address_spl = params.mint.map(|m| {
        pyra_accounts::get_associated_token_address(&deposit_address, &m, &params.token_program)
    });

    let accounts = crate::pyra_program::client::accounts::ClearLegacyDepositAddress {
        vault,
        owner: params.owner,
        owner_spl,
        deposit_address,
        deposit_address_spl,
        mint: params.mint,
        token_program: params.token_program,
        associated_token_program: pyra_accounts::ASSOCIATED_TOKEN_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::ClearLegacyDepositAddress {}.data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
