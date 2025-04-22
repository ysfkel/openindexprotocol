use crate::{find_module_signer_address, find_registered_module_address, get_protocol_pda, Setup};
use borsh::{BorshDeserialize, BorshSerialize};
use open_index_lib::{instruction::Instruction as OpenIndexInstruction, seeds::PROTOCOL_SEED};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct InitModuleTransaction {
    pub registered_module_pda: Pubkey,
    pub transaction: Transaction,
}

pub async fn init_module_transaction(_setup: &Setup) -> InitModuleTransaction {
    let fake_module_program_id = Keypair::new().pubkey();
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let (protocol_pda, _) = get_protocol_pda(program_id);

    let module_signer_pda = find_module_signer_address(&fake_module_program_id).0;
    let registered_module_pda = find_registered_module_address(program_id, &module_signer_pda).0;

    let initialize_ix = &OpenIndexInstruction::InitModule;
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();
    // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id.clone(),
            &initialize_ix,
            vec![
                AccountMeta::new(payer.pubkey().clone(), true),
                AccountMeta::new(protocol_pda, false),
                AccountMeta::new(module_signer_pda, false),
                AccountMeta::new(registered_module_pda, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitModuleTransaction {
        registered_module_pda,
        transaction,
    }
}
