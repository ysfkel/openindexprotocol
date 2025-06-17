use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    issuance::{
        error::IssuanceError, instruction::ExecuteHookInstruction,
        pda::find_issuance_signer_address, state::IssuanceConfig,
    },
    openindex::{
        error::ProtocolError, instruction::ProtocolInstruction, pda::find_module_address,
        state::IndexMints,
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    let issuance_config_account = next_account_info(accounts_iter)?;
    let issuance_signer_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let module_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let openindex_program_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(signer.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        issuance_config_account.owner == program_id,
        IssuanceError::UnknownIssuanceConfigAccount.into()
    );

    let (issuance_signer_pda, issuance_signer_bump) = find_issuance_signer_address(program_id);

    require!(
        *issuance_signer_account.key == issuance_signer_pda,
        IssuanceError::IncorrectIssuanceSignerAccount.into()
    );

    let issuance_config = IssuanceConfig::try_from_slice(&issuance_config_account.data.borrow())?;

    require!(*openindex_program_account.key == issuance_config.openindex_program_id, IssuanceError::IllegalOpenIndexProgramId.into());

    require!(controller_account.owner == openindex_program_account.key, ProtocolError::UnknownControllerAccount.into());

    require!(mint_account.owner == openindex_program_account.key, ProtocolError::IllegalMintAccount.into());

    require!(index_account.owner == openindex_program_account.key, ProtocolError::UnknownIndexAccount.into());

    require!(index_mints_account.owner == openindex_program_account.key, ProtocolError::UnknownIndexMintsAccount.into());

    let (index_pda, index_bump) = find_index_address(openindex_program_account.key, controller_account.key, index_id);

    require!(
        *index_account.key == index_pda,
        ProtocolError::IncorrectIndexAccount.into()
    );

    // ! todo
    // execute hooks 
    // transfer components from user to vault 
    // make mint cpi call to openindex program
    // - move components transfers from openindex to issuance module - openindex core should only mint index tokens



    Ok(())
}
