use solana_program::pubkey::Pubkey;

use crate::{ASSOCIATED_TOKEN_PROGRAM_ID, DRIFT_PROGRAM_ID, PYRA_PROGRAM_ID};

pub fn get_vault(owner: &Pubkey) -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(&[b"vault", owner.as_ref()], &PYRA_PROGRAM_ID);
    pubkey
}

pub fn get_deposit_address(owner: &Pubkey) -> Pubkey {
    let vault = get_vault(owner);
    let (pubkey, _) =
        Pubkey::find_program_address(&[b"deposit_address", vault.as_ref()], &PYRA_PROGRAM_ID);
    pubkey
}

pub fn get_spend_hold() -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(&[b"spend_hold"], &PYRA_PROGRAM_ID);
    pubkey
}

pub fn get_swap_ledger(mint: &Pubkey) -> Pubkey {
    let (pubkey, _) =
        Pubkey::find_program_address(&[b"swap_ledger", mint.as_ref()], &PYRA_PROGRAM_ID);
    pubkey
}

pub fn get_drift_user(owner: &Pubkey, sub_account_id: u16) -> Pubkey {
    let vault = get_vault(owner);
    let (pubkey, _) = Pubkey::find_program_address(
        &[
            b"user",
            vault.as_ref(),
            sub_account_id.to_le_bytes().as_ref(),
        ],
        &DRIFT_PROGRAM_ID,
    );
    pubkey
}

pub fn get_drift_user_stats(owner: &Pubkey) -> Pubkey {
    let vault = get_vault(owner);
    let (pubkey, _) =
        Pubkey::find_program_address(&[b"user_stats", vault.as_ref()], &DRIFT_PROGRAM_ID);
    pubkey
}

pub fn get_drift_state() -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(&[b"drift_state"], &DRIFT_PROGRAM_ID);
    pubkey
}

pub fn get_drift_signer() -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(&[b"drift_signer"], &DRIFT_PROGRAM_ID);
    pubkey
}

pub fn get_drift_spot_market_vault(market_index: u16) -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(
        &[b"spot_market_vault", market_index.to_le_bytes().as_ref()],
        &DRIFT_PROGRAM_ID,
    );
    pubkey
}

pub fn get_drift_spot_market(market_index: u16) -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(
        &[b"spot_market", market_index.to_le_bytes().as_ref()],
        &DRIFT_PROGRAM_ID,
    );
    pubkey
}

pub fn get_swap_ledger_v2(mint: &Pubkey) -> Pubkey {
    let (pubkey, _) =
        Pubkey::find_program_address(&[b"swap_ledger_v2", mint.as_ref()], &PYRA_PROGRAM_ID);
    pubkey
}

pub fn get_associated_token_address(
    wallet: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Pubkey {
    let (address, _) = Pubkey::find_program_address(
        &[wallet.as_ref(), token_program.as_ref(), mint.as_ref()],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    );
    address
}
