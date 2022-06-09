// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use solana_program::msg;
use wallet_instructions::{OnChainWallet, WalletInstruction, WalletResult};
use zeroed_store::ZeroedStore;

#[derive(Debug)]
pub struct InstructionProcessor {
    instruction: WalletInstruction,
    pub store: ZeroedStore<OnChainWallet>,
}

impl InstructionProcessor {
    pub fn new(instruction: WalletInstruction) -> Self {
        InstructionProcessor {
            instruction,
            store: ZeroedStore::<OnChainWallet>::new(),
        }
    }

    pub fn add_store(&mut self, store: ZeroedStore<OnChainWallet>) -> &mut Self {
        self.store = store;

        self
    }

    pub fn process(&mut self) -> WalletResult<&mut Self> {
        match self.instruction {
            WalletInstruction::AddCustodian(custodian) => {
                self.store.data.add_custodian(custodian)?;

                Ok(self)
            }
            _ => {
                msg!("INSTRUCTION NOT IMPLEMENTED");

                panic!()
            }
        }
    }
}
