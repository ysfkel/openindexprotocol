use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    issuance::{
        error::IssuanceError, pda::find_issuance_config_address, seeds::ISSUANCE_CONFIG_SEED,
        state::IssuanceConfig,
    },
    openindex::{error::ProtocolError, pda::create_protocol_address, state::Protocol},
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn process_init_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    hooks: Vec<Pubkey>,
) -> ProgramResult {
    let account_info = &mut accounts.iter();
    let signer = next_account_info(account_info)?;
    let protocol_account = next_account_info(account_info)?;
    let issuance_config_account = next_account_info(account_info)?;
    let openindex_program_account = next_account_info(account_info)?;
    let system_program_account = next_account_info(account_info)?;

    require!(signer.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        issuance_config_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized
    );

    let protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())?;

    let protocol_pda = create_protocol_address(openindex_program_account.key, protocol.bump)?;

    require!(
        *protocol_account.key == protocol_pda,
        ProtocolError::IncorrectProtocolAccount.into()
    );

    require!(
        signer.key == &protocol.owner,
        ProtocolError::OnlyProtocolOwner.into()
    );

    let (issuance_config_pda, issuance_pda_bump) =
        find_issuance_config_address(openindex_program_account.key);

    require!(
        *issuance_config_account.key == issuance_config_pda,
        IssuanceError::IncorrectIssuanceConfigAccount.into()
    );

    let rent = Rent::get()?;
    let space = IssuanceConfig::calc_len(hooks.len());
    let lamports = rent.minimum_balance(space);

    invoke_signed(
        &system_instruction::create_account(
            &signer.key,
            &issuance_config_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            signer.clone(),
            issuance_config_account.clone(),
            system_program_account.clone(),
        ],
        &[&[ISSUANCE_CONFIG_SEED, &[issuance_pda_bump]]],
    )?;

    if !hooks.is_empty() {
        let issuance_config = IssuanceConfig::new(hooks, issuance_pda_bump);
        issuance_config.serialize(&mut &mut issuance_config_account.data.borrow_mut()[..])?;
    }

    Ok(())
}
