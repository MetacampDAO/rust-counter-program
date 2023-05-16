// state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{Sealed},
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}


// Implement function to get the data size constant
impl GreetingAccount {
    pub const SIZE: usize = 4;
}

