Set up your development environment (30 min)
-> Guide: https://www.notion.so/metacamp-community/Introduction-Bootcamp-and-Solana-93ae1a235a964ecf9344dd06abdc90e5?pvs=4#d204cc6f46194ab1b8483c05bfcf14af

If you have cloned this repository, you can build the program as is, on your terminal run the below code. NOTE: After each time you build your Solana program, the below command will output the build path of your compiled program's .so file and the default keyfile that will be used for the program's address.
```
cd PATH/TO/THE/RUST-COUNTER-PROGRAM
cargo build-bpf
```
Using the Solana CLI, you can deploy your program to your currently selected cluster:
solana program deploy ./target/deploy/hello_world.so

To build the program from **scratch** initiate Rust program: rust_counter_program
```

// Initialize basic Rust backbone
cargo init rust_counter_program --lib
cd rust_counter_program

```

Add solana_program and borsh crates
```
// For building solana_program
cargo add solana-program

// For serialization and deserialization
cargo add borsh

```

Open Cargo.toml and add configurations settings
```
[lib]
name = "rust_counter_program"
crate-type = ["cdylib", "lib"]
```

Create/use lib.rs in src folder and import solana_program modules and borsh modules at the top.

```
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
    sysvar::Sysvar,
};
```

Each Solana program starts with an entrypoint and process_instruction
```

// declare and export the program's entrypoint. This is where every Solana program starts and execute its first function.
entrypoint!(process_instruction);

// program entrypoint's implementation.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Welcome to the Counter Program! This program counts how many times you greeted the program!");

    // gracefully exit the program
    Ok(())
}

```

Compile your Rust code into byte code, and deploy to the Solana blockchain
```

// Compile Rust program
cargo build-sbf

// Deploy byte code (require SOL for tx and deposit fee)
solana program deploy ./target/deploy/rust_counter_program.so

// Find program keypair 
cat ./target/deploy/rust_counter_program-keypair.json

```


Expand program with additional files and import them with "pub mod ${file_name}" in lib.rs:
- **instruction.rs -** serialize and deserialize instruction data
- **state.rs -** serialize and deserialize state
