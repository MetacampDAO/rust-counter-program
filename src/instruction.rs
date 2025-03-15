// instruction.rs
use solana_program::{ program_error::ProgramError };
use borsh::{BorshDeserialize};

pub enum ProgramInstruction {
    InitializeCounter,
    IncreaseCounter {
        increase_by: u64
    },
    Delegate,
    CommitAndUndelegate,
}

#[derive(BorshDeserialize)]
struct IncreaseCounterPayload {
    increase_by: u64,
}

impl ProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, _rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => Self::InitializeCounter,
            1 => {
                let payload = IncreaseCounterPayload::try_from_slice(_rest).unwrap();
                Self::IncreaseCounter {
                    increase_by: payload.increase_by
                }
            },
            2 => Self::Delegate,
            3 => Self::CommitAndUndelegate,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}