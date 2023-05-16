// instruction.rs
use solana_program::{ program_error::ProgramError };

// Define instructions
pub enum GreetInstruction {
    Initialize,
    Greet,
}

// Implement byte unpacking function for instruction
impl GreetInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, _rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => Self::Initialize,
            1 => Self::Greet,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}