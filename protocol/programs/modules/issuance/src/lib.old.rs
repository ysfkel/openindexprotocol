use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{entrypoint, ProgramResult},
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::instructions::{self, get_instruction_relative, ID as INSTRUCTIONS_SYSVAR_ID},
};
use thiserror::Error;
// use open_index::instruction::Instruction as OpenIndexInstruction;
entrypoint!(process_instruction);

#[derive(BorshSerialize)]
pub enum OpenIndexInstruction {
    InitProtocol,
    InitController,

    InitControllerGlobalConfig {
        max_index_components: u32,
    },
    InitModule,
    CreateIndex {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },
    Mint,
    Redeem,
    //..
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ProtocolError {
    #[error("Error:Invalid module account")]
    InvalidModuleAccount,
    #[error("Error:Invalid registered module account")]
    InvalidRegisredModuleAccount,
}

impl From<ProtocolError> for ProgramError {
    fn from(e: ProtocolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
pub const MODULE_SEED: &[u8] = b"open_index_module";
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let module_account = next_account_info(accounts_iter)?;
    let registered_module_account = next_account_info(accounts_iter)?;
    let open_index = next_account_info(accounts_iter)?;

    let (signer_pda, bump) = Pubkey::find_program_address(&[program_id.as_ref()], program_id);

    if *module_account.key != signer_pda {
        msg!(
            "Invalid module_account {:?} !=signer_pda {:?}",
            module_account.key,
            signer_pda
        );
        return Err(ProtocolError::InvalidModuleAccount.into());
    }

    msg!("MODULE:: Module pda {:?}", signer_pda);

    let (registered_module_pda, registered_module_bump) = Pubkey::find_program_address(
        &[&MODULE_SEED, &module_account.key.as_ref()],
        open_index.key,
    );

    if *registered_module_account.key != registered_module_pda {
        msg!(
            "Invalid registered_module_account {:?} !=registered_module_pda {:?}",
            registered_module_account.key,
            registered_module_pda
        );
        return Err(ProtocolError::InvalidRegisredModuleAccount.into());
    }

    // Prepare instruction data
    let initialize_ix = &OpenIndexInstruction::Mint;
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();

    // Prepare CPI instruction
    let cpi_accounts = vec![
        AccountMeta::new_readonly(module_account.key.clone(), true),
        AccountMeta::new_readonly(registered_module_account.key.clone(), true),
    ];

    let cpi_instruction = Instruction {
        program_id: *open_index.key,
        accounts: cpi_accounts,
        data: initialize_ix_data,
    };

    // Invoke the CPI
    invoke_signed(
        &cpi_instruction,
        &[module_account.clone(), registered_module_account.clone()], // Pass the actual AccountInfo references
        &[&[program_id.as_ref(), &[bump]]],
    )?;

    msg!("openindex cpi sent");
    Ok(())
}
