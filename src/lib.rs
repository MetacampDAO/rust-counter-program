use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    borsh::try_from_slice_unchecked,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
    sysvar::Sysvar,
};

pub mod instruction;
use instruction::GreetInstruction;

pub mod state;
use state::GreetingAccount;


// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Welcome to the Counter Program! This program counts how many times you greeted the program!");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let user_account = next_account_info(accounts_iter)?;
    let greeting_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (greeting_pda, bump_seed) = Pubkey::find_program_address(
        &[b"greeting_account", user_account.key.as_ref()],
        program_id,
    );

    // Check to ensure that you're using the right PDA
    if greeting_pda != *greeting_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let new_greeting_account = greeting_account.clone();
    let borrowed_lamports = new_greeting_account.try_borrow_lamports().unwrap();
    

    if *borrowed_lamports == &mut 0 {
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(GreetingAccount::SIZE);
        msg!("Creating state account1 at {} with {} lamports", greeting_pda, rent_lamports);
        drop(borrowed_lamports);
        invoke_signed(
            &system_instruction::create_account(
                user_account.key,
                greeting_account.key,
                rent_lamports,
                GreetingAccount::SIZE.try_into().unwrap(),
                program_id,
            ),
            &[
                user_account.clone(),
                greeting_account.clone(),
                system_program.clone(),
            ],
            &[&[b"greeting_account", user_account.key.as_ref(), &[bump_seed]]],
        )?;
        msg!("State account created.");
    };

    // The account must be owned by the program in order to modify its data
    if greeting_account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // msg!("borrowed_lamports: {}", *borrowed_lamports);

    // Increment and store the number of times the account has been greeted
    let mut greeting_data = GreetingAccount::try_from_slice(&greeting_account.data.borrow())?;
    msg!("Setting counter");
    greeting_data.counter += 1;
    msg!("greeting_data: {}", greeting_data.counter);
    greeting_data.serialize(&mut &mut greeting_account.data.borrow_mut()[..])?;

    msg!(
        "Greeted {}, {} time(s)!",
        greeting_account.key,
        greeting_data.counter
    );

    // gracefully exit the program
    Ok(())
}


