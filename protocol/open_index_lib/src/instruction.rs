use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::system_program;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum ProtocolInstruction {
    // 0
    /// Initializes the Open Index Protocol account
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Owner account. Can be governance account
    ///   1. `[]` Protocol account
    ///   2. `[]` System Program account
    InitProtocol,
    InitController,

    InitControllerGlobalConfig {
        max_index_components: u32,
    },
    InitModule,
    CreateIndex,
    AddIndexComponents {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },
    Mint {
        index_id: u64,
        amount: u64,
    },
    Redeem,
    //..
}

impl ProtocolInstruction {
    pub fn init_protocol(program_id: Pubkey, caller: Pubkey,protocol_account: Pubkey) -> Instruction {
        let accounts = vec![
            AccountMeta::new(caller, true),
            AccountMeta::new(protocol_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        let instruction = Self::InitProtocol;
        let data = borsh::to_vec(&instruction).unwrap();
        Instruction { program_id, accounts, data}
    }

    pub fn init_controller(program_id: Pubkey, caller: Pubkey,protocol_account: Pubkey, controller_account: Pubkey) -> Instruction {
        let accounts = vec![
            AccountMeta::new(caller, true),
            AccountMeta::new(protocol_account, false),
            AccountMeta::new(controller_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        let instruction = Self::InitController;
        let data = borsh::to_vec(&instruction).unwrap();
        Instruction { program_id, accounts, data}
    }

    pub fn init_controller_global_config(program_id: Pubkey, caller: Pubkey,protocol_account: Pubkey, controller_global_config_account: Pubkey,max_index_components: u32) -> Instruction {
        let accounts = vec![
            AccountMeta::new(caller, true),
            AccountMeta::new(protocol_account, false),
            AccountMeta::new(controller_global_config_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        let instruction = Self::InitControllerGlobalConfig { max_index_components };
        let data = borsh::to_vec(&instruction).unwrap();
        Instruction { program_id, accounts, data}
    }
     
    #[allow(clippy::too_many_arguments)]
    pub fn create_index(program_id: Pubkey, caller: Pubkey, manager: Pubkey,index_account: Pubkey, mint_account: Pubkey,
        controller_account: Pubkey, controller_global_config_account:Pubkey
    ) -> Instruction {
        let accounts =  vec![
            AccountMeta::new(caller, true),
            AccountMeta::new(manager, false),
            AccountMeta::new(index_account, false),
            AccountMeta::new(mint_account, false),
            AccountMeta::new(controller_account, false),
            AccountMeta::new(controller_global_config_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ];
        let instruction = Self::CreateIndex;
        let data = borsh::to_vec(&instruction).unwrap();
        Instruction { program_id, accounts, data}
    }


    pub fn init_module(program_id: Pubkey, caller: Pubkey,protocol_account: Pubkey, module_signer_account: Pubkey, registered_module_account: Pubkey) -> Instruction {
        let accounts = vec![
            AccountMeta::new(caller, true),
            AccountMeta::new(protocol_account, false),
            AccountMeta::new(module_signer_account, false),
            AccountMeta::new(registered_module_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        let instruction = Self::InitModule;
        let data = borsh::to_vec(&instruction).unwrap();
        Instruction { program_id, accounts, data}
    }
}