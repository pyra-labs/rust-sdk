use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

pub struct CancelWithdrawParams {
    pub withdraw_order: Pubkey,
    pub owner: Pubkey,
    pub order_payer: Pubkey,
}

pub fn cancel_withdraw(params: &CancelWithdrawParams) -> Instruction {
    let accounts = crate::pyra_program::client::accounts::CancelWithdrawDrift {
        withdraw_order: params.withdraw_order,
        owner: params.owner,
        order_payer: params.order_payer,
    }
    .to_account_metas(None);

    let data = crate::pyra_program::client::args::CancelWithdrawDrift {}.data();

    Instruction {
        program_id: pyra_accounts::PYRA_PROGRAM_ID,
        accounts,
        data,
    }
}
