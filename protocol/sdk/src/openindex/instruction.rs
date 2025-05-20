use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use spl_associated_token_account::get_associated_token_address_with_program_id;

use super::pda::find_component_address;
use super::pda::find_component_vault_address;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum ProtocolInstruction {

    /// 0. **InitProtocol**
    ///
    /// Creates **the one-and-only** `protocol_account` PDA for the entire
    /// Open-Index Protocol.  
    /// This instruction **must be executed once, and only once, before any
    /// controllers, modules, or indexes can be created.**
    ///
    /// ### Behaviour
    /// * Derives the PDA at `find_protocol_address(b"protocol")`.
    /// * Allocates `Protocol::LEN` bytes, funded to rent-exemption.
    /// * Writes an initial `Protocol` struct with:
    ///     * `authority`   = `signer` (can later be transferred by governance)
    ///     * `bump`        = PDA bump seed
    ///     * All config fields zero-initialised / default.
    ///
    /// ### Accounts
    /// 0. `[signer]`            signer                   – the wallet that pays rent and becomes protocol authority  
    /// 1. `[writable]`          protocol_account         – PDA derived with seed `b"protocol"`  
    /// 2. `[]`                  system_program_account   – `solana_program::system_program`
    ///
    /// ### Instruction data
    /// * _none_ (the call is self-contained)
    ///
    /// ### Fails with
    /// * `AccountAlreadyInitialized` if `protocol_account` already has lamports  
    /// * `IncorrectProtocolAccount`  if the PDA derivation doesn’t match  
    /// * `MissingRequiredSignature`  if `signer` did not sign
    InitProtocol,
    
    /// 1. **InitController**
    ///
    /// Creates a new **controller** under the protocol.  
    /// A controller is an administrative domain that can own multiple
    /// indexes and has its own configuration and governance.
    ///
    /// ### Behaviour
    /// * Deserialises the `protocol_account` to verify the protocol is
    ///   already initialised.  
    /// * Increments `protocol.next_controller_id`, then derives the PDA  
    ///   `find_controller_address(program_id, controller_id)`.  
    /// 
    /// * Allocates a `Controller::LEN` account at that PDA, rent-exempt.  
    /// * Writes an initial `Controller` struct with:
    ///     * `id`         = incremental `controller_id`  
    ///     * `authority`  = `signer` (may be transferred later)  
    ///     * `bump`       = PDA bump seed  
    ///     * `config_set` = false (must call `InitControllerGlobalConfig`)  
    /// 
    /// * Persists the updated `protocol_account` (so the next call will get
    ///   the next unique controller ID).
    ///
    /// ### Accounts
    /// 0. `[signer]`            signer                   – wallet that pays rent and becomes controller authority  
    /// 1. `[writable]`          protocol_account         – protocol PDA (must already exist)  
    /// 2. `[writable]`          controller_account       – PDA derived from (`b"controller"`, controller_id)  
    /// 3. `[]`                  system_program           – `solana_program::system_program`
    ///
    /// ### Instruction data
    /// * _none_ (all data implicit)
    ///
    /// ### Fails with
    /// * `ProtocolNotInitialized`        if protocol_account is still zeroed  
    /// * `IncorrectProtocolAccount`      if PDA derivation mismatches provided account  
    /// * `AccountAlreadyInitialized`     if controller_account is non-empty  
    /// * `MissingRequiredSignature`      if signer did not sign

    InitController,

   /// 2. **InitControllerGlobalConfig**
    ///
    /// Creates the singleton **controller_global_config_account** that
    /// stores limits enforced uniformly across **all** controllers  
    /// (e.g. the maximum number of component mints an index may hold).
    ///
    /// This account is initialised **once** by the protocol owner and then
    /// referenced read-only by every controller and index instruction.
    ///
    /// ### Behaviour
    /// * Verifies `max_index_components > 0`; otherwise returns
    ///   `InvalidMaxIndexComponents`.
    /// 
    /// * Confirms `protocol_account` is the correct PDA and already
    ///   initialised.  
    /// * Ensures the caller (`signer`) is exactly `protocol.owner`.  
    /// 
    /// * Derives PDA `find_controller_global_config_address()` and creates
    ///   an account of size `ControllerGlobalConfig::LEN`, funded to
    ///   rent-exemption.  
    /// 
    /// * Serialises a `ControllerGlobalConfig` with:
    ///     * `max_index_components`  – the supplied hard cap  
    ///     * `bump`                  – PDA bump seed
    ///
    /// ### Accounts
    /// 0. `[signer]`            signer                             – **must** be the protocol owner  
    /// 1. `[]`                  protocol_account                   – protocol PDA (already exists)  
    /// 2. `[writable]`          controller_global_config_account   – PDA derived from `b"controller_global_config"`  
    /// 3. `[]`                  system_program_account             – `solana_program::system_program`
    ///
    /// ### Instruction data
    /// * `max_index_components: u32`
    ///
    /// ### Fails with
    /// * `InvalidMaxIndexComponents`        if the provided max is zero  
    /// * `ProtocolNotInitialized`           if protocol_account is still zeroed  
    /// * `OnlyProtocolOwner`                if signer ≠ protocol.owner  
    /// * `IncorrectControllerGlobalConfigAccount` if PDA derivation mismatches  
    /// * `AccountAlreadyInitialized`        if controller_global_config_account already has lamports  
    /// * `MissingRequiredSignature`         if signer did not sign
    InitControllerGlobalConfig {
        max_index_components: u32,
    },

    /// 3. **InitModule**
    ///
    /// Registers an **external program (“module”)** that is authorised to
    /// call *into* the Open-Index Protocol via CPI.  
    ///
    /// After registration, the module can execute privileged instructions
    /// such as:
    /// * trading component tokens to rebalance an index,
    /// * auto-compounding staking rewards,
    /// * routing collateral through DeFi strategies,
    /// * or any custom logic that ultimately invokes the core program’s
    ///   `Mint`, `Redeem`, `AddIndexComponents`, etc. instructions.
    ///
    /// **Important:** Controllers and indexes never invoke the module.
    /// Instead, the module decides when to act and invokes the protocol,
    /// subject to whatever auth-checks you implement inside the module.
    ///
    /// ### Behaviour
    /// * Confirms the protocol is initialised and that the caller (`signer`)
    ///   is exactly `protocol.owner`.  
    /// * Derives
    ///   `find_registered_module_address(program_id, module_signer_account)`  
    ///   and creates the PDA with size `Module::LEN`, rent-exempt.  
    /// 
    /// * Serialises `Module { is_active: true, bump }` so the protocol can
    ///   later verify the caller is a registered module.
    ///
    /// ### Accounts
    /// 0. `[signer]`            signer                       – **must** be the protocol owner  
    /// 1. `[]`                  protocol_account             – protocol PDA  
    /// 2. `[]`                  module_signer_account        – the external program’s signer / upgrade authority  
    /// 3. `[writable]`          registered_module_account    – PDA derived from (`b"module"`, module_signer_account)  
    /// 4. `[]`                  system_program               – `solana_program::system_program`
    ///
    /// ### Instruction data
    /// * _none_
    ///
    /// ### Fails with
    /// * `ProtocolNotInitialized`           if protocol_account not yet set  
    /// * `OnlyProtocolOwner`                if signer ≠ protocol.owner  
    /// * `IncorrectModuleAccount`           if PDA derivation mismatches provided account  
    /// * `AccountAlreadyInitialized`        if registered_module_account already has lamports  
    /// * `MissingRequiredSignature`         if signer did not sign
    InitModule,
    
    /// 4. **CreateIndex**
    ///
    /// Deploys a **new index** under an existing controller:
    ///
    /// 1. Allocates an `index_account` PDA that stores metadata  
    /// 2. Allocates the SPL `mint_account` for the index token  
    /// 3. Sets the mint’s authority + freeze authority to an internal PDA  
    /// 4. Records the chosen `manager` (a delegate that can later add
    ///    components, rebalance, etc.)  
    /// 5. Bumps `controller.next_index_id`
    ///
    /// ### Behaviour
    /// * Confirms `controller_account.owner == signer` (only the controller
    ///   owner can create new indexes).  
    /// * Verifies `controller_global_config_account` is initialised, thereby
    ///   enforcing protocol-wide limits (e.g. `max_index_components`).  
    /// 
    /// * Derives two PDAs from the *next* `index_id`:  
    ///     * `index_account`  (seed `b"index"`)  
    ///     * `mint_account`   (seed `b"index_mint"`)  
    /// 
    /// * Allocates both accounts rent-exempt and initialises the mint with
    ///   `decimals = 9`.  
    /// 
    /// * Serialises an `Index { id, owner = signer, manager, bump }`.  
    /// * Serialises the updated `Controller`, so the next call gets a fresh
    ///   `index_id`.
    ///
    /// ### Accounts
    /// 0. `[signer]`            signer                             – **must** be `controller.owner` and pays rent  
    /// 1. `[]`                  manager                            – delegate that will manage the index  
    /// 2. `[writable]`          index_account                      – PDA (`b"index"`, controller_account, index_id)  
    /// 3. `[writable]`          mint_account                       – PDA (`b"index_mint"`, controller_account, index_id)  
    /// 4. `[writable]`          controller_account                 – controller PDA (mutated to bump next_index_id)  
    /// 5. `[]`                  controller_global_config_account   – global config PDA (read-only)  
    /// 6. `[]`                  system_program_account             – `solana_program::system_program`  
    /// 7. `[]`                  token_program_account              – `spl_token::id()`
    ///
    /// ### Instruction data
    /// * _none_ (all data is implicit)
    ///
    /// ### Fails with
    /// * `OnlyControllerOwner`                 if signer ≠ controller.owner  
    /// * `ControllerGlobalConfigNotInitialized` if global config is zeroed  
    /// * `IncorrectIndexAccount` / `IncorrectMintAccount`
    ///   if PDA derivations don’t match supplied accounts  
    /// * `AccountAlreadyInitialized`
    ///   if `index_account` or `mint_account` already carry lamports  
    /// * `MissingRequiredSignature`            if signer did not sign
    CreateIndex,

    /// 5. **AddIndexComponents**
    ///
    /// Registers the initial set of component mints for an index and
    /// creates on-chain bookkeeping for each component:
    ///
    /// 1. Creates `index_mints_account` PDA that stores the ordered list of
    ///    component mints (vector of `Pubkey`).  
    /// 
    /// 2. For **each** component listed in `mints`:
    ///    * Allocates a `component_account` PDA holding the per-component
    ///      metadata (`amount`, mint, bumps).  
    ///    * Allocates a `vault_pda` and its **associated token account**
    ///      (`vault_ata`) to custody that component’s tokens.  
    ///    * Records `amounts[i]` as the units backing **one** index token.
    ///
    /// After this instruction succeeds, the index can be minted/redeemed
    /// because the program now knows exactly which mints and quantities
    /// constitute one unit of the index.
    ///
    /// ### Behaviour
    /// * Checks `signer == controller.owner`; only controller owner may add
    ///   components.  
    /// * Enforces `mints.len() > 0` and that it does not exceed
    ///   `controller_global_config.max_index_components`.  
    /// * Requires `mints.len() == amounts.len()`.  
    /// * Derives `index_mints_account` PDA, allocates it rent-exempt with
    ///   size `IndexMints::calc_len(mints.len())`, and serialises
    ///   `IndexMints { mints, bump }`.  
    /// 
    /// * Iterates over each `(mint, amount)` pair, deriving:
    ///     * `component_account`  (`b"component"`, index_account, mint)  
    ///     * `vault_pda` & `vault_ata`       (`b"component_vault"`, …)  
    ///   Allocates the component account and creates the vault ATA via
    ///   `spl_associated_token_account::create`.  
    ///   Serialises `Component { units = amount, mint, bumps }`.
    ///
    /// ### Accounts
    /// 0. `[signer]`            signer                             – **must** be `controller.owner`  
    /// 1. `[]`                  index_account                      – index PDA  
    /// 2. `[writable]`          index_mints_account                – PDA (`b"index_mints"`, controller_account, index_id) (created)  
    /// 3. `[]`                  controller_account                 – controller PDA  
    /// 4. `[]`                  controller_global_config_account   – global config PDA (read-only)  
    /// 5. `[]`                  system_program_account             – `solana_program::system_program`  
    /// 6. `[]`                  associated_token_program_account   – `spl_associated_token_account::id()`  
    /// 7. `[]`                  token_program_account              – `spl_token::id()`
    ///
    /// ### Per-component bundle (repeated *N* = `mints.len()` times)
    /// * `[writable]` `mint_account[i]`            – the SPL mint in `mints[i]`  
    /// * `[writable]` `component_account[i]`       – PDA (`b"component"`, index_account, mint) (created)  
    /// * `[]`         `vault_pda[i]`               – PDA (`b"component_vault"`, …)  
    /// * `[writable]` `vault_ata[i]`               – ATA owned by `vault_pda[i]`
    ///
    /// Total accounts = 8 + *N* × 4
    ///
    /// ### Instruction data
    /// * `mints:   Vec<Pubkey>` – ordered list of component mints  
    /// * `amounts: Vec<u64>`    – component units per **one** index token
    ///
    /// ### Fails with
    /// * `OnlyControllerOwner`                 if signer ≠ controller.owner  
    /// * `NoMintsProvided`                     if `mints` is empty  
    /// * `MaxIndexComponentsExceeded`          if `mints.len()` exceeds global cap  
    /// * `MintsAmountsLenMismatch`             if lengths differ  
    /// * `InvalidMintAccount`                  if a supplied mint_account ≠ `mints[i]`  
    /// * `IncorrectComponentAccount`, `IncorrectVaultAccount`,  
    ///   `IncorrectVaultATA`                   if PDA derivations mismatch  
    /// * `AccountAlreadyInitialized`           if any PDA already holds lamports  
    /// * `MissingRequiredSignature`            if signer did not sign
    AddIndexComponents {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },

 
    /// 6. **Mint**
    ///
    /// Mints `amount` of **index tokens** to the signer and simultaneously
    /// transfers the proportional quantity of each component token from the
    /// signer into the protocol vaults.
    ///
    /// For every component `i`:
    ///
    /// ```text
    /// signer ATA (component)  ──►  vault_ata[i]
    ///                         +   index mint_account ──► signer ATA (index)
    /// ```
    ///
    /// ### Behaviour
    /// * Requires `amount > 0`.  
    /// * Verifies that `index_mints_account` is the correct PDA and
    ///   deserialises it to obtain the ordered component-mint list `mints`.  
    /// 
    /// * For each component:  
    ///     * Checks PDA correctness for `component_account`, `vault_pda`,
    ///       `vault_ata`.  
    ///     * Calculates `component_amount = amount × component.units`.  
    ///     * Executes `spl_token::transfer` from the signer’s
    ///       `component_token_account` to the vault’s ATA.  
    /// * Verifies `token_account.mint == mint_account`.  
    /// * Executes `spl_token::mint_to` (CPI, signed by
    ///   `mint_authority_pda`) to credit `amount` index tokens to
    ///   `token_account`.
    ///
    /// ### Static accounts (first 8)
    /// 0. `[signer]`            signer                             – caller providing components  
    /// 1. `[]`                  controller_account                 – controller PDA  
    /// 2. `[writable]`          mint_account                       – index SPL mint  
    /// 3. `[]`                  mint_authority_account             – PDA that signs `mint_to`  
    /// 4. `[]`                  index_account                      – index PDA  
    /// 5. `[]`                  index_mints_account                – PDA holding ordered component mints  
    /// 6. `[writable]`          token_account                      – signer’s ATA for the **index mint**  
    /// 7. `[]`                  token_program_account              – `spl_token::id()`
    ///
    /// ### Per-component bundle (repeated *N* = `mints.len()` times)
    /// * `[]` `component_mint_account[i]`      – SPL mint of component *i*  
    /// * `[]`         `component_account[i]`           – component metadata PDA  
    /// * `[]` `vault_pda[i]`                   – PDA owning the vault ATA  
    /// * `[writable]` `vault_ata[i]`                   – ATA holding component *i* inside vault  
    /// * `[writable]` `component_token_account[i]`     – signer’s ATA for component *i*
    ///
    /// Total accounts = 8 + *N* × 5
    ///
    /// ### Instruction data
    /// * `index_id: u64` – index identifier inside controller  
    /// * `amount:   u64` – number of index tokens to mint
    ///
    /// ### Fails with
    /// * `AmountMustBeGreaterThanZero`          if `amount == 0`  
    /// * `UnknownControllerAccount`             if controller_account.owner ≠ program_id  
    /// * `UnknownIndexAccount`                  if index_account.owner ≠ program_id  
    /// * `IncorrectIndexMintsAccount`           if supplied PDA mismatches derivation  
    /// * `IncorrectMintAuthority`               if mint_authority_account ≠ derived PDA  
    /// * `IncorrectComponentAccount` / `IncorrectVaultAccount` /
    ///   `IncorrectVaultATA`                    if any PDA derivation mismatches  
    /// * `ComponentNotInitialized`              if a component_account is zeroed  
    /// * `InvalidMintAccount`                   if `token_account.mint` ≠ mint_account  
    /// * `ArithmeticOverflow`                   on `amount × component.units`  
    /// * `MissingRequiredSignature`             if signer did not sign
    Mint {
        index_id: u64,
        amount: u64,
    },

    /// 7. **Redeem**
    ///
    /// Burns `amount` of index tokens from the signer and returns the
    /// corresponding quantities of each underlying component token from the
    /// protocol vaults back to the signer.
    ///
    /// For every component `i`:
    ///
    /// ```text
    /// vault_ata[i]  ──►  signer ATA (component)
    /// signer ATA (index)  ──►  burned
    /// ```
    ///
    /// ### Behaviour
    /// * Requires `amount > 0`.  
    /// * Deserialises `index_mints_account` to obtain the ordered component
    ///   list `mints`.  
    /// 
    /// * For each component *i*:  
    ///     * Checks PDA correctness for `component_account[i]`, `vault_pda[i]`,
    ///       `vault_ata[i]`.  
    ///     * Asserts `component_mint_account[i] == mints[i]`.  
    ///     * Calculates `component_amount = amount × component.units`.  
    ///     * Executes `spl_token::transfer` from the vault’s ATA to the
    ///       signer’s `component_token_account[i]` (CPI, signed by
    ///       `vault_pda[i]`).  
    /// 
    /// * Executes `spl_token::burn` to destroy `amount` index tokens from the
    ///   signer’s `token_account`.
    ///
    /// ### Static accounts (first 8)
    /// 0. `[signer]`            signer                             – caller receiving components  
    /// 1. `[]`                  controller_account                 – controller PDA  
    /// 2. `[writable]`          mint_account                       – index SPL mint (read-only)  
    /// 3. `[]`                  mint_authority_account             – PDA used only for PDA check  
    /// 4. `[]`                  index_account                      – index PDA  
    /// 5. `[]`                  index_mints_account                – PDA holding ordered component mints  
    /// 6. `[writable]`          token_account                      – signer’s ATA for the **index mint** (debited & burned)  
    /// 7. `[]`                  token_program_account              – `spl_token::id()`
    ///
    /// ### Per-component bundle (repeated *N* = `mints.len()` times)
    /// * `[]` `component_mint_account[i]`      – SPL mint of component *i*  
    /// * `[]`         `component_account[i]`           – component metadata PDA  
    /// * `[]` `vault_pda[i]`                   – PDA owning the vault ATA (signs transfer)  
    /// * `[writable]` `vault_ata[i]`                   – ATA holding component *i* inside vault (debited)  
    /// * `[writable]` `component_token_account[i]`     – signer’s ATA for component *i* (credited)
    ///
    /// Total accounts = 8 + *N* × 5
    ///
    /// ### Instruction data
    /// * `index_id: u64` – index identifier inside controller  
    /// * `amount:   u64` – number of index tokens to redeem (burn)
    ///
    /// ### Fails with
    /// * `AmountMustBeGreaterThanZero`          if `amount == 0`  
    /// * `UnknownControllerAccount`             if controller_account.owner ≠ program_id  
    /// * `UnknownIndexAccount`                  if index_account.owner ≠ program_id  
    /// * `IncorrectIndexMintsAccount`           if supplied PDA mismatches derivation  
    /// * `IncorrectMintAuthority`               if mint_authority_account ≠ derived PDA  
    /// * `InvalidMintAccount`                   if a component mint mismatch occurs  
    /// * `IncorrectComponentAccount` / `IncorrectVaultAccount` /
    ///   `IncorrectVaultATA`                    if any PDA derivation mismatches  
    /// * `ArithmeticOverflow`                   on `amount × component.units`  
    /// * `MissingRequiredSignature`             if signer did not sign
    
    Redeem {
        index_id: u64,
        amount: u64,
    },
}

