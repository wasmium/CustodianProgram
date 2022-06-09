use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    log::sol_log,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use wallet_instructions::{OnChainWallet, WalletInstruction};
use zeroed_store::ZeroedStore;

mod processor;
pub use processor::*;
mod errors;
pub use errors::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let pda_account = next_account_info(accounts_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let wallet = match ZeroedStore::<OnChainWallet>::unpack(&pda_account.data.as_ref().borrow()) {
        Ok(store) => store,
        Err(error) => {
            let error_bytes = error.try_to_vec()?;
            msg!("OnChainWallet::> {:?}", error_bytes);

            return Err(ProgramError::Custom(ContractErrors::StoreError as u32));
        }
    };

    msg!("WALLET: {:?}", wallet);

    let instruction_data = WalletInstruction::try_from_slice(&instruction_data)?;
    msg!("INSTRUCTION: {:?}", instruction_data);
    let mut processed = InstructionProcessor::new(instruction_data);
    processed.add_store(wallet).process().unwrap();

    msg!("PROCESSED: {:?}", processed);
    processed
        .store
        .pack(&mut &mut pda_account.data.borrow_mut()[..])
        .unwrap();

    Ok(())
}