pub fn init_protocol_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new(protocol_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitProtocol;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_controller_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
    controller_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new(protocol_account, false),
        AccountMeta::new(controller_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitController;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_controller_global_config_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
    controller_global_config_account: Pubkey,
    max_index_components: u32,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(protocol_account, false),
        AccountMeta::new(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitControllerGlobalConfig {
        max_index_components,
    };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_module_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
    module_signer_account: Pubkey,
    registered_module_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(protocol_account, false),
        AccountMeta::new_readonly(module_signer_account, false),
        AccountMeta::new(registered_module_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitModule;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_index_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    manager: Pubkey,
    index_account: Pubkey,
    mint_account: Pubkey,
    controller_account: Pubkey,
    controller_global_config_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(manager, false),
        AccountMeta::new(index_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new(controller_account, false),
        AccountMeta::new_readonly(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];
    let instruction = ProtocolInstruction::CreateIndex;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn add_index_components_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    controller_account: Pubkey,
    controller_global_config_account: Pubkey,
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new(index_mints_data_account, false),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new_readonly(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];

    let instruction = ProtocolInstruction::AddIndexComponents { amounts, mints };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn add_index_components_instruction_with_dynamic_accounts(
    program_id: Pubkey,
    caller: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    controller_account: Pubkey,
    controller_global_config_account: Pubkey,
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new(index_mints_data_account, false),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new_readonly(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];

    for mint in mints.iter() {
        let (component_pda, _) = find_component_address(&program_id, &index_account, mint);
        let (vault_pda, _) = find_component_vault_address(&program_id, &index_account, mint);
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, mint, &spl_token::ID);

        accounts.push(AccountMeta::new(mint.clone(), false));
        accounts.push(AccountMeta::new(component_pda, false));
        accounts.push(AccountMeta::new_readonly(vault_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
    }

    let instruction = ProtocolInstruction::AddIndexComponents { amounts, mints };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// redeem
pub fn mint_instruction(
    caller: Pubkey,
    program_id: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    token_account: Pubkey,
    token_program_account: Pubkey,
    index_id: u64,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new_readonly(token_program_account, false),
    ];
    let instruction = ProtocolInstruction::Mint { index_id, amount };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn mint_instruction_with_dynamic_accounts(
    caller: Pubkey,
    program_id: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    token_account: Pubkey,
    token_program_account: Pubkey,
    mints: Vec<Pubkey>,
    token_accounts: Vec<Pubkey>,
    index_id: u64,
    amount: u64,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new_readonly(token_program_account, false),
    ];
    let instruction = ProtocolInstruction::Mint { index_id, amount };
    let data = borsh::to_vec(&instruction).unwrap();

    for (index, _mint) in mints.iter().enumerate() {
        let (component_pda, _) = find_component_address(&program_id, &index_account, _mint);
        let (vault_pda, _) = find_component_vault_address(&program_id, &index_account, _mint);
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, _mint, &spl_token::ID);

        accounts.push(AccountMeta::new_readonly(_mint.clone(), false));
        accounts.push(AccountMeta::new_readonly(component_pda, false));
        accounts.push(AccountMeta::new_readonly(vault_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
        let _token_account = token_accounts.get(index).unwrap();
        accounts.push(AccountMeta::new(*_token_account, false));
    }

    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// redeem
pub fn redeem_instruction(
    caller: Pubkey,
    program_id: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    token_account: Pubkey,
    token_program_account: Pubkey,
    index_id: u64,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new_readonly(token_program_account, false),
    ];
    let instruction = ProtocolInstruction::Redeem { index_id, amount };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn redeem_instruction_with_dynamic_accounts(
    caller: Pubkey,
    program_id: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    token_account: Pubkey,
    token_program_account: Pubkey,
    mints: Vec<Pubkey>,
    token_accounts: Vec<Pubkey>,
    index_id: u64,
    amount: u64,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new_readonly(caller, true),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new_readonly(token_program_account, false),
    ];
    let instruction = ProtocolInstruction::Redeem { index_id, amount };
    let data = borsh::to_vec(&instruction).unwrap();

    for (index, _mint) in mints.iter().enumerate() {
        let (component_pda, _) = find_component_address(&program_id, &index_account, _mint);
        let (vault_pda, _) = find_component_vault_address(&program_id, &index_account, _mint);
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, _mint, &spl_token::ID);

        accounts.push(AccountMeta::new_readonly(_mint.clone(), false));
        accounts.push(AccountMeta::new_readonly(component_pda, false));
        accounts.push(AccountMeta::new_readonly(vault_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
        let _token_account = token_accounts.get(index).unwrap();
        accounts.push(AccountMeta::new(*_token_account, false));
    }

    Instruction {
        program_id,
        accounts,
        data,
    }
}
